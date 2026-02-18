#![no_std]
#![no_main]

use cortex_m_rt::entry ;  
use stm32l4::stm32l4x6 ;
use panic_halt as _ ; 

use cortex_m::interrupt::Mutex ;
use core::cell::RefCell ;


use stm32l4::stm32l4x6::interrupt;
use core::cell::Cell;

static GLOBAL_PER : Mutex<RefCell<Option<stm32l4x6::Peripherals>>>=Mutex::new(RefCell::new(None));
static VALUE: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));
static STATE: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));
use heapless::String;
use core::fmt::Write;
// Fonction helper pour envoyer une string entière
fn usart_send_str(usart: &stm32l4x6::USART3, s: &str) {
    for byte in s.bytes() {
        // Attendre que le registre soit libre
        while usart.isr.read().txe().bit_is_clear() {}
        // Envoyer le byte
        usart.tdr.write(|w| unsafe { w.bits(byte as u32) });
    }
    // Attendre que la transmission soit terminée
    while usart.isr.read().tc().bit_is_clear() {}
    //  tc est à O quand la transmission est en cours
    // tc passe à 1 quand la transmission est terminé 
    // le bit tc est remis à 0 lors de la prochaine transmission comme mentionné dans le datasheet
    
}

#[interrupt]
fn USART3(){

 cortex_m::interrupt::free(|cs| {
        
        let mut per= GLOBAL_PER.borrow(cs).borrow_mut();
        // Check if PA13 caused the interrupt
        if per.as_mut().unwrap().USART3.isr.read().rxne().bit() {
            let mut val = per.as_mut().unwrap().USART3.rdr.read().bits() ;
                val = val & 0xFF  -('\0' as u32) ;
                VALUE.borrow(cs).set(val);  
                STATE.borrow(cs).set(1) ;
        }
    });


}
#[entry]
fn main()-> ! {
    //faire un programme usart, où l'usart communique avec le PC 
    let per_handler= stm32l4x6::Peripherals::take().unwrap() ;
    // Configuration USART3
    //activation de l'horloge de l'usart3 
    per_handler.RCC.apb1enr1.modify(|_, w| w.usart3en().set_bit());
    per_handler.RCC.ahb2enr.modify(|_, w| w.gpioben().set_bit());
    // per_handler.RCC.apb1enr1.write(|w| w.usart3en().set_bit());
    // configuration des broches de l'usart3 
    // broches PB10 (TX) et PB11 (RX)
    //activation de l'horloge GPIOB
    //per_handler.RCC.ahb2enr.write(|w| w.gpioben().set_bit());
    // configuration des broches en tant que alternate function USART3
    //mode des broches
    per_handler.GPIOB.moder.modify(|_, w| 
    w.moder10().bits(0b10)
     .moder11().bits(0b10)
    );
    // selection du mode alternate pour les ports choisit
    per_handler.GPIOB.afrh.modify(|_, w|
    w.afrh10().bits(0b0111)
     .afrh11().bits(0b0111)
    );

    //configuration usart3
    //configuration du baud rate
    //per_handler.USART3.brr.write(|w| unsafe{w.bits(4000000/19200)}) ;
    per_handler.USART3.brr.write(|w| unsafe { w.bits(4_000_000 / 115_200) });
    per_handler.USART3.cr1.modify(|_, w| {
    //w.ue().set_bit()  // active l'USART
     w.te().set_bit()  // active TE
     .re().set_bit()  // activation de la reception RE
     .rxneie().set_bit() // activation de l'interruption de reception RXNEIE
    });
    
    // configuration NVIC pour l'interruption USART3
    unsafe{ cortex_m::peripheral::NVIC::unmask(stm32l4x6::interrupt::USART3)}
    // Definition du niveau de priorité
     let mut core_p = cortex_m::peripheral::Peripherals::take().unwrap();
    unsafe {
        core_p.NVIC.set_priority(stm32l4x6::interrupt::USART3,1);
    }
    //Activation de l'usart 
    per_handler.USART3.cr1.modify(|_, w| {
    w.ue().set_bit()  // active l'USART
    });
    // Transfer de la variable peripherique dans le context gloable
     cortex_m::interrupt::free(|cs| {
        GLOBAL_PER.borrow(cs).replace(Some(per_handler));
    }); 

    loop { 

       cortex_m::interrupt::free(|cs| {
        
         let state=STATE.borrow(cs) ;
         if state.get() == 1 {
              let mut message: String<64> = String::new();
              write!(message, "THE VALUE YOU SENT IS : {}", VALUE.borrow(cs).get()).ok();
              
             state.set(0) ; 
         }
         
        });
        
   }
  
}