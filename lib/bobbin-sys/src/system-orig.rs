use console::{Console, console_ref};

#[cfg(feature="logger")]
use logger::Logger;

use core::cell::UnsafeCell;

static mut SYSTEM_DATA: UnsafeCell<SystemData> = UnsafeCell::new(SystemData { locked: false });

struct SystemData {
    locked: bool,
}

pub struct Config {
}

impl Default for Config {
    fn default() -> Config {
        Config {}
    }
}

#[must_use]
#[derive(Default)]
pub struct System<MCU, CLK, DIS, TCK> 
where
    MCU: Default,
    CLK: Default,
    DIS: Default,
    TCK: Default,
{
    mcu: MCU,
    clock: CLK,
    tick: TCK,
    #[cfg(feature="logger")]
    logger: Logger,
    dispatcher: DIS,
}

impl<MCU, CLK, DIS, TCK> System<MCU, CLK, DIS, TCK> 
where
    MCU: Default,
    CLK: Default,
    DIS: Default,
    TCK: Default,
{

    pub fn init<F: FnOnce()>(f: F) -> Self {
        Self::disable_interrupts();
        Self::lock();

        f();

        System {
            mcu: MCU::default(),
            clock: CLK::default(),
            tick: TCK::default(),
            #[cfg(feature="logger")]
            logger: Logger::default(),
            dispatcher: DIS::default(),
        }
    }

    fn data() -> &'static mut SystemData {
        unsafe { &mut *SYSTEM_DATA.get() }
    }

    #[inline]
    fn enable_interrupts() {
        unsafe { asm!("cpsie i") }
    }

    #[inline]
    fn disable_interrupts() {
        unsafe { asm!("cpsid i") }
    }

    fn locked() -> bool {
        Self::data().locked    
    }

    fn lock() {
        while Self::locked() {}
        Self::data().locked = true;
    }

    fn unlock() {
        Self::data().locked = false;
    }

    pub fn mcu(&self) -> &MCU {
        &self.mcu
    }

    pub fn mcu_mut(&mut self) -> &mut MCU {
        &mut self.mcu
    }

    pub fn clock(&self) -> &CLK {
        &self.clock
    }

    pub fn tick_ref(&self) -> &TCK {
        &self.tick
    }

    pub fn tick(&self) -> TCK {
        TCK::default()
    }

    pub fn console(&self) -> Option<&Console> {
        console_ref()
    }

    #[cfg(feature="logger")]
    pub fn logger(&self) -> &Logger {
        &self.logger
    }

    pub fn dispatcher(&self) -> &DIS {
        &self.dispatcher
    }

    pub fn dispatcher_mut(&mut self) -> &mut DIS {
        &mut self.dispatcher
    }

    pub fn run<T, F: FnOnce(&Self) -> T>(&mut self, f: F) -> T {
        Self::enable_interrupts();
        let ret = f(self);
        Self::disable_interrupts();
        ret
    }
}

impl<MCU, CLK, DIS, TCK> Drop for System<MCU, CLK, DIS, TCK> 
where
    MCU: Default,
    CLK: Default,
    DIS: Default,
    TCK: Default,
{
    fn drop(&mut self) {
        Self::unlock();
        Self::enable_interrupts()
    }
}