use esp_idf_hal::{
    gpio::PinDriver,
    peripherals::Peripherals,
    delay::FreeRtos,
};
use esp_println::println;
fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let _peripherals = Peripherals::take().unwrap();
    let mut led = PinDriver::output(_peripherals.pins.gpio2).unwrap();

    println!("Blinky example std");
    loop {
        led.set_high().unwrap();
        println!("LED ON");
        FreeRtos::delay_ms(1000);

        led.set_low().unwrap();
        println!("LED OFF");
        FreeRtos::delay_ms(1000);
    }
}

