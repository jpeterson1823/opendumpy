#![no_std]
#![no_main]

#![allow(unused)]

mod gbcart;
mod hardware;

use hardware as hw;

// logging imports
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use core::cell::RefCell;
use critical_section::Mutex;
use rp235x_hal as hal;
use rp235x_pac as pac;

// This provides the fallback panic handler symbols
#[defmt::panic_handler]
fn panic() -> ! {
    defmt::error!("Panic occurred!");
    loop {

    }
}
defmt::timestamp! {
    "{=u64:us}",
    // Provide your counter or timer tick here, e.g., 0 for a constant or an uptime value:
    0
}

// copy boot metadata to .start_block so Boot ROM knows how to boot our program
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: hal::block::ImageDef = hal::block::ImageDef::secure_exe();

// set external clock frequency
const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;

// Main entrypoint
#[hal::entry]
fn main() -> ! {
    info!("### OpenDumpyOS v0.1r ####");
    info!("Initializing RP2350B...");

    // grab peripherals singleton
    let mut pac_singleton = match hal::pac::Peripherals::take() {
        Some(pac) => pac,
        None => {
            error!("PAC singleton failed to be taken: hal::pac::Peripherals::take() returned None!");
            defmt::panic!();
        }
    };

    // configure watchdog and clock
    let mut watchdog = hal::Watchdog::new(pac_singleton.WATCHDOG);
    let clocks = match hal::clocks::init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac_singleton.XOSC,
        pac_singleton.CLOCKS,
        pac_singleton.PLL_SYS,
        pac_singleton.PLL_USB,
        &mut pac_singleton.RESETS,
        &mut watchdog
    ) {
        Ok(clock) => clock,
        Err(e) => {
            error!("Failed to initialize hardware clocks: hal::clock::InitError");
            defmt::panic!()
        }
    };

    // move ownership of TIMER0 peripheral to create Timer struct
    let timer = hal::Timer::new_timer0(pac_singleton.TIMER0, &mut pac_singleton.RESETS, &clocks);


    // fill hardware resources before gpio stuffs
    info!("Initializing hardware resources...");
    critical_section::with(|cs| {
        hw::RES.sio.borrow(cs).replace(Some(pac_singleton.SIO));
        hw::RES.pac_resets.borrow(cs).replace(Some(pac_singleton.RESETS));
        hw::RES.pac_io_bank0.borrow(cs).replace(Some(pac_singleton.IO_BANK0));
        hw::RES.pac_pads_bank0.borrow(cs).replace(Some(pac_singleton.PADS_BANK0));
        hw::RES.io_state.borrow(cs).replace(Some(hw::IOState::new()));
    });

    // attempt gameboy setup
    gbcart::init();

    // USB loop
    let mut timestamp = timer.get_counter();
    loop {
        if (timer.get_counter() - timestamp).to_millis() > 1000 {
            debug!("MAIN LOOP!");
        }
    }
}
