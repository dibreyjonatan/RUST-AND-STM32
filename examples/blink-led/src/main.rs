#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use stm32l4::stm32l4x6;

#[entry]
fn main() -> ! {
    let p = stm32l4x6::Peripherals::take().unwrap();
    
    // Enable GPIOA
    p.RCC.ahb2enr.write(|w| w.gpioaen().set_bit());
    
    // PA5 = output
    p.GPIOA.moder.write(|w| w.moder5().output());
    
    loop {
        // ON
        p.GPIOA.bsrr.write(|w| w.br5().set_bit());
        for _ in 0..500_000 { cortex_m::asm::nop(); }
        
        // OFF
        p.GPIOA.bsrr.write(|w| w.bs5().set_bit());
        for _ in 0..500_000 { cortex_m::asm::nop(); }
    }
}