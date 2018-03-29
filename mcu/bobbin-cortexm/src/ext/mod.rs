pub use periph::*;

#[inline(always)]
pub fn wfi() {
    #[cfg(target_os="none")]
    unsafe { asm!("wfi")}
}

#[inline(always)]
pub fn wfe() {
    #[cfg(target_os="none")]
    unsafe { asm!("wfe")}
}

#[inline(always)]
pub fn nop() {
    #[cfg(target_os="none")]
    unsafe { asm!{"nop"}}
}
