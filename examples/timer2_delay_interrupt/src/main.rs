#! [no_std]
#! [no_main]

use cortex_m::interrupt::Mutex ;
use core::cell::RefCell ;
use cortex_m_rt::entry ;  //utilisation de la fonction entry qui definit le debut d'execution du code 
use stm32l4::stm32l4x6 ;
use panic_halt as _ ; 


use stm32l4::stm32l4x6::interrupt;
use core::cell::Cell;

// creation de 2 variables globales
 // Variable du timer 
 static TIM2_PER : Mutex<RefCell<Option<stm32l4x6::Peripherals>>>=Mutex::new(RefCell::new(None));
 // Variable du countdown 
 static COUNTER: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));
  
  fn delay_ms(count : u32){
     if count !=0 { //travaillé si t > 0 
     // activer le timer2 dans le delai 
    cortex_m::interrupt::free(|cs| {
          let mut interrupt_peripheral=TIM2_PER.borrow(cs).borrow_mut();
        interrupt_peripheral.as_mut().unwrap().TIM2.cr1.modify(|_,w| w.cen().set_bit());
    });
    //je mets la valeur à count
      cortex_m::interrupt::free(|cs|
                COUNTER.borrow(cs).set(count));
   
    while  cortex_m::interrupt::free(|cs| COUNTER.borrow(cs).get() ) > 0 {
         cortex_m::asm::wfi();
    }
   //desactiver le timer2 à la sortie
    cortex_m::interrupt::free(|cs| {
          let mut interrupt_peripheral=TIM2_PER.borrow(cs).borrow_mut();
        interrupt_peripheral.as_mut().unwrap().TIM2.cr1.modify(|_,w| w.cen().clear_bit());
    }) ;
}
}
#[interrupt]
fn TIM2(){
     
    cortex_m::interrupt::free(|cs| {
        
        //on decremente la valeur du compteur 
        let current = COUNTER.borrow(cs).get();
        if current > 0 {
        COUNTER.borrow(cs).set(current - 1);
        }
        // l'état du bit est mise à 1 par le hardware, on le met à zero par software
        let mut interrupt_peripheral=TIM2_PER.borrow(cs).borrow_mut();
        interrupt_peripheral.as_mut().unwrap().TIM2.sr.modify(|_,w| w.uif().clear_bit());
    });
}
#[entry]

fn main() ->! {
    //objectif faire clignoter la led PA5 toutes les xs en utilisant le timer2
    //  et une fonction delay dont on procédéra par interruption
    // frequence interne 4MHz
    let per_handler= stm32l4x6::Peripherals::take().unwrap() ;
    //configuration de la led PA5 
   
      //activation de l'horloge 
    per_handler.RCC.ahb2enr.modify(|_, w| w.gpioaen().set_bit());
    per_handler.GPIOA.moder.write(|w| w.moder5().output());

    //configuration du timer2 
    per_handler.RCC.apb1enr1.modify(|_, w| w.tim2en().set_bit());
    //mise à 0 du compteur
    per_handler.TIM2.cnt.write(|w| {w.bits(0)}) ;
    //pour avoir une seconde  T=(1+PSC)(1+ARR)/f  f=4MHz
    // en fixant psc=127, on en déduit ARR
    // on va faire une interruption de 1ms, on a ARR ~ 31,25 donc 32
    per_handler.TIM2.psc.write(|w| unsafe{w.bits(127)});
    per_handler.TIM2.arr.write(|w| w.bits(32)) ;
   
    // Configuration de l'interruption TIM2
    //ACtivation de l'interruption 
    per_handler.TIM2.dier.modify(|_,w| w.uie().set_bit());
    // Enregistrement du Timer2 au niveau du NVIC
    unsafe{ cortex_m::peripheral::NVIC::unmask(stm32l4x6::interrupt::TIM2)}
    // On donne au tim2 une priorité de 2
    // Pour cela il faut acceder aux coeur du processeur et y modifier sa priorité
    let mut core_p = cortex_m::peripheral::Peripherals::take().unwrap();
    unsafe {
        core_p.NVIC.set_priority(stm32l4x6::interrupt::TIM2, 2);
    }
    //cette ligne a été commenter car on ne veut activer le timer uniquement dans le delai 
    //Activation du timer2
     //per_handler.TIM2.cr1.write(|w| w.cen().set_bit());
    
     // Deplacement de la variable timer2 dans le context globale
     cortex_m::interrupt::free(|cs| {
        TIM2_PER.borrow(cs).replace(Some(per_handler));
    }); 
    loop {
        
        // on n'accède aux peripherique que en utilisant RfCell
        cortex_m::interrupt::free(|cs| {
        
        let mut per=TIM2_PER.borrow(cs).borrow_mut();
        per.as_mut().unwrap().GPIOA.bsrr.write(|w| w.bs5().set_bit()) ;
        });


           delay_ms(1000); 
        
          
        cortex_m::interrupt::free(|cs| {
        
        let mut per=TIM2_PER.borrow(cs).borrow_mut();
        per.as_mut().unwrap().GPIOA.bsrr.write(|w| unsafe { w.bits(1 << 21) });
        });
         delay_ms(1000); 

    }

}