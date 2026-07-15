#ifndef __GBCONST_H
#define __GBCONST_H


// GPIO Control Pins
#define GBX_CLK 0
#define GBX_WR  1
#define GBX_RD  2
#define GBX_CS  3
#define GBX_RST 28

// GPIO Control Masks
#define GBX_CLK_MASK 0x0000'0000'0000'0001
#define GBX_WR_MASK  0x0000'0000'0000'0002
#define GBX_RD_MASK  0x0000'0000'0000'0004
#define GBX_CS_MASK  0x0000'0000'0000'0008
#define GBX_RST_MASK  0x0000'0000'0800'0000
#define GBX_CTL_MASK GBX_CLK_MASK | GBX_WR_MASK | GBX_RD_MASK | GBX_CS_MASK | GBX_RST_MASK

// General Information
#define GBX_HEADER_BYTES 80

// Gameboy Specific Constants
#define GB_ABUS_GPIO_MASK  0x0000'0000'0000'FFFF
#define GB_DBUS_GPIO_MASK  0x0000'0000'00FF'0000
#define GB_DBUS_BITSHIFT   28

// Gameboy Color Specific Constants
#define GBC_ABUS_GPIO_MASK 0x0000'0000'07FF'FFFF
#define GBC_DBUS_GPIO_MASK 0x0000'0000'0000'FFFF


#endif
