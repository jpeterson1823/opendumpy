#ifndef __OPENDUMPY_GB_H
#define __OPENDUMPY_GB_H

#include <stdint.h>

// ##### Gameboy Functions #####

// Read/Write Byte Abstractions
uint8_t gb_get(uint16_t addr);
void    gb_put(uint16_t addr, uint8_t byte);

// Read/Write Byte control functions
void    _gb_dbus_put(uint8_t byte);
void    _gb_abus_put(uint16_t addr);

#endif
