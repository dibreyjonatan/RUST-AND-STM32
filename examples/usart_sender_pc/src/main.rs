#![no_std]
#![no_main]

use cortex_m_rt::entry ;  
use stm32l4::stm32l4x6 ;
use panic_halt as _ ; 

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
    w.ue().set_bit()  // active l'USART
     .te().set_bit()  // active TE
    });
    
    loop { 

    let data = "KAMDA IS LEARNING EMBEDDED RUST\r\n";  // \r\n pour le retour à la ligne sur le terminal
    usart_send_str(&per_handler.USART3, data);  //pour utiliser le handler peripherique, on utilise la notion de reference 
                                                // cette technique permet de faire des fonctions en ce sens.

    for _ in 0..500_000 {
        cortex_m::asm::nop();
    }
   }
  
}