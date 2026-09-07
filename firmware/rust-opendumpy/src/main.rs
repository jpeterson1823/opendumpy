#![no_std]
#![no_main]

#![allow(unused)]

mod gbcart;

use core::panic::PanicInfo;
use core::cell::RefCell;
use critical_section::Mutex;
use rp235x_hal as hal;
use rp235x_pac as pac;

// usb stuff
use usb_device::{class_prelude::*, prelude::*};
use usbd_serial::SerialPort;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// copy boot metadata to .start_block so Boot ROM knows how to boot our program
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: hal::block::ImageDef = hal::block::ImageDef::secure_exe();

// set external clock frequency
const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;

// create resources struct
pub struct HwRes {
    sio: Mutex<RefCell<Option<pac::SIO>>>,
    pac_resets: Mutex<RefCell<Option<pac::RESETS>>>,
    pac_io_bank0: Mutex<RefCell<Option<pac::IO_BANK0>>>,
    pac_pads_bank0: Mutex<RefCell<Option<pac::PADS_BANK0>>>
}

// create pac::SIO mutex

static HWRES: HwRes = HwRes {
    sio           : Mutex::new(RefCell::new(None)),
    pac_resets    : Mutex::new(RefCell::new(None)),
    pac_io_bank0  : Mutex::new(RefCell::new(None)),
    pac_pads_bank0: Mutex::new(RefCell::new(None))
};

// Main entrypoint
#[hal::entry]
unsafe fn main() -> ! {
    // grab peripherals singleton
    let mut pac_singleton = hal::pac::Peripherals::take().unwrap();

    // set PAC_SIO_MUTEX
    critical_section::with(|cs| HWRES.sio.borrow(cs).replace(Some(pac_singleton.SIO)));

    // configure watchdog and clock
    let mut watchdog = hal::Watchdog::new(pac_singleton.WATCHDOG);
    let clocks = hal::clocks::init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac_singleton.XOSC,
        pac_singleton.CLOCKS,
        pac_singleton.PLL_SYS,
        pac_singleton.PLL_USB,
        &mut pac_singleton.RESETS,
        &mut watchdog
    )
    .ok()
    .unwrap();

    // move ownership of TIMER0 peripheral to create Timer struct
    let timer = hal::Timer::new_timer0(pac_singleton.TIMER0, &mut pac_singleton.RESETS, &clocks);

    // initialize USB driver
    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac_singleton.USB,
        pac_singleton.USB_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac_singleton.RESETS
    ));
    
    // configure usb as cdc
    let mut serial = SerialPort::new(&usb_bus);

    // create usb device with fake vid & pid
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0xdead, 0xbeef))
        .strings(&[StringDescriptors::default()
            .manufacturer("Bingus Industries")
            .product("OpenDumpy v3")
            .serial_number("L33T")])
        .unwrap()
        .device_class(2)
        .build();

    // attempt gameboy setup
    let _ = serial.write(b"Attempting gbcart::init()...\r\n");
    gbcart::init();
    let _ = serial.write(b"gbcart::init() ran without crash.\r\n");

    // USB loop
    let mut rx_buf = [0u8; 64];
    let mut timestamp = timer.get_counter();
    loop {
        // Needs to be called at least every 10 ms
        if usb_dev.poll(&mut [&mut serial]) {
            match serial.read(&mut rx_buf) {
                Ok(0) => {}
                Ok(count) => {
                    // Challenge for student!
                    rx_buf[..count].iter_mut().for_each(|byte| {
                        *byte = byte.to_ascii_uppercase();
                    });
                    let _ = serial.write(&rx_buf[0..count]);
                }
                Err(_e) => {}
            }
        }

        // Send message every second (non-blocking)
        if (timer.get_counter() - timestamp).to_millis() >= 1_000 {
            timestamp = timer.get_counter();
            let _ = serial.write(b"gameboy::init() ran without crash.\r\n");
        }
    }
}
