use rp235x_pac as pac;
use critical_section::Mutex;
use core::cell::RefCell;

// enum for bus modes
#[derive(PartialEq, Clone, Copy)]
pub enum BusMode {
    READ = 1,
    WRITE = 2
}

// enum for IO states
#[derive(PartialEq, Clone, Copy)]
pub enum IOMode {
    ENABLED = 1,
    DISABLED = 2
}

// create struct to store IO bus states
pub struct IOState {
    pub dbus: BusMode,
    pub cbus: IOMode,
    pub lcdbus: IOMode,
    pub usdbus: IOMode
}
impl IOState {
    pub const fn new() -> Self {
        Self {
            dbus: BusMode::READ,
            cbus: IOMode::DISABLED,
            lcdbus: IOMode::DISABLED,
            usdbus: IOMode::DISABLED
        }
    }
}

// create resources struct
pub struct HwRes {
    pub sio: Mutex<RefCell<Option<pac::SIO>>>,
    pub pac_resets: Mutex<RefCell<Option<pac::RESETS>>>,
    pub pac_io_bank0: Mutex<RefCell<Option<pac::IO_BANK0>>>,
    pub pac_pads_bank0: Mutex<RefCell<Option<pac::PADS_BANK0>>>,
    pub io_state: Mutex<RefCell<Option<IOState>>>   // gotta be a better way for this one in
                                                    // particular...
}

// Create hardware resources mutex struct thing
pub static RES: HwRes = HwRes {
    sio           : Mutex::new(RefCell::new(None)),
    pac_resets    : Mutex::new(RefCell::new(None)),
    pac_io_bank0  : Mutex::new(RefCell::new(None)),
    pac_pads_bank0: Mutex::new(RefCell::new(None)),
    io_state      : Mutex::new(RefCell::new(None)) // see above
};


#[macro_export]
macro_rules! take_sio {
    ( $cs:expr ) => {
        match hw::RES.sio.borrow_ref_mut($cs).take() {
            Some(sio) => sio,
            None => {
                error!("Failed to take SIO from hw::RES!");
                defmt::panic!();
            }
        }
    };
}
#[macro_export]
macro_rules! return_sio {
    ( $cs: expr, $sio:expr ) => {
        hw::RES.sio.borrow($cs).replace(Some($sio));
    }
}
pub(crate) use take_sio;
pub(crate) use return_sio;

#[macro_export]
macro_rules! take_resets {
    ( $cs:expr ) => {
        match hw::RES.pac_resets.borrow_ref_mut($cs).take() {
            Some(resets) => resets,
            None => {
                error!("Failed to take RESETS from hw::RES!");
                defmt::panic!();
            }
        }
    }
}
#[macro_export]
macro_rules! return_resets {
    ( $cs:expr, $resets:expr ) => {
        hw::RES.pac_resets.borrow($cs).replace(Some($resets));
    }
}
pub(crate) use take_resets;
pub(crate) use return_resets;

#[macro_export]
macro_rules! take_pads_bank0 {
    ( $cs:expr ) => {
        match hw::RES.pac_pads_bank0.borrow_ref_mut($cs).take() {
            Some(pads) => pads,
            None => {
                error!("Failed to take PADS_BANK0 from hw::RES!");
                defmt::panic!();
            }
        }
    }
}
#[macro_export]
macro_rules! return_pads_bank0 {
    ( $cs:expr, $pads_bank0:expr ) => {
        hw::RES.pac_pads_bank0.borrow($cs).replace(Some($pads_bank0));
    }
}
pub(crate) use take_pads_bank0;
pub(crate) use return_pads_bank0;

#[macro_export]
macro_rules! take_io_bank0 {
    ( $cs:expr ) => {
        match hw::RES.pac_io_bank0.borrow_ref_mut($cs).take() {
            Some(io) => io,
            None => {
                error!("Failed to take IO_BANK0 from hw::RES!");
                defmt::panic!();
            }
        }
    }
}
#[macro_export]
macro_rules! return_io_bank0 {
    ( $cs:expr, $io_bank0:expr ) => {
        hw::RES.pac_io_bank0.borrow($cs).replace(Some($io_bank0));
    }
}
pub(crate) use take_io_bank0;
pub(crate) use return_io_bank0;

#[macro_export]
macro_rules! take_io_state {
    ( $cs:expr ) => {
        match hw::RES.io_state.borrow_ref_mut($cs).take() {
            Some(state) => state,
            None => {
                error!("Failed to take io_state from hw::RES!");
                defmt::panic!();
            }
        }
    }
}
#[macro_export]
macro_rules! return_io_state {
    ( $cs:expr, $io_state:expr ) => {
        hw::RES.io_state.borrow($cs).replace(Some($io_state));
    }
}
pub(crate) use take_io_state;
pub(crate) use return_io_state;
