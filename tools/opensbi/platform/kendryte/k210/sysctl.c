/*
 * SPDX-License-Identifier:    Apache-2.0
 *
 * Copyright 2018 Canaan Inc.
 * Copyright (c) 2019 Western Digital Corporation or its affiliates.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
#include <sbi/riscv_encoding.h>
#include <sbi/sbi_timer.h>
#include "sysctl.h"

volatile sysctl_t *const sysctl = (volatile sysctl_t *)SYSCTL_BASE_ADDR;

#define SYSCTRL_CLOCK_FREQ_IN0 (26000000UL)

static u32 sysctl_pll0_get_freq(void)
{
	u32 freq_in, nr, nf, od;

	freq_in = SYSCTRL_CLOCK_FREQ_IN0;
	nr	= sysctl->pll0.clkr0 + 1;
	nf	= sysctl->pll0.clkf0 + 1;
	od	= sysctl->pll0.clkod0 + 1;

	/*
	 * Get final PLL output freq
	 * FOUT = FIN / NR * NF / OD
	 *      = (FIN * NF) / (NR * OD)
	 */
	return ((u64)freq_in * (u64)nf) / ((u64)nr * (u64)od);
}

u32 sysctl_get_cpu_freq(void)
{
	int clock_source;

	clock_source = (int)sysctl->clk_sel0.aclk_sel;
	switch (clock_source) {
	case 0:
		return SYSCTRL_CLOCK_FREQ_IN0;
	case 1:
		return sysctl_pll0_get_freq() /
		       (2ULL << (int)sysctl->clk_sel0.aclk_divider_sel);
	default:
		return 0;
	}
}

int sysctl_clock_bus_en(sysctl_clock_t clock, uint8_t en)
{
	/*
	 * The timer is under APB0, to prevent apb0_clk_en1 and apb0_clk_en0
	 * on same register, we split it to peripheral and central two
	 * registers, to protect CPU close apb0 clock accidentally.
	 *
	 * The apb0_clk_en0 and apb0_clk_en1 have same function,
	 * one of them set, the APB0 clock enable.
	 */

	/* The APB clock should carefully disable */
	if(en)
	{
		switch(clock)
		{
			/*
			 * These peripheral devices are under APB0
			 * GPIO, UART1, UART2, UART3, SPI_SLAVE, I2S0, I2S1,
			 * I2S2, I2C0, I2C1, I2C2, FPIOA, SHA256, TIMER0,
			 * TIMER1, TIMER2
			 */
		case SYSCTL_CLOCK_GPIO:
		case SYSCTL_CLOCK_SPI2:
		case SYSCTL_CLOCK_I2S0:
		case SYSCTL_CLOCK_I2S1:
		case SYSCTL_CLOCK_I2S2:
		case SYSCTL_CLOCK_I2C0:
		case SYSCTL_CLOCK_I2C1:
		case SYSCTL_CLOCK_I2C2:
		case SYSCTL_CLOCK_UART1:
		case SYSCTL_CLOCK_UART2:
		case SYSCTL_CLOCK_UART3:
		case SYSCTL_CLOCK_FPIOA:
		case SYSCTL_CLOCK_TIMER0:
		case SYSCTL_CLOCK_TIMER1:
		case SYSCTL_CLOCK_TIMER2:
		case SYSCTL_CLOCK_SHA:
			sysctl->clk_en_cent.apb0_clk_en = en;
			break;

			/*
			 * These peripheral devices are under APB1
			 * WDT, AES, OTP, DVP, SYSCTL
			 */
		case SYSCTL_CLOCK_AES:
		case SYSCTL_CLOCK_WDT0:
		case SYSCTL_CLOCK_WDT1:
		case SYSCTL_CLOCK_OTP:
		case SYSCTL_CLOCK_RTC:
			sysctl->clk_en_cent.apb1_clk_en = en;
			break;

			/*
			 * These peripheral devices are under APB2
			 * SPI0, SPI1
			 */
		case SYSCTL_CLOCK_SPI0:
		case SYSCTL_CLOCK_SPI1:
			sysctl->clk_en_cent.apb2_clk_en = en;
			break;

		default:
			break;
		}
	}

	return 0;
}

int sysctl_clock_device_en(sysctl_clock_t clock, uint8_t en)
{
	switch(clock)
	{
		/*
		 * These devices are PLL
		 */
	case SYSCTL_CLOCK_PLL0:
		sysctl->pll0.pll_out_en0 = en;
		break;
	case SYSCTL_CLOCK_PLL1:
		sysctl->pll1.pll_out_en1 = en;
		break;
	case SYSCTL_CLOCK_PLL2:
		sysctl->pll2.pll_out_en2 = en;
		break;

		/*
		 * These devices are CPU, SRAM, APB bus, ROM, DMA, AI
		 */
	case SYSCTL_CLOCK_CPU:
		sysctl->clk_en_cent.cpu_clk_en = en;
		break;
	case SYSCTL_CLOCK_SRAM0:
		sysctl->clk_en_cent.sram0_clk_en = en;
		break;
	case SYSCTL_CLOCK_SRAM1:
		sysctl->clk_en_cent.sram1_clk_en = en;
		break;
	case SYSCTL_CLOCK_APB0:
		sysctl->clk_en_cent.apb0_clk_en = en;
		break;
	case SYSCTL_CLOCK_APB1:
		sysctl->clk_en_cent.apb1_clk_en = en;
		break;
	case SYSCTL_CLOCK_APB2:
		sysctl->clk_en_cent.apb2_clk_en = en;
		break;
	case SYSCTL_CLOCK_ROM:
		sysctl->clk_en_peri.rom_clk_en = en;
		break;
	case SYSCTL_CLOCK_DMA:
		sysctl->clk_en_peri.dma_clk_en = en;
		break;
	case SYSCTL_CLOCK_AI:
		sysctl->clk_en_peri.ai_clk_en = en;
		break;
	case SYSCTL_CLOCK_DVP:
		sysctl->clk_en_peri.dvp_clk_en = en;
		break;
	case SYSCTL_CLOCK_FFT:
		sysctl->clk_en_peri.fft_clk_en = en;
		break;
	case SYSCTL_CLOCK_SPI3:
		sysctl->clk_en_peri.spi3_clk_en = en;
		break;

		/*
		 * These peripheral devices are under APB0
		 * GPIO, UART1, UART2, UART3, SPI_SLAVE, I2S0, I2S1,
		 * I2S2, I2C0, I2C1, I2C2, FPIOA, SHA256, TIMER0,
		 * TIMER1, TIMER2
		 */
	case SYSCTL_CLOCK_GPIO:
		sysctl->clk_en_peri.gpio_clk_en = en;
		break;
	case SYSCTL_CLOCK_SPI2:
		sysctl->clk_en_peri.spi2_clk_en = en;
		break;
	case SYSCTL_CLOCK_I2S0:
		sysctl->clk_en_peri.i2s0_clk_en = en;
		break;
	case SYSCTL_CLOCK_I2S1:
		sysctl->clk_en_peri.i2s1_clk_en = en;
		break;
	case SYSCTL_CLOCK_I2S2:
		sysctl->clk_en_peri.i2s2_clk_en = en;
		break;
	case SYSCTL_CLOCK_I2C0:
		sysctl->clk_en_peri.i2c0_clk_en = en;
		break;
	case SYSCTL_CLOCK_I2C1:
		sysctl->clk_en_peri.i2c1_clk_en = en;
		break;
	case SYSCTL_CLOCK_I2C2:
		sysctl->clk_en_peri.i2c2_clk_en = en;
		break;
	case SYSCTL_CLOCK_UART1:
		sysctl->clk_en_peri.uart1_clk_en = en;
		break;
	case SYSCTL_CLOCK_UART2:
		sysctl->clk_en_peri.uart2_clk_en = en;
		break;
	case SYSCTL_CLOCK_UART3:
		sysctl->clk_en_peri.uart3_clk_en = en;
		break;
	case SYSCTL_CLOCK_FPIOA:
		sysctl->clk_en_peri.fpioa_clk_en = en;
		break;
	case SYSCTL_CLOCK_TIMER0:
		sysctl->clk_en_peri.timer0_clk_en = en;
		break;
	case SYSCTL_CLOCK_TIMER1:
		sysctl->clk_en_peri.timer1_clk_en = en;
		break;
	case SYSCTL_CLOCK_TIMER2:
		sysctl->clk_en_peri.timer2_clk_en = en;
		break;
	case SYSCTL_CLOCK_SHA:
		sysctl->clk_en_peri.sha_clk_en = en;
		break;

		/*
		 * These peripheral devices are under APB1
		 * WDT, AES, OTP, DVP, SYSCTL
		 */
	case SYSCTL_CLOCK_AES:
		sysctl->clk_en_peri.aes_clk_en = en;
		break;
	case SYSCTL_CLOCK_WDT0:
		sysctl->clk_en_peri.wdt0_clk_en = en;
		break;
	case SYSCTL_CLOCK_WDT1:
		sysctl->clk_en_peri.wdt1_clk_en = en;
		break;
	case SYSCTL_CLOCK_OTP:
		sysctl->clk_en_peri.otp_clk_en = en;
		break;
	case SYSCTL_CLOCK_RTC:
		sysctl->clk_en_peri.rtc_clk_en = en;
		break;

		/*
		 * These peripheral devices are under APB2
		 * SPI0, SPI1
		 */
	case SYSCTL_CLOCK_SPI0:
		sysctl->clk_en_peri.spi0_clk_en = en;
		break;
	case SYSCTL_CLOCK_SPI1:
		sysctl->clk_en_peri.spi1_clk_en = en;
		break;

	default:
		break;
	}

	return 0;
}

int sysctl_clock_enable(sysctl_clock_t clock)
{
	if(clock >= SYSCTL_CLOCK_MAX)
		return -1;
	sysctl_clock_bus_en(clock, 1);
	sysctl_clock_device_en(clock, 1);
	return 0;
}


static void sysctl_reset_ctl(sysctl_reset_t reset, uint8_t rst_value)
{
	switch(reset)
	{
	case SYSCTL_RESET_SOC:
		sysctl->soft_reset.soft_reset = rst_value;
		break;
	case SYSCTL_RESET_ROM:
		sysctl->peri_reset.rom_reset = rst_value;
		break;
	case SYSCTL_RESET_DMA:
		sysctl->peri_reset.dma_reset = rst_value;
		break;
	case SYSCTL_RESET_AI:
		sysctl->peri_reset.ai_reset = rst_value;
		break;
	case SYSCTL_RESET_DVP:
		sysctl->peri_reset.dvp_reset = rst_value;
		break;
	case SYSCTL_RESET_FFT:
		sysctl->peri_reset.fft_reset = rst_value;
		break;
	case SYSCTL_RESET_GPIO:
		sysctl->peri_reset.gpio_reset = rst_value;
		break;
	case SYSCTL_RESET_SPI0:
		sysctl->peri_reset.spi0_reset = rst_value;
		break;
	case SYSCTL_RESET_SPI1:
		sysctl->peri_reset.spi1_reset = rst_value;
		break;
	case SYSCTL_RESET_SPI2:
		sysctl->peri_reset.spi2_reset = rst_value;
		break;
	case SYSCTL_RESET_SPI3:
		sysctl->peri_reset.spi3_reset = rst_value;
		break;
	case SYSCTL_RESET_I2S0:
		sysctl->peri_reset.i2s0_reset = rst_value;
		break;
	case SYSCTL_RESET_I2S1:
		sysctl->peri_reset.i2s1_reset = rst_value;
		break;
	case SYSCTL_RESET_I2S2:
		sysctl->peri_reset.i2s2_reset = rst_value;
		break;
	case SYSCTL_RESET_I2C0:
		sysctl->peri_reset.i2c0_reset = rst_value;
		break;
	case SYSCTL_RESET_I2C1:
		sysctl->peri_reset.i2c1_reset = rst_value;
		break;
	case SYSCTL_RESET_I2C2:
		sysctl->peri_reset.i2c2_reset = rst_value;
		break;
	case SYSCTL_RESET_UART1:
		sysctl->peri_reset.uart1_reset = rst_value;
		break;
	case SYSCTL_RESET_UART2:
		sysctl->peri_reset.uart2_reset = rst_value;
		break;
	case SYSCTL_RESET_UART3:
		sysctl->peri_reset.uart3_reset = rst_value;
		break;
	case SYSCTL_RESET_AES:
		sysctl->peri_reset.aes_reset = rst_value;
		break;
	case SYSCTL_RESET_FPIOA:
		sysctl->peri_reset.fpioa_reset = rst_value;
		break;
	case SYSCTL_RESET_TIMER0:
		sysctl->peri_reset.timer0_reset = rst_value;
		break;
	case SYSCTL_RESET_TIMER1:
		sysctl->peri_reset.timer1_reset = rst_value;
		break;
	case SYSCTL_RESET_TIMER2:
		sysctl->peri_reset.timer2_reset = rst_value;
		break;
	case SYSCTL_RESET_WDT0:
		sysctl->peri_reset.wdt0_reset = rst_value;
		break;
	case SYSCTL_RESET_WDT1:
		sysctl->peri_reset.wdt1_reset = rst_value;
		break;
	case SYSCTL_RESET_SHA:
		sysctl->peri_reset.sha_reset = rst_value;
		break;
	case SYSCTL_RESET_RTC:
		sysctl->peri_reset.rtc_reset = rst_value;
		break;

	default:
		break;
	}
}

void usleep(unsigned long long usec) {
	unsigned long long future = read_time() + usec * 10;
	while (read_time() < future);
}

void sysctl_reset(sysctl_reset_t reset)
{
	sysctl_reset_ctl(reset, 1);
	usleep(10);
	sysctl_reset_ctl(reset, 0);
}
