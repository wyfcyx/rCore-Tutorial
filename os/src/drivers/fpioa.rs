use crate::platform::FPIOA_BASE_ADDR;

const IO_PIN4_OFFSET: usize = 0x10;
const IP_PIN5_OFFSET: usize = 0x14;
const FUNC_UART1_RX: u32 = 64;
const FUNC_UART1_TX: u32 = 65;

pub fn init() {
    let pin4_config = (FPIOA_BASE_ADDR + IO_PIN4_OFFSET) as *mut u32;
    let pin5_config = (FPIOA_BASE_ADDR + IP_PIN5_OFFSET) as *mut u32;
    let uart1_rx_config: u32 = FUNC_UART1_RX | 1 << 20 | 1 << 23;
    let uart1_tx_config: u32 = FUNC_UART1_TX | 0xf << 8 | 1 << 12;
    unsafe {
        // bind FUNC_UART1_RX to PAD4
        pin4_config.write_volatile(uart1_rx_config);
        // bind FUNC_UART1_TX to PAD5
        pin5_config.write_volatile(uart1_tx_config);
    }
}