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
