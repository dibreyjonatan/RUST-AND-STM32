#! [no_std]
#! [no_main]

use cortex_m_rt::entry ;  //utilisation de la fonction entry qui definit le debut d'execution du code 
use stm32l4::stm32l4x6 ;
use panic_halt as _ ; 
#[entry]

fn main() ->! {
    //objectif faire clignoter la led PA5 toutes les 1s en utilisant le timer2 
    // frequence interne 4MHz
    let per_handler= stm32l4x6::Peripherals::take().unwrap() ;
    //configuration de la led PA5 
    per_handler.RCC.ahb2enr.write(|w| w.gpioaen().set_bit());
    per_handler.GPIOA.moder.write(|w| w.moder5().output());
    //configuration du timer2 
    //activation de l'horloge 
    per_handler.RCC.apb1enr1.write(|w| w.tim2en().set_bit());
    //mise à 0 du compteur
    per_handler.TIM2.cnt.write(|w| {w.bits(0)}) ;
    //pour avoir une seconde  T=(1+PSC)(1+ARR)/f  f=4MHz
    // en fixant psc=127, on en déduit ARR
    per_handler.TIM2.psc.write(|w| unsafe{w.bits(127)});
    per_handler.TIM2.arr.write(|w| w.bits(31249)) ;
    per_handler.TIM2.cr1.write(|w| w.cen().set_bit());
    
    let mut _state : i32 =0 ;
    loop{
      
        if per_handler.TIM2.sr.read().uif().bit() {
            _state+=1 ;
            
            per_handler.TIM2.sr.modify(|_, w| w.uif().clear_bit());
            
        }

        if _state%2 == 0 {
           per_handler.GPIOA.bsrr.write(|w| w.bs5().set_bit()) ;
        }
        else{
           per_handler.GPIOA.bsrr.write(|w| unsafe { w.bits(1 << 21) });
        }
    }
}
