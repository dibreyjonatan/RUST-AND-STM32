#![no_std]
#![no_main]
// pas d'utilisation de std et de std main 
use cortex_m_rt::entry ;  //utilisation de la fonction entry qui definit le debut d'execution du code 
use stm32l4::stm32l4x6 ;
use panic_halt as _ ; 
#[entry]
fn main() -> !{
    // le but de ce code c'est de lire l'état du bouton PC13 et d'allumer la led PA5
    // on prend la variable qui permet de gérer les péripheriques de la carte 
    let per_handler= stm32l4x6::Peripherals::take().unwrap() ;

    // configuration led 
    //per_handler.RCC.ahb2enr.write(|w| w.gpioaen().set_bit()); 
    //activation de l'horloge de GPIOC bit O 
    // activation de l'horloge de GPIOC via bit 2
    per_handler.RCC.ahb2enr.modify(|_, w| w.gpioaen().set_bit().gpiocen().set_bit());
    per_handler.GPIOA.moder.write(|w| w.moder5().output());


    //configuration bouton 
    per_handler.GPIOC.moder.write(|w| w.moder13().input()) ;
    per_handler.GPIOC.pupdr.write(|w| unsafe{w.pupdr13().bits(0b01)});
    //boucle 
    //definition d'une variable d'état 
    let mut _state : i32 =0 ;
    loop {
       /*
       // Fonctionnement souhaité 
        On veut cliquer sur le bouton et  la led s'allume, puis cliquer encore sur le bouton et la led s'éteint.
        B1 appuyé --- > ON 
        ensuite au second B1 appuyé --- > OFF 
       */   
       // ON a mis PC13 en pull up, donc quand le bouton n'est pas appuyer, il envoie 5V
       // et passe à 0 quand le bouton est pressé. 
       
       // bouton non pressé 
       // PC13 envoie "1"
       if per_handler.GPIOC.idr.read().idr13().bit()  {  
         if _state %2 == 0 {

         per_handler.GPIOA.bsrr.write(|w| unsafe { w.bits(1 << 21) }); //mise à 0 
         }
        // _state=0 ;
       }
       // bouton pressé 
       // PC13 envoie 0 
       else{ 
        if !per_handler.GPIOC.idr.read().idr13().bit() {
           _state+=1;
         if _state % 2 == 1 {
          per_handler.GPIOA.bsrr.write(|w| w.bs5().set_bit()) ;  // mise à 1 
       }
    }
    }


    }

     
}
