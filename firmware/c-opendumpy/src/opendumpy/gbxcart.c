#include "hardware/gpio.h"
#include "opendumpy/gbconst.h"
#include "opendumpy/gbxcart.h"

// Define cartridge control flags variable.
uint8_t gbx_cart_info = 0x00;

bool gbx_is_gb() {
    return !(gbx_cart_info | GBX_SM_GBC_CART);
}

bool gbx_is_gbc() {
    return gbx_cart_info | GBX_SM_GBC_CART;
}

// Initializes all GPIO for the gameboy cartridge.
void _gbxcart_gpio_setup() {
    // Initialize all GBX control GPIO pins and set them to GPIO_OUT
    gpio_init_mask64(GBX_GPIO_CTL_MASK);
    gpio_set_dir_out_masked64(GBX_GPIO_CTL_MASK);

    // cart-specific gpio initialization
    if (gbx_is_gb())
        gpio_init_mask64(GB_GPIO_ABUS_MASK | GB_GPIO_DBUS_MASK);
    else
        gpio_init_mask64(GBC_GPIO_ABUS_MASK | GBC_GPIO_DBUS_MASK);
}
