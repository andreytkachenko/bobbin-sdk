pub use ::stm32_common::usart_f24::*;

::bobbin_mcu::periph!( USART1, Usart1, USART1_PERIPH, UsartPeriph, USART1_OWNED, USART1_REF_COUNT, 0x40011000, 0x00, 0x35);
::bobbin_mcu::periph!( USART2, Usart2, USART2_PERIPH, UsartPeriph, USART2_OWNED, USART2_REF_COUNT, 0x40004400, 0x01, 0x36);
::bobbin_mcu::periph!( USART3, Usart3, USART3_PERIPH, UsartPeriph, USART3_OWNED, USART3_REF_COUNT, 0x40004800, 0x02, 0x37);
::bobbin_mcu::periph!( UART4, Uart4, UART4_PERIPH, UsartPeriph, UART4_OWNED, UART4_REF_COUNT, 0x40004c00, 0x03, 0x38);
::bobbin_mcu::periph!( UART5, Uart5, UART5_PERIPH, UsartPeriph, UART5_OWNED, UART5_REF_COUNT, 0x40005000, 0x04, 0x39);
::bobbin_mcu::periph!( USART6, Usart6, USART6_PERIPH, UsartPeriph, USART6_OWNED, USART6_REF_COUNT, 0x40011400, 0x05, 0x3a);
::bobbin_mcu::periph!( UART7, Uart7, UART7_PERIPH, UsartPeriph, UART7_OWNED, UART7_REF_COUNT, 0x40007800, 0x06, 0x3b);
::bobbin_mcu::periph!( UART8, Uart8, UART8_PERIPH, UsartPeriph, UART8_OWNED, UART8_REF_COUNT, 0x40007c00, 0x07, 0x3c);
