use core::convert::TryInto;
use embedded_hal::blocking::i2c::{Write, WriteRead};
use lsm303dlhc::{I16x3, Lsm303dlhc};
use stm32f3xx_hal::{
    gpio, i2c,
    i2c::{I2c, SclPin, SdaPin},
    prelude::*,
    rcc::Clocks,
};

// For this specific board, these are the pins and alternate modes corresponding to the compass peripheral.
// From user manual:
// https://www.st.com/content/ccc/resource/technical/document/user_manual/8a/56/97/63/8d/56/41/73/DM00063382.pdf/files/DM00063382.pdf/jcr:content/translations/en.DM00063382.pdf
// In the pin description, column LSM303DLHC cells SCL and SCA.
type CompassScl = gpio::Pin<gpio::Gpiob, gpio::U<6>, gpio::Alternate<gpio::OpenDrain, 4>>;
type CompassSda = gpio::Pin<gpio::Gpiob, gpio::U<7>, gpio::Alternate<gpio::OpenDrain, 4>>;
type CompassI2C<I2CPeripheral> = I2c<I2CPeripheral, (CompassScl, CompassSda)>;

// We can be given one of a number of I2C peripherals, so allow for any.
pub struct Compass<I2CPeripheral> {
    compass: Lsm303dlhc<CompassI2C<I2CPeripheral>>,
}
impl<I2CPeripheral, Bus, Error> Compass<I2CPeripheral>
where
    I2CPeripheral: i2c::Instance<Bus = Bus>,
    CompassScl: SclPin<I2CPeripheral>,
    CompassSda: SdaPin<I2CPeripheral>,
    CompassI2C<I2CPeripheral>: WriteRead<Error = Error> + Write<Error = Error>,
{
    pub fn new(
        i2c_channel: I2CPeripheral,
        mut gpiob: gpio::gpiob::Parts,
        clocks: Clocks,
        i2c_bus: &mut Bus,
    ) -> Result<Self, Error>
where {
        // afrl is "alternate function register low" for selecting alternate function for pins 0-7.
        let scl = gpiob
            .pb6
            .into_af_open_drain::<4>(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);
        let sda = gpiob
            .pb7
            .into_af_open_drain::<4>(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);

        let i2c_initialised = i2c::I2c::new(
            i2c_channel,
            (scl, sda),
            100_u32.kHz().try_into().unwrap(),
            clocks,
            // Section 3.2.2 in reference manual documents i2c being available over apb1.
            i2c_bus,
        );
        Lsm303dlhc::new(i2c_initialised).map(|compass_device| Compass {
            compass: compass_device,
        })
    }

    pub fn get_compass_reading(&mut self) -> Result<I16x3, Error> {
        self.compass.mag()
    }
}
