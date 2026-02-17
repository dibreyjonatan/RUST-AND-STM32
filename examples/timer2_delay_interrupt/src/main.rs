#! [no_std]
#! [no_main]

use cortex_m_rt::entry ;  //utilisation de la fonction entry qui definit le debut d'execution du code 
use stm32l4::stm32l4x6 ;
use panic_halt as _ ; 
#[entry]

fn main() ->! {
    //objectif faire clignoter la led PA5 toutes les xs en utilisant le timer2
    //  et une fonction delay dont on procédéra par interruption
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
    // on va faire une interruption de 1ms, on a ARR ~ 31,25 donc 32
    per_handler.TIM2.psc.write(|w| unsafe{w.bits(127)});
    per_handler.TIM2.arr.write(|w| w.bits(32)) ;
    per_handler.TIM2.cr1.write(|w| w.cen().set_bit());
    
    loop {

    }

}