#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::sdmmc::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( SDMMC1, Sdmmc, SDMMC1_PERIPH, SdmmcPeriph, 0x40012c00, 0x00, 0x09);


