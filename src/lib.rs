mod gbloader {
    pub struct DMG {
        entry_point: u16,            // Entry point of the ROM which is always 0x0100
        nintendo_logo: Vec<u8>, // Nintendo logo as uint8_t array of size 0x30 : 0x0104 - 0x0133
        title: String,          // Title of the game as ASCII : 0x0134 - 0x0143
        new_license_code: String, // New license code used on games released after SGB. Only set if m_licenseCode == 0x33 : 0x0144 - 0x0145
        sgb_flag: u8, // 0x00 - No SGB functionality, 0x03 - Game supports SGB functionality : 0x0146
        cartridge_type: u8, // Specifies which external cartridge exists in the cartridge (eg. Memory Bank Controller) : 0x0147
        rom_size: u8, // Specifies the ROM size of the cartridge. Calculated as 32KB << N : 0x0148
        ram_size: u8, // Size of external RAM in cartridge (if any). Must be 0x00 for MBC2 : 0x0149
        destination_code: u8, // 0x00 == Japanese, 0x01 == Non-Japanese : 0x014A
        license_code: u8, // Single byte license code. A value of 0x33 points to the use of m_newLicenseCode : 0x014B
        mask_rom_version_number: u8, // Version number of the game, usually 0x00 : 0x014C
        header_checksum: u8, // Checksum across bytes 0x0134 - 0x014C, the game won't work if the checksum is incorrect : 0x014D
        global_checksum: u16, // Checksum calculated by adding all bytes of the cartridge, except the two checksum bytes : 0x014E - 0x014F
    }

    impl DMG {
        pub fn new(rom_data: Vec<u8>) -> Result<DMG, std::str::Utf8Error> {
            let license_code = rom_data[0x14B];
            let cartridge_type = rom_data[0x147];
            let title = std::str::from_utf8(&rom_data[0x134..0x144])?.to_string();
            let new_license_code = if license_code == 0x33 {
                std::str::from_utf8(&rom_data[0x144..0x146])?.to_string()
            } else {
                "".to_string()
            };

            Ok(DMG {
                entry_point: 0x100,
                nintendo_logo: rom_data[0x104..0x133].to_vec(),
                title: title,
                sgb_flag: rom_data[0x146],
                cartridge_type: rom_data[0x147],
                rom_size: rom_data[0x148],
                ram_size: if cartridge_type != 0x05 {
                    rom_data[0x149]
                } else {
                    0
                },
                destination_code: rom_data[0x14A],
                license_code: rom_data[0x14B],
                mask_rom_version_number: rom_data[0x14C],
                header_checksum: rom_data[0x14D],
                global_checksum: ((rom_data[0x14E] as u16) << 8) | rom_data[0x14F] as u16,
                new_license_code: new_license_code,
            })
        }

        pub fn get_entry_point(&self) -> u16 {
            self.entry_point
        }

        pub fn get_nintendo_logo(&self) -> &Vec<u8> {
            &self.nintendo_logo
        }

        pub fn get_title(&self) -> &String {
            &self.title
        }

        pub fn get_sgb_flag(&self) -> u8 {
            self.sgb_flag
        }

        pub fn get_cartridge_type(&self) -> u8 {
            self.cartridge_type
        }

        pub fn get_rom_size(&self) -> u8 {
            self.rom_size
        }

        pub fn get_ram_size(&self) -> u8 {
            self.ram_size
        }

        pub fn get_destination_code(&self) -> u8 {
            self.destination_code
        }

        pub fn get_license_code(&self) -> u8 {
            self.license_code
        }

        pub fn get_mask_romversion_number(&self) -> u8 {
            self.mask_rom_version_number
        }

        pub fn get_header_checksum(&self) -> u8 {
            self.header_checksum
        }

        pub fn get_global_checksum(&self) -> u16 {
            self.global_checksum
        }

        pub fn get_new_license_code(&self) -> &String {
            &self.new_license_code
        }
    }
}

#[cfg(test)]
mod tests {
    use super::gbloader::*;
    use std::fs::File;
    use std::io::*;

    fn load_rom() -> DMG {
        let mut file = File::open("test_roms/header_only_test.gb").unwrap();
        let mut buffer = vec![];
        file.read_to_end(&mut buffer).unwrap();

        DMG::new(buffer).unwrap()
    }

    #[test]
    fn new() {
        let mut file = File::open("test_roms/header_only_test.gb").unwrap();
        let mut buffer = vec![];
        file.read_to_end(&mut buffer).unwrap();

        assert_eq!(DMG::new(buffer).is_ok(), true);
    }

    #[test]
    fn get_entry_point() {
        let header = load_rom();
        assert_eq!(header.get_entry_point(), 0x0100);
    }

    #[test]
    fn get_title() {
        let header = load_rom();
        assert_eq!(header.get_title(), "GBLOADERTEST1234");
    }

    #[test]
    fn get_new_license_code() {
        let header = load_rom();
        assert_eq!(header.get_new_license_code(), "01");
    }

    #[test]
    fn get_sgb_flag() {
        let header = load_rom();
        assert_eq!(header.get_sgb_flag(), 0x03);
    }

    #[test]
    fn get_cartridge_type() {
        let header = load_rom();
        assert_eq!(header.get_cartridge_type(), 0x01);
    }

    #[test]
    fn get_rom_size() {
        let header = load_rom();
        assert_eq!(header.get_rom_size(), 0x02);
    }

    #[test]
    fn get_ram_size() {
        let header = load_rom();
        assert_eq!(header.get_ram_size(), 0x03);
    }

    #[test]
    fn get_destination_code() {
        let header = load_rom();
        assert_eq!(header.get_destination_code(), 0x01);
    }

    #[test]
    fn get_license_code() {
        let header = load_rom();
        assert_eq!(header.get_license_code(), 0x33);
    }

    #[test]
    fn get_mask_romversion_number() {
        let header = load_rom();
        assert_eq!(header.get_mask_romversion_number(), 0);
    }

    #[test]
    fn get_header_checksum() {
        let header = load_rom();
        assert_eq!(header.get_header_checksum(), 0xFF);
    }

    #[test]
    fn get_global_checksum() {
        let header = load_rom();
        assert_eq!(header.get_global_checksum(), 0);
    }

    #[test]
    fn get_nintendo_logo() {
        let nintendo_logo_reference = vec![
            0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C,
            0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
            0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
            0x99, 0x9F, 0xBB, 0xB9, 0x33,
        ];

        let header = load_rom();
        let nintendo_logo = header.get_nintendo_logo();

        assert_eq!(&nintendo_logo_reference, nintendo_logo);
    }
}
