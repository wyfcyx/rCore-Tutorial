#![allow(dead_code)]

use bitflags::bitflags;

use crate::platform::SYSCTL_BASE_ADDR;
use crate::interrupt::timer::usleep;

const SYSCTL_CLK_EN_CENT_OFFSET: usize = 0x28;
bitflags! {
    pub struct SYSCTLClockEnableCent: u32 {
        const CPU_CLOCK_ENABLE = 1 << 0;
        const SRAM0_CLOCK_ENABLE = 1 << 1;
        const SRAM1_CLOCK_ENABLE = 1 << 2;
        const APB0_CLOCK_ENABLE = 1 << 3;
        const APB1_CLOCK_ENABLE = 1 << 4;
        const APB2_CLOCK_ENABLE = 1 << 5;
    }
}

impl SYSCTLClockEnableCent {
    fn get_ptr() -> *mut u32 {
        (SYSCTL_BASE_ADDR + SYSCTL_CLK_EN_CENT_OFFSET) as *mut u32
    }
    unsafe fn read() -> u32 {
        Self::get_ptr().read_volatile()
    }
    unsafe fn write(v: u32) {
        Self::get_ptr().write_volatile(v)
    }
    fn set_cpu_clock_enable(enable: bool) {
        let mut flags = Self::from_bits(
            unsafe { Self::read() }
        ).unwrap();
        flags.set(Self::CPU_CLOCK_ENABLE, enable);
        unsafe { Self::write(flags.bits) }
    }
    fn set_sram_clock_enable(sram_id: i32, enable: bool) {
        let mut flags = Self::from_bits(
            unsafe { Self::read() }
        ).unwrap();
        let bit = match sram_id {
            0 => Self::SRAM0_CLOCK_ENABLE,
            1 => Self::SRAM1_CLOCK_ENABLE,
            _ => { panic!("given SRAM id in set_sram_clock_enable() is not supported!") }
        };
        flags.set(bit, enable);
        unsafe { Self::write(flags.bits) }
    }
    fn set_apb_clock_enable(apb_id: i32, enable: bool) {
        let mut flags = Self::from_bits(
            unsafe { Self::read() }
        ).unwrap();
        let bit = match apb_id {
            0 => Self::APB0_CLOCK_ENABLE,
            1 => Self::APB1_CLOCK_ENABLE,
            2 => Self::APB2_CLOCK_ENABLE,
            _ => { panic!("given APB id in set_apb_block_enable() is not supported!") }
        };
        flags.set(bit, enable);
        unsafe { Self::write(flags.bits) }
    }
}

const SYSCTL_CLK_EN_PERI_OFFSET: usize = 0x2c;
bitflags! {
    pub struct SYSCTLClockEnablePeri: u32 {
        const ROM_CLOCK_ENABLE      = 1 << 0;
        const DMA_CLOCK_ENABLE      = 1 << 1;
        const AI_CLOCK_ENABLE       = 1 << 2;
        const DVP_CLOCK_ENABLE      = 1 << 3;
        const FFT_CLOCK_ENABLE      = 1 << 4;
        const GPIO0_CLOCK_ENABLE    = 1 << 5;
        const SPI0_CLOCK_ENABLE     = 1 << 6;
        const SPI1_CLOCK_ENABLE     = 1 << 7;
        const SPI2_CLOCK_ENABLE     = 1 << 8;
        const SPI3_CLOCK_ENABLE     = 1 << 9;
        const I2S0_CLOCK_ENABLE     = 1 << 10;
        const I2S1_CLOCK_ENABLE     = 1 << 11;
        const I2S2_CLOCK_ENABLE     = 1 << 12;
        const I2C0_CLOCK_ENABLE     = 1 << 13;
        const I2C1_CLOCK_ENABLE     = 1 << 14;
        const I2C2_CLOCK_ENABLE     = 1 << 15;
        const UART1_CLOCK_ENABLE    = 1 << 16;
        const UART2_CLOCK_ENABLE    = 1 << 17;
        const UART3_CLOCK_ENABLE    = 1 << 18;
        const AES_CLOCK_ENABLE      = 1 << 19;
        const FPIOA_CLOCK_ENABLE    = 1 << 20;
        const TIMER0_CLOCK_ENABLE   = 1 << 21;
        const TIMER1_CLOCK_ENABLE   = 1 << 22;
        const TIMER2_CLOCK_ENABLE   = 1 << 23;
        const WDT0_CLOCK_ENABLE     = 1 << 24;
        const WDT1_CLOCK_ENABLE     = 1 << 25;
        const SHA_CLOCK_ENABLE      = 1 << 26;
        const OTP_CLOCK_ENABLE      = 1 << 27;
        const RTC_CLOCK_ENABLE      = 1 << 29;
    }
}
impl SYSCTLClockEnablePeri {
    fn get_ptr() -> *mut u32 {
        (SYSCTL_BASE_ADDR + SYSCTL_CLK_EN_PERI_OFFSET) as *mut u32
    }
    unsafe fn read() -> u32 {
        Self::get_ptr().read_volatile()
    }
    unsafe fn write(v: u32) {
        Self::get_ptr().write_volatile(v)
    }
    fn set_uart_clock_enable(uart_id: i32, enable: bool) {
        let mut flags = Self::from_bits(
            unsafe { Self::read() }
        ).unwrap();
        let bit = match uart_id {
            1 => Self::UART1_CLOCK_ENABLE,
            2 => Self::UART2_CLOCK_ENABLE,
            3 => Self::UART3_CLOCK_ENABLE,
            _ => { panic!("invalid uart id!"); }
        };
        flags.set(bit, enable);
        unsafe { Self::write(flags.bits) }
    }
}

const SYSCTL_SOFT_RESET_OFFSET: usize = 0x30;
bitflags! {
    struct SYSCTLSoftReset: u32 {
        const SOFT_RESET = 1 << 0;
    }
}
impl SYSCTLSoftReset {
    fn get_ptr() -> *mut u32 {
        (SYSCTL_BASE_ADDR + SYSCTL_SOFT_RESET_OFFSET) as *mut u32
    }
    unsafe fn read() -> u32 {
        Self::get_ptr().read_volatile()
    }
    unsafe fn write(v: u32) {
        Self::get_ptr().write_volatile(v)
    }
    fn set_soft_reset(value: bool) {
        let mut flags = Self::from_bits( unsafe{ Self::read() }).unwrap();
        flags.set(Self::SOFT_RESET, value);
        unsafe { Self::write(flags.bits) }
    }
}

const SYSCTL_PERI_RESET_OFFSET: usize = 0x34;
bitflags! {
    struct SYSCTLPeriReset: u32 {
        const ROM_RESET     = 1 << 0;
        const DMA_RESET     = 1 << 1;
        const AI_RESET      = 1 << 2;
        const DVP_RESET     = 1 << 3;
        const FFT_RESET     = 1 << 4;
        const GPIO_RESET    = 1 << 5;
        const SPI0_RESET    = 1 << 6;
        const SPI1_RESET    = 1 << 7;
        const SPI2_RESET    = 1 << 8;
        const SPI3_RESET    = 1 << 9;
        const I2S0_RESET    = 1 << 10;
        const I2S1_RESET    = 1 << 11;
        const I2S2_RESET    = 1 << 12;
        const I2C0_RESET    = 1 << 13;
        const I2C1_RESET    = 1 << 14;
        const I2C2_RESET    = 1 << 15;
        const UART1_RESET   = 1 << 16;
        const UART2_RESET   = 1 << 17;
        const UART3_RESET   = 1 << 18;
        const AES_RESET     = 1 << 19;
        const FPIOA_RESET   = 1 << 20;
        const TIMER0_RESET  = 1 << 21;
        const TIMER1_RESET  = 1 << 22;
        const TIMER2_RESET  = 1 << 23;
        const WDT0_RESET    = 1 << 24;
        const WDT1_RESET    = 1 << 25;
        const SHA_RESET     = 1 << 26;
        const RTC_RESET     = 1 << 29;
    }
}
impl SYSCTLPeriReset {
    fn get_ptr() -> *mut u32 {
        (SYSCTL_BASE_ADDR + SYSCTL_PERI_RESET_OFFSET) as *mut u32
    }
    unsafe fn read() -> u32 {
        Self::get_ptr().read_volatile()
    }
    unsafe fn write(v: u32) {
        Self::get_ptr().write_volatile(v)
    }
    fn set_uart_reset(uart_id: i32, value: bool) {
        let mut flags = Self::from_bits(
            unsafe { Self::read() }
        ).unwrap();
        let bit = match uart_id {
            1 => Self::UART1_RESET,
            2 => Self::UART2_RESET,
            3 => Self::UART3_RESET,
            _ => { panic!("invalid uart id!"); }
        };
        flags.set(bit, value);
        unsafe { Self::write(flags.bits) }
    }
}
#[derive(Copy, Clone)]
pub enum SYSCTLClock {
    SYSCTLClockPLL0,
    SYSCTLClockPLL1,
    SYSCTLClockPLL2,
    SYSCTLClockCPU,
    SYSCTLClockSRAM0,
    SYSCTLClockSRAM1,
    SYSCTLClockAPB0,
    SYSCTLClockAPB1,
    SYSCTLClockAPB2,
    SYSCTLClockROM,
    SYSCTLClockDMA,
    SYSCTLClockAI,
    SYSCTLClockDVP,
    SYSCTLClockFFT,
    SYSCTLClockGPI0,
    SYSCTLClockSPI0,
    SYSCTlClockSPI1,
    SYSCTLClockSPI2,
    SYSCTLClockSPI3,
    SYSCTLClockI2S0,
    SYSCTLClockI2S1,
    SYSCTLClockI2S2,
    SYSCTLClockI2C0,
    SYSCTLClockI2C1,
    SYSCTLClockI2C2,
    SYSCTLClockUART1,
    SYSCTLClockUART2,
    SYSCTLClockUART3,
    SYSCTLClockAES,
    SYSCTLClockFPIOA,
    SYSCTLClockTimer0,
    SYSCTLClockTimer1,
    SYSCTLClockTimer2,
    SYSCTLClockWDT0,
    SYSCTLClockWDT1,
    SYSCTLClockSHA,
    SYSCTLClockOTP,
    SYSCTLClockRTC,
    SYSCTLClockACLK = 40,
    SYSCTLClockHCLK,
    SYSCTLClockIn0,
    SYSCTLClockMax,
}

pub fn sysctl_clock_enable(sysctl_clock: SYSCTLClock) {
    if sysctl_clock as i32 >= SYSCTLClock::SYSCTLClockMax as i32 {
        panic!("unknown sysctl clock type!");
    }
    sysctl_clock_bus_en(sysctl_clock, true);
    sysctl_clock_device_en(sysctl_clock, true);
}

fn sysctl_clock_bus_en(sysctl_clock: SYSCTLClock, enable: bool) -> i32 {
    if !enable {
        // we should disable related clock enable carefully
        return 0;
    }
    match sysctl_clock {
        /*
         * These peripheral devices are under APB0
         * GPIO, UART1, UART2, UART3, SPI_SLAVE, I2S0, I2S1,
         * I2S2, I2C0, I2C1, I2C2, FPIOA, SHA256, TIMER0,
         * TIMER1, TIMER2
         */
        SYSCTLClock::SYSCTLClockGPI0 |
        SYSCTLClock::SYSCTLClockSPI2 |
        SYSCTLClock::SYSCTLClockI2S0 |
        SYSCTLClock::SYSCTLClockI2S1 |
        SYSCTLClock::SYSCTLClockI2S2 |
        SYSCTLClock::SYSCTLClockI2C0 |
        SYSCTLClock::SYSCTLClockI2C1 |
        SYSCTLClock::SYSCTLClockI2C2 |
        SYSCTLClock::SYSCTLClockUART1 |
        SYSCTLClock::SYSCTLClockUART2 |
        SYSCTLClock::SYSCTLClockUART3 |
        SYSCTLClock::SYSCTLClockFPIOA |
        SYSCTLClock::SYSCTLClockTimer0 |
        SYSCTLClock::SYSCTLClockTimer1 |
        SYSCTLClock::SYSCTLClockTimer2 |
        SYSCTLClock::SYSCTLClockSHA => {
            SYSCTLClockEnableCent::set_apb_clock_enable(0, true);
        },
        /*
         * These peripheral devices are under APB1
         * WDT, AES, OTP, DVP, SYSCTL
         */
        SYSCTLClock::SYSCTLClockAES |
        SYSCTLClock::SYSCTLClockWDT0 |
        SYSCTLClock::SYSCTLClockWDT1 |
        SYSCTLClock::SYSCTLClockOTP |
        SYSCTLClock::SYSCTLClockRTC => {
            SYSCTLClockEnableCent::set_apb_clock_enable(1, true);
        },
        /*
         * These peripheral devices are under APB2
         * SPI0, SPI1
         */
        SYSCTLClock::SYSCTLClockSPI0 | SYSCTLClock::SYSCTlClockSPI1 => {
            SYSCTLClockEnableCent::set_apb_clock_enable(2, true);
        },
        _ => { panic!("sysctl clock type is not supported!"); }
    }
    0
}

fn sysctl_clock_device_en(sysctl_clock: SYSCTLClock, enable: bool) {
    match sysctl_clock {
        SYSCTLClock::SYSCTLClockUART1 |
        SYSCTLClock::SYSCTLClockUART2 |
        SYSCTLClock::SYSCTLClockUART3 => {
            SYSCTLClockEnablePeri::set_uart_clock_enable(
                sysctl_clock as i32 - SYSCTLClock::SYSCTLClockUART1 as i32 + 1,
                enable
            )
        }
        _ => {
            panic!("we cannot handle this now!")
        }
    }
}

#[derive(Copy, Clone)]
pub enum SYSCTLReset {
    SYSCTLResetSOC,
    SYSCTLResetROM,
    YSCTLResetDMA,
    SYSCTLResetAI,
    SYSCTLResetDVP,
    SYSCTLResetFFT,
    SYSCTLResetGPIO,
    SYSCTLResetSPI0,
    SYSCTLResetSPI1,
    SYSCTLResetSPI2,
    SYSCTLResetSPI3,
    SYSCTLResetI2S0,
    SYSCTLResetI2S1,
    SYSCTLResetI2S2,
    SYSCTLResetI2C0,
    SYSCTLResetI2C1,
    SYSCTLResetI2C2,
    SYSCTLResetUART1,
    SYSCTLResetUART2,
    SYSCTLResetUART3,
    SYSCTLResetAES,
    SYSCTLResetFPIOA,
    SYSCTLResetTimer0,
    SYSCTLResetTimer1,
    SYSCTLResetTimer2,
    SYSCTLResetWDT0,
    SYSCTLResetWDT1,
    SYSCTLResetSHA,
    SYSCTLResetRTC,
    SYSCTLResetMax
}

pub fn sysctl_reset(sysctl_reset: SYSCTLReset) {
    sysctl_reset_ctl(sysctl_reset, true);
    usleep(100);
    sysctl_reset_ctl(sysctl_reset, false);
}

pub fn sysctl_reset_ctl(sysctl_reset: SYSCTLReset, value: bool) {
    match sysctl_reset {
        SYSCTLReset::SYSCTLResetSOC => {
            SYSCTLSoftReset::set_soft_reset(value);
        },
        SYSCTLReset::SYSCTLResetUART1 |
        SYSCTLReset::SYSCTLResetUART2 |
        SYSCTLReset::SYSCTLResetUART3 => {
            SYSCTLPeriReset::set_uart_reset(sysctl_reset as i32 - SYSCTLReset::SYSCTLResetUART1 as i32 + 1, value);
        },
        _ => {
            panic!("unhandled sysctl_reset_ctl!");
        }
    }
}


