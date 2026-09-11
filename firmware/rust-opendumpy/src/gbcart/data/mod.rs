mod licensee_codes;

// represents a block of memory with a starting address and a size in Bytes
pub struct Block<T> {
    addr: T,
    size: usize,
}
impl<T> Block<T>
where
    T: Copy
{
    pub const fn new(addr: T, size: usize) -> Self {
        Self { addr, size }
    }

}


mod header {
    use heapless::string::String;
    use heapless::vec::Vec;
    use super::licensee_codes;
    use super::Block;

    mod field {
        use super::Block;

        pub const HEADER           : Block<u16> = Block::new(0x0100, 80usize);
        pub const ENTRY_POINT      : Block<u16> = Block::new(0x0100, 4usize);
        pub const NINTENDO_LOGO    : Block<u16> = Block::new(0x0104, 48usize);
        pub const TITLE            : Block<u16> = Block::new(0x0134, 16usize);
        pub const MANUFACTURER_CODE: Block<u16> = Block::new(0x013F, 4usize);
        pub const CGB_FLAG         : Block<u16> = Block::new(0x0143, 1usize);
        pub const NEW_LICENSEE_CODE: Block<u16> = Block::new(0x0144, 2usize);
        pub const SGB_FLAG         : Block<u16> = Block::new(0x0146, 1usize);
        pub const CARTRIDGE_TYPE   : Block<u16> = Block::new(0x0147, 1usize);
        pub const ROM_SIZE         : Block<u16> = Block::new(0x0148, 1usize);
        pub const RAM_SIZE         : Block<u16> = Block::new(0x0149, 1usize);
        pub const DESTINATION_CODE : Block<u16> = Block::new(0x014A, 1usize);
        pub const OLD_LICENSEE_CODE: Block<u16> = Block::new(0x014B, 1usize);
        pub const MASK_ROM_VERSION : Block<u16> = Block::new(0x014C, 1usize);
        pub const HEADER_CHECKSUM  : Block<u16> = Block::new(0x014D, 1usize);
        pub const GLOBAL_CHECKSUM  : Block<u16> = Block::new(0x014E, 2usize);

        pub const VALID_ASCII_RANGE: core::ops::Range<u8> =  0x20..0x61;
    }

    pub struct GBHeader {
        entry_point      : [u8; field::HEADER.size],
        nintendo_logo    : [u8; field::NINTENDO_LOGO.size],
        title            : [u8; field::TITLE.size],
        manufacturer_code: [u8; field::MANUFACTURER_CODE.size],
        cgb_flag         : [u8; field::CGB_FLAG.size],
        new_licensee_code: [u8; field::NEW_LICENSEE_CODE.size],
        sgb_flag         : [u8; field::SGB_FLAG.size],
        cartridge_type   : [u8; field::CARTRIDGE_TYPE.size],
        rom_size         : [u8; field::ROM_SIZE.size],
        ram_size         : [u8; field::RAM_SIZE.size],
        destination_code : [u8; field::DESTINATION_CODE.size],
        old_licensee_code: [u8; field::OLD_LICENSEE_CODE.size],
        mask_rom_version : [u8; field::MASK_ROM_VERSION.size],
        header_checksum  : [u8; field::HEADER_CHECKSUM.size],
        global_checksum  : [u8; field::GLOBAL_CHECKSUM.size]
    }
    impl GBHeader {
        pub fn empty() -> Self {
            Self {
                entry_point      : [0u8; field::HEADER.size],
                nintendo_logo    : [0u8; field::NINTENDO_LOGO.size],
                title            : [0u8; field::TITLE.size],
                manufacturer_code: [0u8; field::MANUFACTURER_CODE.size],
                cgb_flag         : [0u8; field::CGB_FLAG.size],
                new_licensee_code: [0u8; field::NEW_LICENSEE_CODE.size],
                sgb_flag         : [0u8; field::SGB_FLAG.size],
                cartridge_type   : [0u8; field::CARTRIDGE_TYPE.size],
                rom_size         : [0u8; field::ROM_SIZE.size],
                ram_size         : [0u8; field::RAM_SIZE.size],
                destination_code : [0u8; field::DESTINATION_CODE.size],
                old_licensee_code: [0u8; field::OLD_LICENSEE_CODE.size],
                mask_rom_version : [0u8; field::MASK_ROM_VERSION.size],
                header_checksum  : [0u8; field::HEADER_CHECKSUM.size],
                global_checksum  : [0u8; field::GLOBAL_CHECKSUM.size]
            }
        }

        pub fn from_bytes(bytes: [u8; field::HEADER.size]) -> Self {
            let mut i = 0usize;

            let entry_point = bytes[i..field::HEADER.size].try_into().unwrap();
            i += field::HEADER.size;

            let nintendo_logo = bytes[i..i+field::NINTENDO_LOGO.size].try_into().unwrap();
            i += field::NINTENDO_LOGO.size;

            let title = bytes[i..i+field::TITLE.size].try_into().unwrap();
            i += field::TITLE.size;                        

            let manufacturer_code = bytes[i..i+field::MANUFACTURER_CODE.size].try_into().unwrap();
            i += field::MANUFACTURER_CODE.size;

            let cgb_flag = bytes[i..i+field::CGB_FLAG.size].try_into().unwrap();
            i += field::CGB_FLAG.size;

            let new_licensee_code = bytes[i..i+field::NEW_LICENSEE_CODE.size].try_into().unwrap();
            i += field::NEW_LICENSEE_CODE.size;

            let sgb_flag = bytes[i..i+field::SGB_FLAG.size].try_into().unwrap();
            i += field::SGB_FLAG.size;

            let cartridge_type = bytes[i..i+field::CARTRIDGE_TYPE.size].try_into().unwrap();
            i += field::CARTRIDGE_TYPE.size;

            let rom_size = bytes[i..i+field::ROM_SIZE.size].try_into().unwrap();
            i += field::ROM_SIZE.size;

            let ram_size = bytes[i..i+field::RAM_SIZE.size].try_into().unwrap();
            i += field::RAM_SIZE.size;

            let destination_code = bytes[i..i+field::DESTINATION_CODE.size].try_into().unwrap();
            i += field::DESTINATION_CODE.size;

            let old_licensee_code = bytes[i..i+field::OLD_LICENSEE_CODE.size].try_into().unwrap();
            i += field::OLD_LICENSEE_CODE.size;

            let mask_rom_version = bytes[i..i+field::MASK_ROM_VERSION.size].try_into().unwrap();
            i += field::MASK_ROM_VERSION.size;

            let header_checksum = bytes[i..i+field::HEADER_CHECKSUM.size].try_into().unwrap();
            i += field::HEADER_CHECKSUM.size;

            let global_checksum = bytes[i..i+field::GLOBAL_CHECKSUM.size].try_into().unwrap();
            i += field::GLOBAL_CHECKSUM.size;

            Self {
                entry_point,
                nintendo_logo,
                title,
                manufacturer_code,
                cgb_flag,
                new_licensee_code,
                sgb_flag,
                cartridge_type,
                rom_size,
                ram_size,
                destination_code,
                old_licensee_code,
                mask_rom_version,
                header_checksum,
                global_checksum,
            }

        }

        pub fn has_cbg_support(&self) -> bool {
            // for more info: https://gbdev.io/pandocs/The_Cartridge_Header.html
            self.cgb_flag[0] == 0x80 || self.cgb_flag[0] == 0xC0
        }

        pub fn get_title(&self) -> String<{field::TITLE.size}> {
            let vec: Vec<u8, {field::TITLE.size}> = Vec::from_slice(self.title.as_slice()).unwrap();
            match String::from_utf8(vec) {
                Ok(str) => str,
                Err(error) => {
                    defmt::error!("GBHeader::get_title(&self): Failed to turn title bytes into characters.");
                    defmt::panic!();
                }
            }
        }

        pub fn get_new_licensee_name(&self) -> &str {
            licensee_codes::lookup(&self.new_licensee_code)
        }

        pub fn has_sgb_support(&self) -> bool {
            self.sgb_flag[0] == 0x03
        }

    }
}

