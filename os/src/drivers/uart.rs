#![allow(dead_code)]
#![allow(unreachable_patterns)]

use super::sysctl;
use crate::platform::UART_BASE_ADDR;
// TODO: Support UART2/3
/*
const UART_STRIDE: usize = 0x10000;

enum UARTChannel {
    UART1,
    UART2,
    UART3,
}
 */

/** Receive FIFO trigger */
const UART_RECEIVE_FIFO_1: u32 = 0;
const UART_RECEIVE_FIFO_4: u32 = 1;
const UART_RECEIVE_FIFO_8: u32 = 2;
const UART_RECEIVE_FIFO_14: u32 = 3;

/** Send FIFO trigger */
const UART_SEND_FIFO_0: u32 = 0;
const UART_SEND_FIFO_2: u32 = 1;
const UART_SEND_FIFO_4: u32 = 2;
const UART_SEND_FIFO_8: u32 = 3;

enum UARTRegister {
    RBR,
    DLL,
    THR,
    DLH,
    IER,
    FCR,
    IIR,
    LCR,
    LSR,
    DLF,
}

impl UARTRegister {
    fn get_offset(&self) -> usize {
        match self {
            Self::RBR | Self::DLL | Self::THR => 0x0,
            Self::DLH | Self::IER => 0x4,
            Self::FCR | Self::IIR => 0x8,
            Self::LCR => 0x0c,
            Self::LSR => 0x14,
            Self::DLF => 0xc0,
            _ => {
                panic!("unsupported uart register!");
            }
        }
    }

    fn get_ptr(&self) -> *mut u32 {
        (UART_BASE_ADDR + self.get_offset()) as *mut u32
    }

    pub unsafe fn read(&self) -> u32 {
        self.get_ptr().read_volatile()
    }

    pub unsafe fn write(&self, value: u32) {
        self.get_ptr().write_volatile(value)
    }
}

pub fn init(baud_rate: u32) {
    let data_width = 8;
    let stopbit_val = 0;
    let parity_val = 0;
    let divisor = 195000000u32 / baud_rate;
    let dlh = ((divisor >> 12) & 0xff) as u8;
    let dll = ((divisor >> 4) & 0xff) as u8;
    let dlf = (divisor & 0xf) as u8;
    println!("divisor = {}, dlh = {}, dll = {}, dlf = {}", divisor, dlh, dll, dlf);

    sysctl::sysctl_clock_enable(sysctl::SYSCTLClock::SYSCTLClockUART1);
    sysctl::sysctl_reset(sysctl::SYSCTLReset::SYSCTLResetUART1);

    unsafe {
        let lcr_reg = UARTRegister::LCR;
        lcr_reg.write(lcr_reg.read() | (1 << 7));
        let dlh_reg = UARTRegister::DLH;
        let dll_reg = UARTRegister::DLL;
        let dlf_reg = UARTRegister::DLF;
        dlh_reg.write(dlh as u32);
        dll_reg.write(dll as u32);
        dlf_reg.write(dlf as u32);
        lcr_reg.write(0u32);
        lcr_reg.write((data_width - 5) | (stopbit_val << 2) | (parity_val << 3));
        let ier_reg = UARTRegister::IER;
        ier_reg.write(0x81);
        let fcr_reg = UARTRegister::FCR;
        fcr_reg.write(UART_RECEIVE_FIFO_4 << 6 | UART_SEND_FIFO_8 << 4 | 0x1 << 3 | 0x1);
    }
}

pub unsafe fn gpuart_putchar(c: char) {
    while UARTRegister::LSR.read() & (1 << 5) > 0 {
        continue;
    }
    UARTRegister::THR.write(c as u32);
}
