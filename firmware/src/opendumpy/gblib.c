#include "opendumpy/gblib.h"
#include "opendumpy/gbconst.h"
#include "opendumpy/gbxcart.h"
#include "hardware/gpio.h"

/**
 * @brief Reads the current value output on the Gameboy Data Bus
 * @param addr 2-byte address to be read
 * @return Read byte from dbus
 */
uint8_t gb_get(uint16_t addr) {
    // put addr out on abus and pulse clock
    _gb_abus_put(addr);
    gbxcart_pulse_clock();

    // read & return new data on dbus
    return (uint8_t)(gpio_get_all() >> GB_GPIO_DBUS_BITSHIFT);
}

/**
 * @brief Writes byte to provided address
 * @param addr 2-byte address to write to
 * @param byte byte data to be written at provided address
 */
void gb_put(uint16_t addr, uint8_t byte) {
    // if ROM is selected, do nothing
    if (gbx_cart_info | GBX_SM_CS_EN)
        return;

    // set cartridge to write mode if not already
    gbxcart_rd_set(false);
    gbxcart_wr_set(true);

    // put address & data
    _gb_abus_put(addr);
    _gb_dbus_put(byte);
    // pulse clock to write
    gbxcart_pulse_clock();
}

/**
 * @brief Puts data out on Gameboy Data Bus
 * @param data byte to be output on dbus
 */
void _gb_dbus_put(uint8_t byte) {
    uint32_t data = (uint32_t)(byte);
    // shift data bytes to match GB_DBUS_GPIO_MASK and write
    gpio_put_masked(GB_DBUS_GPIO_MASK, (uint32_t)(data)<<GB_DBUS_BITSHIFT);
}

/**
 * @brief Puts 2 byte address out on Gameboy Address Bus
 * @param addr 2 byte address to be output on abus
 */
void _gb_abus_put(uint16_t addr) {
    gpio_put_masked(GB_ABUS_GPIO_MASK, (uint32_t)(addr));
}
