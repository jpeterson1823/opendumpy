#ifndef __OPENDUMPY_GB_H
#define __OPENDUMPY_GB_H

#include <stdint.h>

// General Information
#define GBX_HEADER_SIZE    80
#define GBC_ABUS_GPIO_MASK 0x0000'0000'07FF'FFFF
#define GBC_DBUS_GPIO_MASK 0x0000'0000'0000'FFFF
#define GB_ABUS_GPIO_MASK  0x0000'0000'0000'FFFF
#define GB_DBUS_GPIO_MASK  0x0000'0000'00FF'0000

// Cartridge control flag variable. Use bitmask to check flag status.
extern uint8_t CART_FLAGS;
#define GBC_CART  0x01
#define ROM_CS_EN 0x80

// Cartridge Control Functions
void cart_rom_select();
void cart_sram_select();
void cart_pulse_clock();
void cart_setup();
void _cart_gpio_init();
void _cart_read_header(uint8_t** header_buff);
void _cart_determine_type(uint8_t** header_buff);

// ##### Gameboy Functions #####
// Read/Write Byte Abstractions
uint8_t gb_get(uint16_t addr);
void    gb_put(uint16_t addr, uint8_t byte);
// Read/Write Byte control functions
uint8_t _gb_dbus_get();
uint8_t _gb_dbus_put();
void    _gb_abus_put(uint16_t addr);


// ##### Gameboy Color Functions #####
// Read/Write Byte Abstractions
uint16_t gbc_get(uint32_t addr);
void     gbc_put(uint32_t addr, uint16_t data);
// Read/Write Byte control functions
uint16_t _gbc_dbus_get();
void     _gbc_dbus_put(uint16_t data);
void     _gbc_abus_put(uint32_t addr);
// GPIO control functions
void _gbc_abus_select();
void _gbc_dbus_select();




#endif
