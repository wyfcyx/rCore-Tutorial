//
// Created by shinbokuow on 2020/7/29.
//

#ifndef OPENSBI_UART_H
#define OPENSBI_UART_H

#include <stdint.h>
#include <stddef.h>
#include <stdio.h>
#include "platform.h"

typedef struct _urt
{
	union
	{
		volatile uint32_t RBR;
		volatile uint32_t DLL;
		volatile uint32_t THR;
	};

	union
	{
		volatile uint32_t DLH;
		volatile uint32_t IER;
	};

	union
	{
		volatile uint32_t FCR;
		volatile uint32_t IIR;
	};

	volatile uint32_t LCR;
	volatile uint32_t MCR;
	volatile uint32_t LSR;
	volatile uint32_t MSR;

	volatile uint32_t SCR;
	volatile uint32_t LPDLL;
	volatile uint32_t LPDLH;

	volatile uint32_t reserved1[2];

	union
	{
		volatile uint32_t SRBR[16];
		volatile uint32_t STHR[16];
	};

	volatile uint32_t FAR;
	volatile uint32_t TFR;
	volatile uint32_t RFW;
	volatile uint32_t USR;
	volatile uint32_t TFL;
	volatile uint32_t RFL;
	volatile uint32_t SRR;
	volatile uint32_t SRTS;
	volatile uint32_t SBCR;
	volatile uint32_t SDMAM;
	volatile uint32_t SFE;
	volatile uint32_t SRT;
	volatile uint32_t STET;
	volatile uint32_t HTX;
	volatile uint32_t DMASA;
	volatile uint32_t TCR;
	volatile uint32_t DE_EN;
	volatile uint32_t RE_EN;
	volatile uint32_t DET;
	volatile uint32_t TAT;
	volatile uint32_t DLF;
	volatile uint32_t RAR;
	volatile uint32_t TAR;
	volatile uint32_t LCR_EXT;
	volatile uint32_t reserved2[9];
	volatile uint32_t CPR;
	volatile uint32_t UCV;
	volatile uint32_t CTR;
} uart_t;

typedef enum _uart_device_number
{
	UART_DEVICE_1,
	UART_DEVICE_2,
	UART_DEVICE_3,
	UART_DEVICE_MAX,
} uart_device_number_t;

typedef enum _uart_bitwidth
{
	UART_BITWIDTH_5BIT = 5,
	UART_BITWIDTH_6BIT,
	UART_BITWIDTH_7BIT,
	UART_BITWIDTH_8BIT,
} uart_bitwidth_t;

typedef enum _uart_stopbit
{
	UART_STOP_1,
	UART_STOP_1_5,
	UART_STOP_2
} uart_stopbit_t;

typedef enum _uart_rede_sel
{
	DISABLE = 0,
	ENABLE,
} uart_rede_sel_t;

typedef enum _uart_parity
{
	UART_PARITY_NONE,
	UART_PARITY_ODD,
	UART_PARITY_EVEN
} uart_parity_t;

typedef enum _uart_interrupt_mode
{
	UART_SEND = 1,
	UART_RECEIVE = 2,
} uart_interrupt_mode_t;

typedef enum _uart_send_trigger
{
	UART_SEND_FIFO_0,
	UART_SEND_FIFO_2,
	UART_SEND_FIFO_4,
	UART_SEND_FIFO_8,
} uart_send_trigger_t;

typedef enum _uart_receive_trigger
{
	UART_RECEIVE_FIFO_1,
	UART_RECEIVE_FIFO_4,
	UART_RECEIVE_FIFO_8,
	UART_RECEIVE_FIFO_14,
} uart_receive_trigger_t;

/**
 * @brief       Init uart
 *
 * @param[in]   channel     Uart index
 *
 */
void uart_init(uart_device_number_t channel);

/**
 * @brief       Set uart param
 *
 * @param[in]   channel         Uart index
 * @param[in]   baud_rate       Baudrate
 * @param[in]   data_width      Data width
 * @param[in]   stopbit         Stop bit
 * @param[in]   parity          Odd Even parity
 *
 */
void uart_configure(uart_device_number_t channel, uint32_t baud_rate, uart_bitwidth_t data_width, uart_stopbit_t stopbit, uart_parity_t parity);

/**
 * @brief       Send data from uart
 *
 * @param[in]   channel     Uart index
 * @param[in]   buffer      The data be transfer
 * @param[in]   len         The data length
 *
 * @return      Transfer length
 */
int uart_send_data(uart_device_number_t channel, const char *buffer, size_t buf_len);

/**
 * @brief       Read data from uart
 *
 * @param[in]   channel     Uart index
 * @param[in]   buffer      The Data received
 * @param[in]   len         Receive length
 *
 * @return      Receive length
 */
int uart_receive_data(uart_device_number_t channel, char *buffer, size_t buf_len);

/**
 * @brief       Set receive interrupt threshold
 *
 * @param[in]   channel             Uart index
 * @param[in]   trigger             Threshold of receive interrupt
 *
 */
void uart_set_receive_trigger(uart_device_number_t channel, uart_receive_trigger_t trigger);

int uart_channel_getc(uart_device_number_t channel);
int uart_channel_putc(char c, uart_device_number_t channel);

#endif //OPENSBI_UART_H
