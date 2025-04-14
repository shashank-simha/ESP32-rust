use esp_idf_hal::{
    uart::*,
    gpio,
    prelude::*,
    peripherals::Peripherals,
    delay::{FreeRtos, NON_BLOCK},
};
use esp_println::println;


const CR: u8 = 13;


fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("UART Demo!");

    let peripherals = Peripherals::take().unwrap();
    let tx = peripherals.pins.gpio5;
    let rx = peripherals.pins.gpio4;

    let config = config::Config::new().baudrate(Hertz(115_200));
    let uart = UartDriver::new(peripherals.uart1, tx, rx, Option::<gpio::Gpio0>::None, Option::<gpio::Gpio1>::None, &config).unwrap();

    let mut cli_buf: Vec<u8> = Vec::new();

    loop
    {
        let mut buf:[u8; 10] = [0; 10];
        match uart.read(&mut buf, NON_BLOCK)
        {
            Ok(num_bytes) => {
                if num_bytes > 0
                {
                    let b = buf[0];
                    cli_buf.push(b);
                    if b == CR
                    {
                        match uart.write(&cli_buf)
                        {
                            Ok(_) => println!("{:?}\n", cli_buf),
                            Err(_) => {}
                        }
                        cli_buf.clear();
                    }
                }
            }
            Err(_) => {}
        }
        FreeRtos::delay_ms(100);
    }
}
