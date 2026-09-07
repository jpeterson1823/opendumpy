#include "opendumpy/gbxlib.h"
#include "opendumpy/gbconst.h"
#include "opendumpy/gblib.h"

/**
 * @brief Reads the cartridge header into the provided buffer.
 * Assumes the buffer size is GBX_HEADER_SIZE.
 *
 * @param header_buffer Buffer to write header bytes into.
 * @return Number of bytes read.
 */
uint8_t gbx_read_header_into_buffer(uint8_t* header_buffer) {
    uint16_t addr = GBX_HEADER_ADDR;
    for (; addr - GBX_HEADER_ADDR <= GBX_HEADER_SIZE; addr++)
        header_buffer[addr - GBX_HEADER_SIZE] = gb_get(addr);
    return (uint8_t)(addr - GBX_HEADER_ADDR);
}


/**
 * @brief Reads the title from cartridge header into the provided buffer.
 * Assumes the buffer size is GBX_HEADER_GAME_TITLE_SIZE.
 *
 * @param title_buffer Buffer to write title bytes into.
 * @return Number of bytes read.
 */
uint8_t gbx_get_title(char* title_buffer) {
    uint16_t addr = GBX_HEADER_GAME_TITLE_ADDR;
    for(; addr - GBX_HEADER_GAME_TITLE_ADDR <= GBX_HEADER_GAME_TITLE_SIZE; addr++)
        title_buffer[addr - GBX_HEADER_GAME_TITLE_SIZE] = gb_get(addr);
    return (uint8_t)(addr - GBX_HEADER_ADDR);
}

/**
 * @brief Reads manufacture code from cartridge header into the provided buffer.
 * Assumes the buffer size is GBX_HEADER_MFG_CODE_SIZE
 *
 * @parm mfgcode_buffer Buffer to write manufacture code into.
 * @return Number of bytes read.
 */
uint8_t gbx_get_mfgcode(char* mfgcode_buffer) {
    uint16_t addr = GBX_HEADER_MFG_CODE_ADDR;
    for (; addr - GBX_HEADER_MFG_CODE_ADDR <= GBX_HEADER_MFG_CODE_SIZE; addr++)
        mfgcode_buffer[addr - GBX_HEADER_MFG_CODE_ADDR] = gb_get(addr);
    return (uint8_t)(addr - GBX_HEADER_MFG_CODE_ADDR);
}

/**
 * @brief Reads the licensee code from cartridge header into the provided buffer.
 * Assumes the buffer size is GBX_HEADER_NEW_LICENSEE_SIZE
 *
 * @return Number of bytes read.
 */
uint8_t gbx_get_licensee_code(char* lcode_buff) {
    // read old licensee addr
    uint8_t byte = gb_get(GBX_HEADER_OLD_LICENSEE_ADDR);

    // If licensee code is 0x33, read new licensee code and return
    if (byte == 0x33) {
        lcode_buff[0] = gb_get(GBX_HEADER_NEW_LICENSEE_ADDR);
        lcode_buff[1] = gb_get(GBX_HEADER_NEW_LICENSEE_ADDR + 1);
        return 2;
    }

    // Otherwise, addd byte to lcode_buff and return
    else {
        lcode_buff[0] = byte;
        lcode_buff[1] = 0x00;
        return 1;
    }
}

/**
 * @brief Reads the cartridge type from the cartridge header.
 * @return Cartridge type byte from header.
 */
uint8_t gbx_get_cart_type() {
    return gb_get(GBX_HEADER_CART_TYPE_ADDR);
}

/**
 * @brief Read the ROM size from the cartridge header.
 * Calculated from 32KiB * (1 << value_at_addr)
 * @return Size of ROM in bytes.
 */
size_t gbx_get_rom_size() {
    return 32*1024 * (0x00000001 << gb_get(GBX_HEADER_ROM_SIZE_ADDR));
}

/**
 * @brief Read the RAM size from the cartridge header.
 * Calculated from 32KiB * (1 << value_at_addr)
 *
 * @return Size of RAM in bytes.
 */
size_t gbx_get_ram_size() {
    return 32*1024 * (0x00000001 << gb_get(GBX_HEADER_RAM_SIZE_ADDR));
}

/**
 * @brief Reads the cartridge header and determines the cartridge's destination.
 * @return True if cartridge is Japanese; false otherwise.
 */
bool gbx_is_japanese() {
    return gb_get(GBX_HEADER_DESTCODE_ADDR) == 0x00;
}

/**
 * @brief Reads the cartridge header and determines if the cartridge supports SGB.
 * @return True if cartridge supports SGB; false otherwise.
 */
bool gbx_supports_sgb() {
    return gb_get(GBX_HEADER_SGB_FLAG_ADDR) == 0x03;
}
