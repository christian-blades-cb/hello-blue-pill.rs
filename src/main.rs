#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

// extern crate cortex_m;
extern crate stm32f1xx_hal as hal;

use hal::stm32;
use cortex_m_rt::{entry, exception, ExceptionFrame};
use hal::prelude::*;
use hal::i2c::{I2c, blocking_i2c, DutyCycle};
use embedded_graphics::image::Image1BPP;
use embedded_graphics::prelude::*;
use embedded_graphics::fonts::Font6x8;
use ssd1306::prelude::*;
use core::panic::PanicInfo;

#[entry]
fn main() -> ! {
    // let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    
    // let mut afio = dp.AFIO.constrain(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    let scl = gpiob.pb10.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb11.into_alternate_open_drain(&mut gpiob.crh);

    let i2c = I2c::i2c2(
        dp.I2C2,
        (scl, sda),
        hal::i2c::Mode::Fast{
            frequency: 400_000,
            duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
        &mut rcc.apb1);

    let b_i2c = blocking_i2c(i2c, clocks, 1000, 3, 1000, 1000);
    
    let mut disp: GraphicsMode<_> = ssd1306::Builder::new().with_size(DisplaySize::Display128x64).connect_i2c(b_i2c).into();
    disp.init().unwrap();
    disp.flush().unwrap();
    
    let im = Image1BPP::new(include_bytes!("./rust.raw"), 64, 64).translate(Coord::new(56, 0));

    disp.draw(im.into_iter());
    disp.draw(Font6x8::render_str("Hello world!").into_iter());
    disp.flush().unwrap();

    loop {}
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
