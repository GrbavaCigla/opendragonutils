use std::error::Error;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::u16;

use udev::Device;

#[derive(Debug)]
pub struct UsbDevice {
    pub vendor_id: u16,
    pub product_id: u16,

    pub product_string: String,
    pub vendor_string: String,

    pub systen_path: PathBuf,
}

impl UsbDevice {
    pub fn from(device: Device) -> Result<Self, Box<dyn Error>> {
        let vendor_id = u16::from_str_radix(
            device
                .property_value("ID_VENDOR_ID")
                .ok_or("No ID_VENDOR_ID property found.")?
                .to_str()
                .ok_or("Failed to convert ID_VENDOR_ID to string.")?,
            16,
        )?;
        let product_id = u16::from_str_radix(
            device
                .property_value("ID_MODEL_ID")
                .ok_or("No ID_MODEL_ID property found.")?
                .to_str()
                .ok_or("Failed to convert ID_MODEL_ID to string.")?,
            16,
        )?;

        let vendor_string = device
            .property_value("ID_VENDOR_FROM_DATABASE")
            .ok_or("No ID_MODEL_ID property found.")?;
        let product_string = device
            .property_value("ID_MODEL_FROM_DATABASE")
            .ok_or("No ID_MODEL_ID property found.")?;

        Ok(UsbDevice {
            systen_path: PathBuf::from(device.syspath()),
            vendor_id: vendor_id,
            product_id: product_id,
            product_string: product_string.to_string_lossy().to_string(),
            vendor_string: vendor_string.to_string_lossy().to_string(),
        })
    }
}

pub fn list_redragon_devices() -> Result<Vec<UsbDevice>, Box<dyn Error>> {
    let holtek_vendor = 0x04d9;
    let product_ids = [0xfc38, 0xfc4c];

    let mut enumerator = udev::Enumerator::new()?;

    let mut res = vec![];

    for device in enumerator.scan_devices()? {
        let device = match UsbDevice::from(device) {
            Ok(dev) => dev,
            Err(_) => continue
        };

        if holtek_vendor == device.vendor_id && product_ids.contains(&device.product_id) {
            res.push(device);
        }
    }

    Ok(res)
}
