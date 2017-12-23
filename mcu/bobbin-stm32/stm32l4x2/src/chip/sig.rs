//! Signals

pub trait Signal<T> {}

pub trait I2cSmba {}
pub trait SignalI2cSmba<T> {}
pub trait I2cScl {}
pub trait SignalI2cScl<T> {}
pub trait I2cSda {}
pub trait SignalI2cSda<T> {}
pub trait Adc {}
pub trait SignalAdc<T> {}
pub trait Tx {}
pub trait SignalTx<T> {}
pub trait Rx {}
pub trait SignalRx<T> {}
pub trait Cts {}
pub trait SignalCts<T> {}
pub trait Rts {}
pub trait SignalRts<T> {}
pub trait Ck {}
pub trait SignalCk<T> {}
pub trait SpiNss {}
pub trait SignalSpiNss<T> {}
pub trait SpiMiso {}
pub trait SignalSpiMiso<T> {}
pub trait SpiMosi {}
pub trait SignalSpiMosi<T> {}
pub trait SpiSck {}
pub trait SignalSpiSck<T> {}

pub struct I2c1Smba {}
pub struct I2c1Scl {}
pub struct I2c1Sda {}
pub struct I2c2Smba {}
pub struct I2c2Scl {}
pub struct I2c2Sda {}
pub struct I2c3Smba {}
pub struct I2c3Scl {}
pub struct I2c3Sda {}
pub struct I2c4Smba {}
pub struct I2c4Scl {}
pub struct I2c4Sda {}
pub struct Adc1In1 {}
pub struct Adc1In2 {}
pub struct Adc1In3 {}
pub struct Adc1In4 {}
pub struct Adc1In5 {}
pub struct Adc1In6 {}
pub struct Adc1In7 {}
pub struct Adc1In8 {}
pub struct Adc1In9 {}
pub struct Adc1In10 {}
pub struct Adc1In11 {}
pub struct Adc1In12 {}
pub struct Adc1In13 {}
pub struct Adc1In14 {}
pub struct Adc1In15 {}
pub struct Usart1Tx {}
pub struct Usart1Rx {}
pub struct Usart1Cts {}
pub struct Usart1Rts {}
pub struct Usart1Ck {}
pub struct Usart2Tx {}
pub struct Usart2Rx {}
pub struct Usart2Cts {}
pub struct Usart2Rts {}
pub struct Usart2Ck {}
pub struct Usart3Tx {}
pub struct Usart3Rx {}
pub struct Usart3Cts {}
pub struct Usart3Rts {}
pub struct Usart3Ck {}
pub struct Uart4Tx {}
pub struct Uart4Rx {}
pub struct Uart4Cts {}
pub struct Uart4Rts {}
pub struct Uart4Ck {}
pub struct Lpuart1Tx {}
pub struct Lpuart1Rx {}
pub struct Lpuart1Cts {}
pub struct Lpuart1Rts {}
pub struct Spi1Nss {}
pub struct Spi1Miso {}
pub struct Spi1Mosi {}
pub struct Spi1Sck {}
pub struct Spi3Nss {}
pub struct Spi3Miso {}
pub struct Spi3Mosi {}
pub struct Spi3Sck {}
pub struct Spi2Nss {}
pub struct Spi2Miso {}
pub struct Spi2Mosi {}
pub struct Spi2Sck {}
pub struct Tim2Ch1 {}
pub struct Comp1Out {}
pub struct Sai1Extclk {}
pub struct Tim2Etr {}
pub struct Eventout {}
pub struct Tim2Ch2 {}
pub struct Usart2RtsDe {}
pub struct Tim15Ch1n {}
pub struct Tim2Ch3 {}
pub struct QuadspiBk1Ncs {}
pub struct Comp2Out {}
pub struct Tim15Ch1 {}
pub struct Tim2Ch4 {}
pub struct QuadspiClk {}
pub struct Sai1MclkA {}
pub struct Tim15Ch2 {}
pub struct Sai1FsB {}
pub struct Lptim2Out {}
pub struct Dfsdm1Ckout {}
pub struct Lptim2Etr {}
pub struct Tim1Bkin {}
pub struct Tim3Ch1 {}
pub struct QuadspiBk1Io3 {}
pub struct Tim1BkinComp2 {}
pub struct Tim16Ch1 {}
pub struct Tim1Ch1n {}
pub struct Tim3Ch2 {}
pub struct Dfsdm1Datin0 {}
pub struct QuadspiBk1Io2 {}
pub struct Mco {}
pub struct Tim1Ch1 {}
pub struct Dfsdm1Ckin1 {}
pub struct Sai1SckA {}
pub struct Tim1Ch2 {}
pub struct Dfsdm1Datin1 {}
pub struct Sai1FsA {}
pub struct Tim15Bkin {}
pub struct Tim1Ch3 {}
pub struct UsbcrsSync {}
pub struct Sai1SdA {}
pub struct Tim1Ch4 {}
pub struct Tim1Bkin2 {}
pub struct Can1Rx {}
pub struct Usbdm {}
pub struct Tim1Bkin2Comp1 {}
pub struct Tim1Etr {}
pub struct Usart1RtsDe {}
pub struct Can1Tx {}
pub struct Usbdp {}
pub struct Jtms {}
pub struct Swdat {}
pub struct IrOut {}
pub struct Usbnoe {}
pub struct Sai1SdB {}
pub struct Jtck {}
pub struct Swclk {}
pub struct Lptim1Out {}
pub struct Jtdi {}
pub struct Usart3RtsDe {}
pub struct Uart4RtsDe {}
pub struct TscG3Io1 {}
pub struct Tim1Ch2n {}
pub struct Tim3Ch3 {}
pub struct Dfsdm1Ckin0 {}
pub struct QuadspiBk1Io1 {}
pub struct Tim1Ch3n {}
pub struct Tim3Ch4 {}
pub struct Lpuart1RtsDe {}
pub struct QuadspiBk1Io0 {}
pub struct Lptim2In1 {}
pub struct RtcOut {}
pub struct Jtdo {}
pub struct Traceswo {}
pub struct Sai1SckB {}
pub struct Njtrst {}
pub struct TscG2Io1 {}
pub struct Sai1MclkB {}
pub struct Lptim1In1 {}
pub struct TscG2Io2 {}
pub struct Tim16Bkin {}
pub struct Lptim1Etr {}
pub struct TscG2Io3 {}
pub struct Tim16Ch1n {}
pub struct Lptim1In2 {}
pub struct TscG2Io4 {}
pub struct Sdmmc1D4 {}
pub struct Sdmmc1D5 {}
pub struct TscSync {}
pub struct TscG1Io1 {}
pub struct TscG1Io2 {}
pub struct Dfsdm1Datin2 {}
pub struct TscG1Io3 {}
pub struct RtcRefin {}
pub struct Dfsdm1Ckin2 {}
pub struct TscG1Io4 {}
pub struct Traced0 {}
pub struct Dfsdm1Ckin3 {}
pub struct TscG4Io1 {}
pub struct Sdmmc1D6 {}
pub struct Dfsdm1Datin3 {}
pub struct TscG4Io2 {}
pub struct Sdmmc1D7 {}
pub struct TscG4Io3 {}
pub struct Sdmmc1D0 {}
pub struct TscG4Io4 {}
pub struct Sdmmc1D1 {}
pub struct Traced1 {}
pub struct TscG3Io2 {}
pub struct Sdmmc1D2 {}
pub struct TscG3Io3 {}
pub struct Sdmmc1D3 {}
pub struct Traced3 {}
pub struct TscG3Io4 {}
pub struct Sdmmc1Ck {}
pub struct Traced2 {}
pub struct Tim3Etr {}
pub struct Sdmmc1Cmd {}
pub struct QuadspiBk2Ncs {}
pub struct QuadspiBk2Io0 {}
pub struct QuadspiBk2Io1 {}
pub struct QuadspiBk2Io2 {}
pub struct QuadspiBk2Io3 {}
pub struct TscG6Io1 {}
pub struct TscG6Io2 {}
pub struct TscG6Io3 {}
pub struct TscG6Io4 {}
pub struct Traceck {}
pub struct TscG7Io1 {}
pub struct TscG7Io2 {}
pub struct TscG7Io3 {}
pub struct TscG7Io4 {}
pub struct TscG5Io1 {}
pub struct TscG5Io2 {}
pub struct TscG5Io3 {}
pub struct TscG5Io4 {}
pub struct Tim1Bkin2Comp2 {}
pub struct Tim1BkinComp1 {}
