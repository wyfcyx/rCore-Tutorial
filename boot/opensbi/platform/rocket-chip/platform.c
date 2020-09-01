/*
 * SPDX-License-Identifier: BSD-2-Clause
 *
 * Copyright (c) 2019 Western Digital Corporation or its affiliates.
 */

#include <sbi/riscv_encoding.h>
#include <sbi/sbi_const.h>
#include <sbi/sbi_hart.h>
#include <sbi/sbi_platform.h>
#include <sbi/sbi_console.h>
#include <sbi/riscv_locks.h>

#include <htif/htif.h>

/*
 * Include these files as needed.
 * See config.mk PLATFORM_xxx configuration parameters.
 */
#include <plat/irqchip/plic.h>
#include <plat/sys/clint.h>

#define PLATFORM_CLINT_ADDR 0x2000000
#define PLATFORM_PLIC_ADDR 0xc000000
#define PLATFORM_HART_COUNT 2
#define PLATFORM_PLIC_NUM_SOURCES 3

#define UART0_BASE_ADDR 0x60000000

volatile uint32_t *uart0_rx_fifo  = (uint32_t *)(UART0_BASE_ADDR + 0x00);
volatile uint32_t *uart0_tx_fifo  = (uint32_t *)(UART0_BASE_ADDR + 0x04);
volatile uint32_t *uart0_stat_reg = (uint32_t *)(UART0_BASE_ADDR + 0x08);
volatile uint32_t *uart0_ctrl_reg = (uint32_t *)(UART0_BASE_ADDR + 0x0C);

volatile uint64_t tohost __attribute__((section(".htif")))   = 0;
volatile uint64_t fromhost __attribute__((section(".htif"))) = 0;
volatile int htif_console_buf;
static spinlock_t io_lock;

static void platform_console_putc(char ch);

#define TOHOST_CMD(dev, cmd, payload)                        \
	(((uint64_t)(dev) << 56) | ((uint64_t)(cmd) << 48) | \
	 (uint64_t)(payload))

static void __check_fromhost()
{
	uint64_t fh = fromhost;
	if (!fh)
		return;
	fromhost = 0;

	// this should be from the console
	htif_assert(FROMHOST_DEV(fh) == 1);
	switch (FROMHOST_CMD(fh)) {
	case 0:
		htif_console_buf = 1 + (uint8_t)FROMHOST_DATA(fh);
		break;
	case 1:
		break;
	default:
		break;
		htif_assert(0);
	}
}

static void __set_tohost(u64 dev, u64 cmd, u64 data)
{
	while (tohost)
		__check_fromhost();
	tohost = TOHOST_CMD(dev, cmd, data);
}

/*
 * Platform early initialization.
 */
static int platform_early_init(bool cold_boot)
{
	SPIN_LOCK_INIT(&io_lock);
	sbi_printf("init");
	return 0;
}

/*
 * Platform final initialization.
 */
static int platform_final_init(bool cold_boot)
{
	void *fdt;

	if (!cold_boot)
		return 0;

	fdt = sbi_scratch_thishart_arg1_ptr();
	plic_fdt_fixup(fdt, "riscv,plic0");
	return 0;
}

/*
 * Get number of PMP regions for given HART.
 */
static u32 platform_pmp_region_count(u32 hartid)
{
	return 1;
}

/*
 * Get PMP regions details (namely: protection, base address, and size) for
 * a given HART.
 */
static int platform_pmp_region_info(u32 hartid, u32 index, ulong *prot,
				    ulong *addr, ulong *log2size)
{
	int ret = 0;

	switch (index) {
	case 0:
		*prot     = PMP_R | PMP_W | PMP_X;
		*addr     = 0;
		*log2size = __riscv_xlen;
		break;
	default:
		ret = -1;
		break;
	};

	return ret;
}

/*
 * Initialize the platform console.
 */
static int platform_console_init(void)
{
	// 0b10011 = Enable Intr | Clear tx/rx FIFO
	*uart0_ctrl_reg = 0x13;
	return 0;
}

/*
 * Write a character to the platform console output.
 */
static void platform_console_putc(char ch)
{
	// putchar can block
	// write to both htif and uart
	spin_lock(&io_lock);
	__set_tohost(1, 1, ch);
	// because tx fifo full generates an interrupts
	// we don't want it, so we just wait until tx fifo empty
	while (((*uart0_stat_reg) & (1 << 2)) == 0)
		;
	*uart0_tx_fifo = (uint32_t)ch;
	spin_unlock(&io_lock);
}

/*
 * Read a character from the platform console input.
 */
static int platform_console_getc(void)
{
	// getchar should return -1 on no data
	// read from uart
	spin_lock(&io_lock);
	int ch;
	if ((*uart0_stat_reg) & (1 << 0)) {
		ch = *uart0_rx_fifo;
	} else {
		// rx fifo empty
		ch = -1;
	}
	spin_unlock(&io_lock);
	return ch;
}

/*
 * Initialize the platform interrupt controller for current HART.
 */
static int platform_irqchip_init(bool cold_boot)
{
	u32 hartid = sbi_current_hartid();
	int ret;

	/* Example if the generic PLIC driver is used */
	if (cold_boot) {
		ret = plic_cold_irqchip_init(PLATFORM_PLIC_ADDR,
					     PLATFORM_PLIC_NUM_SOURCES,
					     PLATFORM_HART_COUNT);
		if (ret)
			return ret;
	}

	return plic_warm_irqchip_init(hartid, 2 * hartid, 2 * hartid + 1);
}

/*
 * Initialize IPI for current HART.
 */
static int platform_ipi_init(bool cold_boot)
{
	int ret;

	/* Example if the generic CLINT driver is used */
	if (cold_boot) {
		ret = clint_cold_ipi_init(PLATFORM_CLINT_ADDR,
					  PLATFORM_HART_COUNT);
		if (ret)
			return ret;
	}

	return clint_warm_ipi_init();
}

/*
 * Send IPI to a target HART
 */
static void platform_ipi_send(u32 target_hart)
{
	/* Example if the generic CLINT driver is used */
	clint_ipi_send(target_hart);
}

/*
 * Wait for target HART to acknowledge IPI.
 */
static void platform_ipi_sync(u32 target_hart)
{
	/* Example if the generic CLINT driver is used */
	clint_ipi_sync(target_hart);
}

/*
 * Clear IPI for a target HART.
 */
static void platform_ipi_clear(u32 target_hart)
{
	/* Example if the generic CLINT driver is used */
	clint_ipi_clear(target_hart);
}

/*
 * Initialize platform timer for current HART.
 */
static int platform_timer_init(bool cold_boot)
{
	int ret;

	/* Example if the generic CLINT driver is used */
	if (cold_boot) {
		ret = clint_cold_timer_init(PLATFORM_CLINT_ADDR,
					    PLATFORM_HART_COUNT);
		if (ret)
			return ret;
	}

	return clint_warm_timer_init();
}

/*
 * Get platform timer value.
 */
static u64 platform_timer_value(void)
{
	/* Example if the generic CLINT driver is used */
	return clint_timer_value();
}

/*
 * Start platform timer event for current HART.
 */
static void platform_timer_event_start(u64 next_event)
{
	/* Example if the generic CLINT driver is used */
	clint_timer_event_start(next_event);
}

/*
 * Stop platform timer event for current HART.
 */
static void platform_timer_event_stop(void)
{
	/* Example if the generic CLINT driver is used */
	clint_timer_event_stop();
}

/*
 * Reboot the platform.
 */
static int platform_system_reboot(u32 type)
{
	sbi_printf("System reboot\n");
	return 0;
}

/*
 * Shutdown or poweroff the platform.
 */
static int platform_system_shutdown(u32 type)
{
	sbi_printf("System shutdown\n");
	return 0;
}

/*
 * Platform descriptor.
 */
const struct sbi_platform platform = {

	.name		    = "Rocket Chip",
	.features	   = SBI_PLATFORM_DEFAULT_FEATURES,
	.hart_count	 = PLATFORM_HART_COUNT,
	.hart_stack_size    = 4096,
	.disabled_hart_mask = 0,

	.early_init = platform_early_init,
	.final_init = platform_final_init,

	.pmp_region_count = platform_pmp_region_count,
	.pmp_region_info  = platform_pmp_region_info,

	.console_init = platform_console_init,
	.console_putc = platform_console_putc,
	.console_getc = platform_console_getc,

	.irqchip_init = platform_irqchip_init,
	.ipi_init     = platform_ipi_init,
	.ipi_send     = platform_ipi_send,
	.ipi_sync     = platform_ipi_sync,
	.ipi_clear    = platform_ipi_clear,

	.timer_init	= platform_timer_init,
	.timer_value       = platform_timer_value,
	.timer_event_start = platform_timer_event_start,
	.timer_event_stop  = platform_timer_event_stop,

	.system_reboot   = platform_system_reboot,
	.system_shutdown = platform_system_shutdown
};
