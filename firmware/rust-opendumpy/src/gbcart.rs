#![allow(unused)]

use core::panic;
use critical_section::Mutex;
use core::sync::atomic::AtomicPtr;
use core::mem::MaybeUninit;
use rp235x_pac as pac;

// pull hardware resources into this file
use super::hardware as hw;


// set module global dbus mode var to default: write
//static DBUS_MODE: hw::BusMode = hw::BusMode::WRITE;

// set module global cbus state var to disabled (false)
//static CBUS_STATE: bool = false;

// Represents GPIO[28:0] and GPIO[41]
const IO_MASK_LO  : u32 = 0x1fff_ffff;
const IO_MASK_HI  : u32 = 0x0000_0200;
// Represents GPIO[27:20]
const DBUS_MASK: u32 = 0x07F8_0000;
// Represents GPIO[19:0]
const ABUS_MASK: u32 = 0x0007_FFFF;
// Represents GPIO[41]; to be used with pac::SIO::gpio_hi_out_set()
const CBUS_EN_MASK: u32 = 0x0000_0200;

// Bit shift required to move byte to correct position for write mask; 20 pins left -> 19 bits left
const DBUS_BIT_SHIFT: u8 = 19;


pub fn init() {
    // function select value for SIO gpio function
    const FUNCSEL_SIO: u8 = 5;

    // enter critical section to use hardware resources
    critical_section::with(|cs| {
        // take necessary hardware resources
        let sio = hw::RES.sio.borrow_ref_mut(cs).take().unwrap();
        let resets = hw::RES.pac_resets.borrow_ref_mut(cs).take().unwrap();
        let pads_bank0 = hw::RES.pac_pads_bank0.borrow_ref_mut(cs).take().unwrap();
        let io_bank0 = hw::RES.pac_io_bank0.borrow_ref_mut(cs).take().unwrap();

        // take the IO mux and the pad bank out of reset
        resets.reset().modify(|_, w| {
            w.io_bank0().clear_bit();
            w.pads_bank0().clear_bit()
        });
        while resets.reset_done().read().io_bank0().bit_is_clear()
            || resets.reset_done().read().pads_bank0().bit_is_clear() {}

        // set the output data low *before* enabling any drivers, so nothing glitches high
        sio.gpio_out_clr().write(|w| unsafe { w.bits(IO_MASK_LO) });
        sio.gpio_oe_clr().write(|w| unsafe { w.bits(IO_MASK_LO) });
        sio.gpio_hi_out_clr().write(|w| unsafe { w.bits(IO_MASK_HI) });
        sio.gpio_hi_oe_clr().write(|w| unsafe { w.bits(IO_MASK_HI) });


        // set up pads and SIO
        for n in 0..29usize {
            // Pad setup
            // - output driver on
            // - input buffer off
            // - pull-down on
            // ISO is deliberately left set here
            pads_bank0.gpio(n).modify(|_, w| {
                w.od().clear_bit();     // output disable = 0
                w.ie().clear_bit();     // input buffer off
                w.pue().clear_bit();    // pull-up disabled
                w.pde().set_bit()       // pull-down enabled
            });

            // set pin function to SIO
            io_bank0.gpio(n).gpio_ctrl().write(|w| unsafe { w.funcsel().bits(FUNCSEL_SIO) });
        }
        // do the same for GPIO41
        pads_bank0.gpio(41usize).modify(|_, w| {
            w.od().clear_bit();     // output disable = 0
            w.ie().clear_bit();     // input buffer off
            w.pue().clear_bit();    // pull-up disabled
            w.pde().set_bit()       // pull-down enabled
        });
        io_bank0.gpio(41usize).gpio_ctrl().write(|w| unsafe { w.funcsel().bits(FUNCSEL_SIO) });

        // enable output drivers
        sio.gpio_oe_set().write(|w| unsafe { w.bits(IO_MASK_LO) });
        sio.gpio_hi_oe_set().write(|w| unsafe { w.bits(IO_MASK_HI) });

        // then drop the pad isolation latches.
        for n in 0..29usize {
            pads_bank0.gpio(n).modify(|_,w| w.iso().clear_bit());
        }
        // do the same for GPIO41
        pads_bank0.gpio(41usize).modify(|_,w| w.iso().clear_bit());

        // hardware resources no longer needed. release mutexes
        hw::RES.sio.borrow(cs).replace(Some(sio));
        hw::RES.pac_resets.borrow(cs).replace(Some(resets));
        hw::RES.pac_pads_bank0.borrow(cs).replace(Some(pads_bank0));
        hw::RES.pac_io_bank0.borrow(cs).replace(Some(io_bank0));
    });
}

fn set_dbus(mode: hw::BusMode) {
    // enter critical section
    critical_section::with(|cs| {
        // take io_state from hw::RES
        let mut io_state = hw::RES.io_state.borrow_ref_mut(cs).take().unwrap();

        // if bus already in requested mode, do nothing and return
        if io_state.dbus == mode {
            // return io_state to hw::RES
            hw::RES.io_state.borrow(cs).replace(Some(io_state));
            return
        }

        // take SIO & pads_bank0 from mutex
        let sio = hw::RES.sio.borrow_ref_mut(cs).take().unwrap();
        let pads_bank0 = hw::RES.pac_pads_bank0.borrow_ref_mut(cs).take().unwrap();

        // affect change on bus
        match mode {
            // configure bus GPIO for read mode
            hw::BusMode::READ => {
                // release output drivers to set bus to high impedence
                sio.gpio_oe_clr().write(|w| unsafe { w.bits(DBUS_MASK) });

                // reconfigure each pin on dbus (GPIO 20-28) to input
                for n in 20..29usize {
                    pads_bank0.gpio(n).write(|w| unsafe {
                        w.pue().clear_bit();    // pull-up disabled
                        w.pde().clear_bit();    // pull-down disabled
                        w.ie().clear_bit();     // input buffer on
                        w.od().set_bit()        // enable output path
                    });
                }

                // clear stale data to prevent accidental output when re-enabling
                sio.gpio_out_clr().write(|w| unsafe { w.bits(DBUS_MASK) });

                // update dbus mode
                io_state.dbus = hw::BusMode::READ;
            },
            // configure bus GPIO for write mode
            hw::BusMode::WRITE => {
                // clear stale data to prevent accidental output when re-enabling
                sio.gpio_out_clr().write(|w| unsafe { w.bits(DBUS_MASK) });

                // reconfigure each pin on dbus (GPIO 20-28) to output
                for n in 20..29usize {
                    pads_bank0.gpio(n).write(|w| unsafe {
                        w.ie().clear_bit();     // input buffer off
                        w.od().clear_bit();     // enable output
                        w.pue().clear_bit();    // pull-up disabled
                        w.pde().set_bit()       // pull-down enabled
                    });
                }

                // enable output drivers
                sio.gpio_oe_set().write(|w| unsafe { w.bits(DBUS_MASK) });

                // update dbus mode
                io_state.dbus = hw::BusMode::WRITE;
            }
        };

        // return hardware resources
        hw::RES.sio.borrow(cs).replace(Some(sio));
        hw::RES.pac_pads_bank0.borrow(cs).replace(Some(pads_bank0));
        hw::RES.io_state.borrow(cs).replace(Some(io_state));
    });
}

// internal function; assumes dbus has already been set to read mode
#[inline]
fn sio_read(sio: &pac::SIO, addr: u32) -> u8 {
    // set address
    let abus_state = sio.gpio_out().read().bits() & ABUS_MASK;
    sio.gpio_out_xor().write(|w| unsafe {
        w.bits((abus_state ^ addr) & ABUS_MASK)
    });

    // read byte
    ((sio.gpio_in().read().bits() & DBUS_MASK) >> 19) as u8
}

// internal function; assumes dbus has already been set to write mode
#[inline]
fn sio_write(sio: &pac::SIO, addr: u32, byte: u8) {
    // get current data out on io bus
    let io_bus_state = sio.gpio_out().read().bits();

    // get dbus and abus states
    let dbus_state = io_bus_state & DBUS_MASK;
    let abus_state = io_bus_state & ABUS_MASK;

    // shift byte data to correct position for write
    let byte_mask = (byte as u32) << DBUS_BIT_SHIFT;

    // write data to io bus
    sio.gpio_out_xor()
       .write(|w| unsafe {
           w.bits((dbus_state ^ byte_mask) & DBUS_MASK)
       });
}

// assumes dbus already in READ mode
pub fn read(addr: u32) -> u8 {
    // byte var
    let mut byte = 0u8;
    
    // enter critical section
    critical_section::with(|cs| {
        // Take SIO from hw::RES
        let sio = hw::RES.sio.borrow_ref_mut(cs).take().unwrap();
        // read pins
        byte = self::sio_read(&sio, addr);
        // return SIO to hw::RES
        hw::RES.sio.borrow(cs).replace(Some(sio));
    });

    // return byte var
    byte
}

// assumes dbus already in READ mode
pub fn read_bytes(addr: u32, buffer: &mut [u8]) {
    // enter critical section
    critical_section::with(|cs| {
        // take SIO from hw::RES
        let sio = hw::RES.sio.borrow_ref_mut(cs).take().unwrap();

        // read each byte and store in buffer
        for (n, byte) in buffer.iter_mut().enumerate() {
            *byte = self::sio_read(&sio, addr + n as u32);
        }

        // return SIO to hw::RES
        hw::RES.sio.borrow(cs).replace(Some(sio));
    });
}

// assumes dbus already in WRITE mode
pub fn write(addr: u32, byte: u8) {
    // enter critical section
    critical_section::with(|cs| {
        // take SIO from hw::RES
        let sio = hw::RES.sio.borrow_ref_mut(cs).take().unwrap();

        // write byte
        self::sio_write(&sio, addr, byte);

        // return SIO to hw::RES
        hw::RES.sio.borrow(cs).replace(Some(sio));
    });
}

// assumes dbus already in WRITE mode
pub fn write_bytes(addr: u32, bytes: &[u8]) {
    // enter critical section
    critical_section::with(|cs| {
        // take SIO from hw::RES
        let sio = hw::RES.sio.borrow_ref_mut(cs).take().unwrap();

        // write each byte to specified addr
        for (n, byte) in bytes.iter().enumerate() {
            self::sio_write(&sio, addr + n as u32, *byte);
        }

        // return SIO to hw::RES
        hw::RES.sio.borrow(cs).replace(Some(sio));
    });
}

pub fn cbus_enable() {
    // enter critical section
    critical_section::with(|cs| {
        // grap io_state from hw::RES
        let mut io_state = hw::RES.io_state.borrow_ref_mut(cs).take().unwrap();
        // if cbus already enabled, do nothing and return
        if io_state.cbus == hw::IOMode::ENABLED {
            // return io_state to hw::RES
            hw::RES.io_state.borrow(cs).replace(Some(io_state));
            return
        }

        // take SIO from hw::RES
        let sio = hw::RES.sio.borrow_ref_mut(cs).take().unwrap();
        // set GPIO41
        sio.gpio_hi_out_set().write(|w| unsafe { w.bits(CBUS_EN_MASK) });

        // update io_state
        io_state.cbus = hw::IOMode::ENABLED;

        // return resources to hw::RES
        hw::RES.sio.borrow(cs).replace(Some(sio));
        hw::RES.io_state.borrow(cs).replace(Some(io_state));
    });
}

pub fn cbus_disable() {
    // enter critical section
    critical_section::with(|cs| {
        // grap io_state from hw::RES
        let mut io_state = hw::RES.io_state.borrow_ref_mut(cs).take().unwrap();
        // if cbus already disabled, do nothing and return
        if io_state.cbus == hw::IOMode::DISABLED {
            // return io_state to hw::RES
            hw::RES.io_state.borrow(cs).replace(Some(io_state));
            return
        }

        // take SIO from hw::RES
        let sio = hw::RES.sio.borrow_ref_mut(cs).take().unwrap();
        // clr GPIO41
        sio.gpio_hi_out_clr().write(|w| unsafe { w.bits(CBUS_EN_MASK) });

        // update io_state
        io_state.cbus = hw::IOMode::DISABLED;

        // return resources to hw::RES
        hw::RES.sio.borrow(cs).replace(Some(sio));
        hw::RES.io_state.borrow(cs).replace(Some(io_state));
    });
}
