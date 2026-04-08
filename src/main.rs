#![no_std]
#![no_main]

use embedded_hal::digital::{InputPin, OutputPin};
use panic_halt as _;
use vcc_gnd_yd_rp2040::entry;
use vcc_gnd_yd_rp2040::{
    hal::{
        clocks::{init_clocks_and_plls, Clock},
        pac, 
        watchdog::Watchdog,
        Sio,
    },
    Pins, XOSC_CRYSTAL_FREQ,
};

#[entry] // начало работы прошивки с этого момента 
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap(); // Видимо тут мы подключаемся к переферии то есть к vcc_gnd_yd_rp2040 пинам чтобы не подключатся вручную
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    
    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ, // Исходная частота кварца, от которой всё начинается
        pac.XOSC, // Модуль кварца	Включает внешний генератор и усиливает герцовку подключённых модулей чтобы они работали быстрее
        pac.CLOCKS,
        pac.PLL_SYS, // Умножает частоту для системной шины (обычно до 125 MHz)
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )

    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led_pin = pins.led.into_push_pull_output(); led_pin.set_low().unwrap(); // Светодиод на GPIO25 сразу выключен.

    let mut usr_btn = pins.user_key.into_pull_up_input(); // Кнопка управления светодиодом.

    let mut prev_state = false;
    let mut _mode = 0;

    loop {
        let curr_state = usr_btn.is_low().unwrap();

        if curr_state == true && prev_state == false {
            if _mode < 2 {
                _mode += 1;
            } else if _mode == 2 {
                _mode = 0;
            }
        } 

        prev_state = curr_state;

        if _mode == 0 { // Погашенный светодиод.
            led_pin.set_low().unwrap();
        } else if _mode == 1 { // Светодиод светится постоянно.
            led_pin.set_high().unwrap();
        } else if _mode == 2 { // Cветодиод мерцает.
            led_pin.set_high().unwrap();
            delay.delay_ms(50);
            led_pin.set_low().unwrap();
            delay.delay_ms(50);
        }
    }
}