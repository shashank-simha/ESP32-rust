use crossbeam_channel::bounded;
use esp_idf_hal::{
    delay::FreeRtos,
    gpio::{AnyIOPin, AnyOutputPin, IOPin, Input, Output, OutputPin, PinDriver, Pull},
    peripherals::Peripherals,
};
use esp_println::println;


static BLINKY_STACK_SIZE: usize = 2000;
static BUTTON_STACK_SIZE: usize = 2000;


fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Threads Demo!");

    let _peripherals = Peripherals::take().unwrap();
    let led_pin = PinDriver::output(_peripherals.pins.gpio2.downgrade_output()).unwrap();
    let mut btn_pin = PinDriver::input(_peripherals.pins.gpio1.downgrade()).unwrap();
    btn_pin.set_pull(Pull::Down).unwrap();

    let (tx, rx) = bounded(1);

    let _blinky_thread = std::thread::Builder::new()
        .stack_size(BLINKY_STACK_SIZE)
        .spawn(move || blinky_thread_function(led_pin, rx));

    let _blinky_thread = std::thread::Builder::new()
        .stack_size(BUTTON_STACK_SIZE)
        .spawn(move || button_thread_function(btn_pin, tx));
}

fn blinky_thread_function(mut led_pin: PinDriver<AnyOutputPin, Output>, rx: crossbeam_channel::Receiver<bool>)
{
    let mut blinky_status = false;

    loop {
        match rx.try_recv()
        {
            Ok(x) => blinky_status = x,
            Err(_) => {},
        }

        if blinky_status
        {
            led_pin.set_high().unwrap();
            println!("LED ON");
            FreeRtos::delay_ms(1000);

            led_pin.set_low().unwrap();
            println!("LED OFF");
        }
        FreeRtos::delay_ms(1000);
    }
}

fn button_thread_function(btn_pin: PinDriver<AnyIOPin, Input>, tx: crossbeam_channel::Sender<bool>)
{
    let mut btn_status = false;

    loop {
        if btn_pin.is_high()
        {
            if !btn_status
            {
                btn_status = true;
                println!("BUTTON ON");
                tx.send(btn_status).unwrap();
            }
        }
        else
        {
            if btn_status
            {
                btn_status = false;
                println!("BUTTON OFF");
                tx.send(btn_status).unwrap();
            }
        }
        FreeRtos::delay_ms(100);
    }
}
