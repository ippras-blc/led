use esp_idf_svc::{
    hal::{
        gpio::OutputPin,
        peripheral::Peripheral,
        rmt::{
            config::TransmitConfig, FixedLengthSignal, PinState, Pulse, RmtChannel, TxRmtDriver,
        },
    },
    sys::EspError,
};
use rgb::RGB8;
use std::time::Duration;

/// Led (WS2812)
pub struct Led<'a> {
    driver: TxRmtDriver<'a>,
}

impl<'a> Led<'a> {
    /// Onboard RGB LED pin
    ///
    /// * Rust ESP Board gpio2
    /// * ESP32-C3-DevKitC-01 gpio8
    /// * ESP32-C3-DevKitC-02 gpio8
    pub fn new(
        pin: impl Peripheral<P = impl OutputPin> + 'a,
        channel: impl Peripheral<P = impl RmtChannel> + 'a,
    ) -> Result<Self, EspError> {
        let config = TransmitConfig::new().clock_divider(2);
        let tx = TxRmtDriver::new(channel, pin, &config)?;
        Ok(Self { driver: tx })
    }

    pub fn set_color(&mut self, rgb: RGB8) -> Result<(), EspError> {
        let color: u32 = ((rgb.g as u32) << 16) | ((rgb.r as u32) << 8) | rgb.b as u32;
        let ticks_hz = self.driver.counter_clock()?;
        let t0h = Pulse::new_with_duration(ticks_hz, PinState::High, &Duration::from_nanos(350))?;
        let t0l = Pulse::new_with_duration(ticks_hz, PinState::Low, &Duration::from_nanos(800))?;
        let t1h = Pulse::new_with_duration(ticks_hz, PinState::High, &Duration::from_nanos(700))?;
        let t1l = Pulse::new_with_duration(ticks_hz, PinState::Low, &Duration::from_nanos(600))?;
        let mut signal = FixedLengthSignal::<24>::new();
        for i in (0..24).rev() {
            let p = 2_u32.pow(i);
            let bit = p & color != 0;
            let (high_pulse, low_pulse) = if bit { (t1h, t1l) } else { (t0h, t0l) };
            signal.set(23 - i as usize, &(high_pulse, low_pulse))?;
        }
        self.driver.start_blocking(&signal)?;
        Ok(())
    }
}
