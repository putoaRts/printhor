#[cfg(feature = "with-serial-usb")]
compile_error!("Not supported");

#[cfg(feature = "with-serial-port-1")]
pub mod uart_port1 {
    use crate::board::device::UartPort1RxDevice;
    use crate::board::device::UartPort1RingBufferedRxDevice;
    use printhor_hwa_common::TrackedStaticCell;

    pub struct UartPort1RxInputStream {
        receiver: UartPort1RingBufferedRxDevice,
    }

    impl UartPort1RxInputStream {
        pub fn new(receiver: UartPort1RxDevice) -> Self {
            #[link_section =".bss"]
            static BUFF: TrackedStaticCell<[u8; crate::UART_PORT1_BUFFER_SIZE]> = TrackedStaticCell::new();
            let buffer = BUFF.init::<{crate::board::MAX_STATIC_MEMORY}>("UartPort1RXRingBuff", [0; crate::UART_PORT1_BUFFER_SIZE]);

            Self {
                receiver: receiver.into_ring_buffered(buffer),
            }
        }
    }

    impl async_gcode::ByteStream for UartPort1RxInputStream
    {
        type Item = Result<u8, async_gcode::Error>;

        async fn next(&mut self) -> Option<Self::Item> {

            // DMA1 REQ:
            // USART_2 RX ->    CHANNEL_4, STREAM_5 [CH: I2C1_RX, DAC1]
            //                  CHANNEL_6 - STREAM_7 [CH: I2C1_TX, I2C4_TX, I2C2_TX]
            // USART_2 TX ->    CHANNEL_4 STREAM_6 [CH: I2C1_TX, TIM5_UP, DAC2]


            let mut buff: [u8; 1] = [0; 1];

            match self.receiver.read(&mut buff).await {
                Ok(_r) => Some(Ok(buff[0])),
                Err(_e) => None,
            }
        }
    }
}

#[cfg(feature = "with-serial-port-2")]
compiler_error!("Not implemented");