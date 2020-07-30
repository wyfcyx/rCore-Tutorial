#![allow(dead_code)]

use crate::platform::PLIC_BASE_ADDR;
const PLIC_PRIORITY_OFFSET: usize = 0x0;
const PLIC_PRIORITY_STRIDE: usize = 0x4;
const PLIC_IP_OFFSET: usize = 0x1000;
const PLIC_IE_OFFSET: usize = 0x2000;
const PLIC_IE_STRIDE_PER_TARGET: usize = 0x80;
const PLIC_THRESHOLD_OFFSET: usize = 0x20_0000;
const PLIC_THRESHOLD_STRIDE_PER_TARGET: usize = 0x1000;
const PLIC_CLAIM_COMPLETE_OFFSET: usize = 0x20_0004;

pub enum PLICIntrSource {
    NoIntr = 0,
    SPI0Intr = 1,
    SPI1Intr = 2,
    SPISlaveIntr = 3,
    SPI3Intr = 4,
    I2S0Intr = 5,
    I2S1Intr = 6,
    I2S2Intr = 7,
    I2C0Intr = 8,
    I2C1Intr = 9,
    I2C2Intr = 10,
    UART1Intr = 11,
    UART2Intr = 12,
    UART3Intr = 13,
    Timer0AIntr = 14,
    Timer0BIntr = 15,
    Timer1AIntr = 16,
    Timer1BIntr = 17,
    Timer2AIntr = 18,
    Timer2BIntr = 19,
    RTCIntr = 20,
    WDT0Intr = 21,
    WDT1Intr = 22,
    APBGPIOIntr = 23,
    DVPIntr = 24,
    AIIntr = 25,
    FFTIntr = 26,
    DMA0Intr = 27,
    DMA1Intr = 28,
    DMA2Intr = 29,
    DMA3Intr = 30,
    DMA4Intr = 31,
    DMA5Intr = 32,
    UARTHSIntr = 33,
    GPIO0Intr = 34,
    GPIO1Intr = 35,
    GPIO2Intr = 36,
    GPIO3Intr = 37,
    GPIO4Intr = 38,
    GPIO5Intr = 39,
    GPIO6Intr = 40,
    GPIO7Intr = 41,
    GPIO8Intr = 42,
    GPIO9Intr = 43,
    GPIO10Intr = 44,
    GPIO11Intr = 45,
    GPIO12Intr = 46,
    GPIO13Intr = 47,
    GPIO14Intr = 48,
    GPIO15Intr = 49,
    GPIO16Intr = 50,
    GPIO17Intr = 51,
    GPIO18Intr = 52,
    GPIO19Intr = 53,
    GPIO20Intr = 54,
    GPIO21Intr = 55,
    GPIO22Intr = 56,
    GPIO23Intr = 57,
    GPIO24Intr = 58,
    GPIO25Intr = 59,
    GPIO26Intr = 60,
    GPIO27Intr = 61,
    GPIO28Intr = 62,
    GPIO29Intr = 63,
    GPIO30Intr = 64,
    GPIO31Intr = 65,
}

pub enum PLICTarget {
    Hart0MIntr,
    Hart0SIntr,
    Hart1MIntr,
    Hart1SIntr,
}

fn get_target_threshold_ptr(target: PLICTarget) -> *mut u32 {
    let mut addr = PLIC_BASE_ADDR;
    addr += PLIC_THRESHOLD_OFFSET;
    addr += PLIC_THRESHOLD_STRIDE_PER_TARGET * (target as usize);
    addr as *mut u32
}

pub fn get_target_threshold(target: PLICTarget) -> u32 {
    let ptr = get_target_threshold_ptr(target);
    unsafe { ptr.read_volatile() }
}

pub fn set_target_threshold(target: PLICTarget, threshold: u32) {
    let ptr = get_target_threshold_ptr(target);
    unsafe { ptr.write_volatile(threshold) }
}

fn get_intr_source_priority_ptr(source: PLICIntrSource) -> *mut u32 {
    let mut addr = PLIC_BASE_ADDR;
    addr += PLIC_PRIORITY_OFFSET;
    addr += PLIC_PRIORITY_STRIDE * (source as usize);
    addr as *mut u32
}

pub fn get_intr_source_priority(source: PLICIntrSource) -> u32 {
    let ptr = get_intr_source_priority_ptr(source);
    unsafe { ptr.read_volatile() }
}

pub fn set_intr_source_priority(source: PLICIntrSource, priority: u32) {
    let ptr = get_intr_source_priority_ptr(source);
    unsafe { ptr.write_volatile(priority) }
}

pub fn init() {
    let hart0_m_threshold = get_target_threshold_ptr(PLICTarget::Hart0MIntr);
    let hart1_m_threshold = get_target_threshold_ptr(PLICTarget::Hart1MIntr);
    unsafe {
        hart0_m_threshold.write_volatile(0);
        hart1_m_threshold.write_volatile(1);
    }
}