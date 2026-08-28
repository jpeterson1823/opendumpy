#ifndef __GBXLIB_H
#define __GBXLIB_H

#include <stdint.h>
#include <stddef.h>

// Information getters
uint8_t gbx_read_header_into_buffer(uint8_t* header_buffer);
uint8_t gbx_get_title(char* title_buffer);
uint8_t gbx_get_mfgcode(char* mfgcode_buffer);
uint8_t gbx_get_licensee_code(char* lcode_buff);
uint8_t gbx_get_cart_type();
size_t  gbx_get_rom_size();
size_t  gbx_get_ram_size();

// Flag checks
bool gbx_is_japanese();
bool gbx_supports_sgb();
bool gbx_supports_cgb();

#endif
