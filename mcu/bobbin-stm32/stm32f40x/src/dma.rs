pub use ::stm32_common::dma::*;

::bobbin_mcu::periph!( DMA1, Dma1, DMA1_PERIPH, DmaPeriph, DMA1_OWNED, DMA1_REF_COUNT, 0x40026000, 0x00, 0x39);
::bobbin_mcu::periph!( DMA2, Dma2, DMA2_PERIPH, DmaPeriph, DMA2_OWNED, DMA2_REF_COUNT, 0x40026400, 0x01, 0x3a);

::bobbin_mcu::channel!(DMA1_STREAM0, Dma1Stream0, dma1_stream0, DMA1, Dma1, DMA1_STREAM0_CH, DmaCh, DMA1_PERIPH, DMA1_STREAM0_OWNED, DMA1_STREAM0_REF_COUNT, 0);
::bobbin_mcu::channel!(DMA1_STREAM1, Dma1Stream1, dma1_stream1, DMA1, Dma1, DMA1_STREAM1_CH, DmaCh, DMA1_PERIPH, DMA1_STREAM1_OWNED, DMA1_STREAM1_REF_COUNT, 1);
::bobbin_mcu::channel!(DMA1_STREAM2, Dma1Stream2, dma1_stream2, DMA1, Dma1, DMA1_STREAM2_CH, DmaCh, DMA1_PERIPH, DMA1_STREAM2_OWNED, DMA1_STREAM2_REF_COUNT, 2);
::bobbin_mcu::channel!(DMA1_STREAM3, Dma1Stream3, dma1_stream3, DMA1, Dma1, DMA1_STREAM3_CH, DmaCh, DMA1_PERIPH, DMA1_STREAM3_OWNED, DMA1_STREAM3_REF_COUNT, 3);
::bobbin_mcu::channel!(DMA1_STREAM4, Dma1Stream4, dma1_stream4, DMA1, Dma1, DMA1_STREAM4_CH, DmaCh, DMA1_PERIPH, DMA1_STREAM4_OWNED, DMA1_STREAM4_REF_COUNT, 4);
::bobbin_mcu::channel!(DMA1_STREAM5, Dma1Stream5, dma1_stream5, DMA1, Dma1, DMA1_STREAM5_CH, DmaCh, DMA1_PERIPH, DMA1_STREAM5_OWNED, DMA1_STREAM5_REF_COUNT, 5);
::bobbin_mcu::channel!(DMA1_STREAM6, Dma1Stream6, dma1_stream6, DMA1, Dma1, DMA1_STREAM6_CH, DmaCh, DMA1_PERIPH, DMA1_STREAM6_OWNED, DMA1_STREAM6_REF_COUNT, 6);
::bobbin_mcu::channel!(DMA1_STREAM7, Dma1Stream7, dma1_stream7, DMA1, Dma1, DMA1_STREAM7_CH, DmaCh, DMA1_PERIPH, DMA1_STREAM7_OWNED, DMA1_STREAM7_REF_COUNT, 7);
::bobbin_mcu::channel!(DMA2_STREAM0, Dma2Stream0, dma2_stream0, DMA2, Dma2, DMA2_STREAM0_CH, DmaCh, DMA2_PERIPH, DMA2_STREAM0_OWNED, DMA2_STREAM0_REF_COUNT, 0);
::bobbin_mcu::channel!(DMA2_STREAM1, Dma2Stream1, dma2_stream1, DMA2, Dma2, DMA2_STREAM1_CH, DmaCh, DMA2_PERIPH, DMA2_STREAM1_OWNED, DMA2_STREAM1_REF_COUNT, 1);
::bobbin_mcu::channel!(DMA2_STREAM2, Dma2Stream2, dma2_stream2, DMA2, Dma2, DMA2_STREAM2_CH, DmaCh, DMA2_PERIPH, DMA2_STREAM2_OWNED, DMA2_STREAM2_REF_COUNT, 2);
::bobbin_mcu::channel!(DMA2_STREAM3, Dma2Stream3, dma2_stream3, DMA2, Dma2, DMA2_STREAM3_CH, DmaCh, DMA2_PERIPH, DMA2_STREAM3_OWNED, DMA2_STREAM3_REF_COUNT, 3);
::bobbin_mcu::channel!(DMA2_STREAM4, Dma2Stream4, dma2_stream4, DMA2, Dma2, DMA2_STREAM4_CH, DmaCh, DMA2_PERIPH, DMA2_STREAM4_OWNED, DMA2_STREAM4_REF_COUNT, 4);
::bobbin_mcu::channel!(DMA2_STREAM5, Dma2Stream5, dma2_stream5, DMA2, Dma2, DMA2_STREAM5_CH, DmaCh, DMA2_PERIPH, DMA2_STREAM5_OWNED, DMA2_STREAM5_REF_COUNT, 5);
::bobbin_mcu::channel!(DMA2_STREAM6, Dma2Stream6, dma2_stream6, DMA2, Dma2, DMA2_STREAM6_CH, DmaCh, DMA2_PERIPH, DMA2_STREAM6_OWNED, DMA2_STREAM6_REF_COUNT, 6);
::bobbin_mcu::channel!(DMA2_STREAM7, Dma2Stream7, dma2_stream7, DMA2, Dma2, DMA2_STREAM7_CH, DmaCh, DMA2_PERIPH, DMA2_STREAM7_OWNED, DMA2_STREAM7_REF_COUNT, 7);
