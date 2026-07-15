#ifndef __GBXCART_H
#define __GBXCART_H

#include <stdint.h>

// Cartridge control flag variable. Use bitmask to check flag status.
extern uint8_t GBXCART_INFO;

// State Machine Constants & Masks
#define GBX_GBC_CART 0x01
#define GBX_RD_EN 0x40
#define GBX_WR_EN 0x60
#define GBX_CS_EN 0x80

// Cartridge Control Functions
void gbxcart_rom_select();
void gbxcart_sram_select();
void gbxcart_pulse_clock();
void gbxcart_cs_set(bool set);
void gbxcart_wr_set(bool set);
void gbxcart_rd_set(bool set);
void gbxcart_rst_set(bool set);
void gbxcart_setup();

// background (private) setup functions
void _gbxcart_gpio_init();
void _gbxcart_read_header(uint8_t** header_buff);
void _gbxcart_determine_type(uint8_t** header_buff);

#endif
