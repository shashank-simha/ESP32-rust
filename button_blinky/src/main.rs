use esp_idf_hal::{
    delay::FreeRtos,
    peripherals::Peripherals,
    gpio::{IOPin, PinDriver, Pull},
};
use esp_println::println;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Button + Blinky");

    let _peripherals = Peripherals::take().unwrap();

    let mut led_pin = PinDriver::output(_peripherals.pins.gpio2).unwrap();

    let mut btn_pin = PinDriver::input(_peripherals.pins.gpio1.downgrade()).unwrap();
    btn_pin.set_pull(Pull::Down).unwrap();

    loop{
        if btn_pin.is_high()
        {
            led_pin.set_high().unwrap();
            println!("LED ON");
            FreeRtos::delay_ms(1000);

            led_pin.set_low().unwrap();
            println!("LED OFF");
        }
        else
        {
            // turn off the led
            led_pin.set_low().unwrap();
        }
        FreeRtos::delay_ms(1000);
    }
}
