use esp_idf_svc::hal::{
    i2c::{I2cConfig, I2cDriver},
    prelude::Peripherals,
    units::KiloHertz,
};
use rda5807m::{Address, Rda5708m};
use std::{thread::sleep, time::Duration};

fn main() {
    esp_idf_svc::log::EspLogger::initialize_default();

    let mut config = I2cConfig::new().baudrate(KiloHertz(100).into());

    // without setting a timeout value, the tuner would occasionally timeout
    config.timeout = Some(Duration::from_millis(10).into());

    let peripherals = Peripherals::take().unwrap();
    let i2c_driver = I2cDriver::new(
        peripherals.i2c0,
        peripherals.pins.gpio16,
        peripherals.pins.gpio17,
        &config,
    )
    .unwrap();

    let mut rda5807m = Rda5708m::new(i2c_driver, Address::default());
    rda5807m.start().unwrap();
    sleep(Duration::from_millis(100));
    // set volume max 15
    rda5807m.set_volume(1).unwrap();
    // get freq
    let freq = rda5807m.get_frequency().unwrap();
    // set freq
    rda5807m.set_frequency(101500).unwrap();
    // seek up
    rda5807m.seek_up(true).unwrap();
    log::info!("Hello, world! {freq}");
}
