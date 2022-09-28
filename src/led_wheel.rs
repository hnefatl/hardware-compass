use stm32f3xx_hal::gpio;

// Type alias to reduce boilerplate.
pub type LED = gpio::Pin<gpio::Gpioe, gpio::Ux, gpio::Output<gpio::PushPull>>;

pub struct LEDWheel {
    pub nw: LED,
    pub n: LED,
    pub ne: LED,
    pub e: LED,
    pub se: LED,
    pub s: LED,
    pub sw: LED,
    pub w: LED,
}
impl LEDWheel {
    pub const COUNT: usize = 8;

    pub fn new(mut gpioe: gpio::gpioe::Parts) -> LEDWheel {
        #[rustfmt::skip]
        LEDWheel {
            nw: gpioe.pe8.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper).downgrade(),
            n: gpioe.pe9.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper).downgrade(),
            ne: gpioe.pe10.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper).downgrade(),
            e: gpioe.pe11.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper).downgrade(),
            se: gpioe.pe12.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper).downgrade(),
            s: gpioe.pe13.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper).downgrade(),
            sw: gpioe.pe14.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper).downgrade(),
            w: gpioe.pe15.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper).downgrade(),
        }
    }

    pub fn by_index(&mut self, index: usize) -> &mut LED {
        let leds: [&mut LED; Self::COUNT] = [
            &mut self.n,
            &mut self.ne,
            &mut self.e,
            &mut self.se,
            &mut self.s,
            &mut self.sw,
            &mut self.w,
            &mut self.nw,
        ];
        leds[index % 8]
    }
}
