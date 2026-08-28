#ifndef __GBXCART_H
#define __GBXCART_H

#include <stdint.h>

// Cartridge control flag variable. Use bitmask to check flag status.
extern uint8_t gbx_cart_info;

// State Machine Constants & Masks
#define GBX_SM_GBC_CART 0x01
#define GBX_SM_RD_EN 0x40
#define GBX_SM_WR_EN 0x60
#define GBX_SM_CS_EN 0x80

// Cartridge Control Functions
void gbxcart_rom_select();
void gbxcart_sram_select();
void gbxcart_pulse_clock();
void gbxcart_cs_set(bool set);
void gbxcart_wr_set(bool set);
void gbxcart_rd_set(bool set);
void gbxcart_rst_set(bool set);
void gbxcart_setup();

bool gbx_is_gb();
bool gbx_is_gbc();

// background (private) setup functions
void _gbxcart_gpio_setup();
void _gbxcart_read_header(uint8_t** header_buff);
void _gbxcart_determine_type(uint8_t** header_buff);

#endif
