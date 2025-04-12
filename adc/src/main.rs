use esp_idf_hal::{
    adc,
    delay::FreeRtos,
    peripherals::Peripherals,
};
use esp_println::println;


fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("ADC demo");

    let peripherals = Peripherals::take().unwrap();

    let adc1 = adc::oneshot::AdcDriver::new(peripherals.adc1).unwrap();
    let adc1_config = adc::oneshot::config::AdcChannelConfig{
                        attenuation: adc::attenuation::DB_11,
                        ..Default::default()};

    let mut a1_ch0 = adc::oneshot::AdcChannelDriver::new(&adc1, peripherals.pins.gpio1, &adc1_config).unwrap();

    loop {
        match adc1.read(&mut a1_ch0)
        {
            Ok(x) => println!("A1_CH0: {x}\n"),
            Err(e) => println!("error reading ADC: {e}\n"),
        }

        FreeRtos::delay_ms(1000);
    }
}
