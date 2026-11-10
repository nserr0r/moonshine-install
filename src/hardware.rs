use std::path::Path;

use crate::config::Arch;
use crate::config::Firmware;

/// Что установщик узнаёт о машине, на которой запущен.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hardware {
    pub arch: Arch,
    pub firmware: Firmware,
}

impl Default for Hardware {
    fn default() -> Self {
        Self {
            arch: Arch::X86_64,
            firmware: Firmware::Bios,
        }
    }
}

impl Hardware {
    pub fn detect() -> Self {
        let arch = arch(std::env::consts::ARCH);

        Self {
            arch,
            firmware: firmware(arch, Path::new("/sys/firmware/efi").exists()),
        }
    }
}

pub fn arch(name: &str) -> Arch {
    match name {
        "aarch64" => Arch::Aarch64,
        _ => Arch::X86_64,
    }
}

/// На x86 диск грузится и по BIOS, и по UEFI; на ARM — UEFI, если он есть, иначе U-Boot.
pub fn firmware(arch: Arch, efi: bool) -> Firmware {
    match (efi, arch) {
        (_, Arch::X86_64) => Firmware::Hybrid,
        (true, Arch::Aarch64) => Firmware::Uefi,
        (false, Arch::Aarch64) => Firmware::Uboot,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_architecture_comes_from_the_target_name() {
        assert_eq!(arch("aarch64"), Arch::Aarch64);
        assert_eq!(arch("x86_64"), Arch::X86_64);
    }

    #[test]
    fn the_firmware_follows_efi_and_the_architecture() {
        assert_eq!(firmware(Arch::X86_64, true), Firmware::Hybrid);
        assert_eq!(firmware(Arch::Aarch64, true), Firmware::Uefi);
        assert_eq!(firmware(Arch::X86_64, false), Firmware::Hybrid);
        assert_eq!(firmware(Arch::Aarch64, false), Firmware::Uboot);
    }
}
