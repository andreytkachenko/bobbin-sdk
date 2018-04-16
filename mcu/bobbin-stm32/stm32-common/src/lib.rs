#![no_std]

extern crate bobbin_bits;
extern crate bobbin_mcu;
extern crate bobbin_hal;

pub mod ext;
pub mod exti;
pub mod dma;
pub mod dma_f3;
pub mod gpio;
pub mod lpuart;
pub mod usart;
pub mod usart_f24;
pub mod lptim;
pub mod tim_bas;
pub mod tim_gen;
pub mod tim_adv;
pub mod crc;
pub mod crc_24;
pub mod rng;
pub mod rtc;
pub mod iwdg;
pub mod wwdg;
pub mod adc_f1;
pub mod adc_f24;
pub mod adc_f3;
pub mod adc_l0;
pub mod i2c_v1;
pub mod i2c_v2;
pub mod spi_v1;
pub mod spi_v2;
pub mod dcmi;
pub mod dac;
pub mod sig;
pub mod pin;

