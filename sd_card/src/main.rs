use embedded_sdmmc::*;
use esp_idf_hal::{
    gpio::*,
    peripherals::Peripherals,
    prelude::*,
    spi::{config::Duplex, *},
    delay::FreeRtos,
};
use embedded_hal::delay::DelayNs;
use esp_println::println;

pub struct SdMmcClock;

impl TimeSource for SdMmcClock {
    fn get_timestamp(&self) -> Timestamp {
        Timestamp { year_since_1970: 0, zero_indexed_month: 0, zero_indexed_day: 0, hours: 0, minutes: 0, seconds: 0 }
    }
}

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("SPI SD card demo!");

    let peripherals = Peripherals::take().unwrap();

    let driver = SpiDriver::new(
        peripherals.spi2,
        peripherals.pins.gpio39,
        peripherals.pins.gpio40,
        Some(peripherals.pins.gpio41),
        &config::DriverConfig::default(),
    ).unwrap();

    let mut spi_config = SpiConfig::new();
    spi_config.duplex = Duplex::Full;
    let _ = spi_config.baudrate(24.MHz().into());
    let spi = SpiDeviceDriver::new(driver, Option::<Gpio38>::None, &spi_config).unwrap();

    let cs = PinDriver::output(peripherals.pins.gpio38).unwrap();
    let mut sdcard = embedded_sdmmc::SdCard::new(spi, <dyn DelayNs>::new(10));

    println!("Card size is {} bytes", sdcard.num_bytes());
}
