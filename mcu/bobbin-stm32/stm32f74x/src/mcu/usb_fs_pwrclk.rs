#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::usb_fs_pwrclk::*;

periph!( USB_FS_PWRCLK, UsbFsPwrclk, USB_FS_PWRCLK_PERIPH, UsbFsPwrclkPeriph, 0x50000e00, 0x17);
