use dht11::Dht11;
use esp_idf_hal::{
    gpio::*,
    peripherals::Peripherals,
    delay::{Ets, FreeRtos},
};
use esp_println::println;


fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("DHT11 demo!");

    let peripherals = Peripherals::take().unwrap();

    let dht11_pin = PinDriver::input_output_od(peripherals.pins.gpio1.downgrade()).unwrap();
    let mut dht11 = Dht11::new(dht11_pin);

    loop {
        let mut dht11_delay = Ets;

        match dht11.perform_measurement(&mut dht11_delay)
        {
            Ok(measurement) => println!(
                "temp: {temp}C, humidity: {humid}%",
                temp = (measurement.temperature as f32 / 10.0),
                humid = (measurement.humidity as f32 / 10.0)
            ),
            Err(e) => println!("{:?}", e),
        }

        FreeRtos::delay_ms(2000);
    }
}
