#![no_std]
#![no_main]

use embedded_hal::{digital::OutputPin, i2c::I2c};
use lcd1602_diver::{self, LCD1602};
use panic_halt as _;
use rp2040_hal::{self, gpio::{FunctionI2C, Pin, PullUp}, pac::{self, resets::reset}, uart::{DataBits, StopBits, UartConfig, UartPeripheral}, usb::UsbBus, Clock, I2C};
use fugit::RateExtU32;
use core::fmt::Write;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

#[rp2040_hal::entry]
fn main() -> !{
    //Initial pac / peripherals
    let mut pac = pac::Peripherals::take().unwrap();
    let core  = pac::CorePeripherals::take().unwrap();

    let mut watchdog = rp2040_hal::Watchdog::new(pac.WATCHDOG);

    let clock = rp2040_hal::clocks::init_clocks_and_plls(12_000_000u32, pac.XOSC, pac.CLOCKS, pac.PLL_SYS, pac.PLL_USB, &mut pac.RESETS, &mut watchdog).ok().unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clock.system_clock.freq().to_Hz());

    let sio = rp2040_hal::Sio::new(pac.SIO);

    //Initial Pins / GPIO
    let pins= rp2040_hal::gpio::Pins::new(pac.IO_BANK0, pac.PADS_BANK0, sio.gpio_bank0, &mut pac.RESETS);

    let sda_pin: Pin<_, FunctionI2C, PullUp> = pins.gpio16.reconfigure(); //Brown
    let scl_pin: Pin<_, FunctionI2C, PullUp> = pins.gpio17.reconfigure(); //Red

    let mut i2c = I2C::i2c0(pac.I2C0, sda_pin, scl_pin, 400.kHz(), &mut pac.RESETS, clock.system_clock.freq());

    let mut i_led = pins.gpio25.into_push_pull_output();

    let uart_pins = (pins.gpio0.into_function(), pins.gpio1.into_function()); 

    let mut uart = rp2040_hal::uart::UartPeripheral::new(pac.UART0, uart_pins, &mut pac.RESETS)
    .enable(
        UartConfig::new(115200.Hz(), DataBits::Eight, None, StopBits::One),
        clock.peripheral_clock.freq(),
    ).unwrap();

    uart.write_full_blocking(b"Raspberry Pi Pico started and registered\r\n");

    
    // I2C address of the LCD
    let lcd_addr = 0x27u8;

    let mut lcd = LCD1602::new_i2c(i2c, lcd_addr, &mut delay).expect("FAILED LCD");

    let _ = lcd.reset(&mut delay);

    let _ = lcd.clear(&mut delay);

    
    loop {
        let _  = lcd.write_str("Le Thanh Hieu", &mut delay);
    
        delay.delay_ms(1000);
    
        let _ = lcd.clear(&mut delay);

        let _ = lcd.write_str("Yeu", &mut delay);

        delay.delay_ms(1000);
    
        let _ = lcd.clear(&mut delay);

        let _ = lcd.write_bytes(b"Nguyen Thi", &mut delay);
        let _ = lcd.set_cursor_pos(40, &mut delay);
        let _ = lcd.write_bytes(b"Minh Thu", &mut delay);
        let _ = lcd.clear(&mut delay);

        delay.delay_ms(10000);

    }
}
