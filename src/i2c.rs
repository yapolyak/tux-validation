use anyhow::Result;
use i2cdev::core::*;
use i2cdev::linux::LinuxI2CDevice;

pub trait I2cScanner {
    fn scan(&self) -> Result<Vec<u16>>; // TODO: add address range as parameter
}

pub struct LinuxI2cScanner {
    pub bus_path: String,
}

impl I2cScanner for LinuxI2cScanner {
    fn scan(&self) -> Result<Vec<u16>> {
        let mut detected = Vec::new();

        for addr in 0x03..=0x77 {
            if let Ok(mut dev) = LinuxI2CDevice::new(&self.bus_path, addr) {
                // smbus_write_quick() sends a 0-byte write. 
                // If the device exists, the kernel returns 0 (Ok).
                // If it fails (NACK), it returns an error.
                if dev.smbus_write_quick(false).is_ok() {
                    detected.push(addr);
                }
            }
        }
        Ok(detected)
    }
}

pub struct I2cValidationResult {
    pub missing: Vec<u16>,
    pub unexpected: Vec<u16>,
    pub present: Vec<u16>,
}

pub fn validate_bus(
    scanner: &impl I2cScanner, 
    expected_addresses: &[u16]
) -> Result<I2cValidationResult> {
    let detected = scanner.scan()?;
    
    let mut result = I2cValidationResult {
        missing: Vec::new(),
        unexpected: Vec::new(),
        present: Vec::new(),
    };

    for &addr in expected_addresses {
        if detected.contains(&addr) {
            result.present.push(addr);
        } else {
            result.missing.push(addr);
        }
    }

    for &addr in &detected {
        if !expected_addresses.contains(&addr) {
            result.unexpected.push(addr);
        }
    }

    Ok(result)
}