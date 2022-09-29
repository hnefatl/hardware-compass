#![no_std]
#![no_main]
#![feature(exhaustive_patterns)]
#![feature(stmt_expr_attributes)]

//use panic_halt as _; // breakpoint on `rust_begin_unwind` to catch panics
use panic_semihosting as _;

use cortex_m::asm;
use cortex_m_rt::entry;
use embedded_hal::digital::v2::PinState;
use stm32f3xx_hal::{pac, prelude::*};

mod led_wheel;
use led_wheel::LEDWheel;

mod compass;
use compass::Compass;

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();

    let mut reset_and_clock_control = peripherals.RCC.constrain();
    let mut flash = peripherals.FLASH.constrain();
    let clocks = reset_and_clock_control.cfgr.freeze(&mut flash.acr);

    // For determining which bus (ahb) is needed, section 3.2.2 in
    // https://www.st.com/resource/en/reference_manual/dm00043574-stm32f303xb-c-d-e-stm32f303x6-8-stm32f328x8-stm32f358xc-stm32f398xe-advanced-arm-based-mcus-stmicroelectronics.pdf
    // documents which peripherals are reachable over which buses.
    let gpioe = peripherals.GPIOE.split(&mut reset_and_clock_control.ahb);
    let mut led_wheel = LEDWheel::new(gpioe);

    let gpiob = peripherals.GPIOB.split(&mut reset_and_clock_control.ahb);
    let mut compass = Compass::new(
        peripherals.I2C1,
        gpiob,
        clocks,
        // Section 3.2.2 in reference manual documents i2c being available over apb1.
        &mut reset_and_clock_control.apb1,
    )
    .unwrap();

    loop {
        let mag = compass.get_compass_reading().unwrap();
        if mag.x != 0 || mag.y != 0 || mag.z != 0 {
            asm::nop()
        }
        // z is down, x and y are towards and perpendicular to north pole bearing? not sure
        let Ok(_) = led_wheel
            .n
            .set_state(if mag.x > 0 { PinState::High } else { PinState::Low });
        let Ok(_) = led_wheel
            .s
            .set_state(if mag.x < 0 { PinState::High } else { PinState::Low });
    }
}
