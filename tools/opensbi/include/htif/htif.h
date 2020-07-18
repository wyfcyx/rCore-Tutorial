// See LICENSE for license details.

#ifndef _RISCV_HTIF_H
#define _RISCV_HTIF_H

#include <sbi/sbi_console.h>
#include <stdint.h>

#if __riscv_xlen == 64
# define TOHOST_CMD(dev, cmd, payload) \
  (((uint64_t)(dev) << 56) | ((uint64_t)(cmd) << 48) | (uint64_t)(payload))
#else
# define TOHOST_CMD(dev, cmd, payload) ({ \
  if ((dev) || (cmd)) __builtin_trap(); \
  (payload); })
#endif
#define FROMHOST_DEV(fromhost_value) ((uint64_t)(fromhost_value) >> 56)
#define FROMHOST_CMD(fromhost_value) ((uint64_t)(fromhost_value) << 8 >> 56)
#define FROMHOST_DATA(fromhost_value) ((uint64_t)(fromhost_value) << 16 >> 16)

#define htif_assert(x) ({ \
    if (!(x)) { \
      char assert_what[100]; \
      int num = sbi_sprintf(assert_what, "htif_assert failed in %s: %d", \
                            __FILE__, __LINE__); \
      for (int i = 0; i < num; i++) \
        platform_console_putc(assert_what[i]); \
    } })

//void query_htif(uintptr_t dtb);
//void htif_console_putchar(uint8_t);
//int htif_console_getchar();
//void htif_poweroff() __attribute__((noreturn));
//void htif_syscall(uintptr_t);

#endif
