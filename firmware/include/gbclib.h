#ifndef __GBCLIB_H
#define __GBCLIB_H

#include <stdint.h>

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
