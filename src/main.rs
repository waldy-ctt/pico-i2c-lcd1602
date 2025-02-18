#![no_std]
#![no_main]

use embedded_hal::digital::OutputPin;
use panic_halt as _;
use rtt_target::{rprint, rtt_init_print};
use rp2040_hal::{self, pac::{self}, Clock};

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

    rtt_init_print!();
    rprint!("Hello Mate!");
    let mut i_led = pins.gpio25.into_push_pull_output();
    loop{
        i_led.set_high().unwrap();

    }
}
