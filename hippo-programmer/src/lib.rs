use std::time::Duration;

use ftdaye::{ftdaye::jtag::FtdiMpsse, xilinx7::IR_USER3};

#[derive(Debug)]
pub enum FtdiError {
    USBDeviceNotFound,
    USBDeviceFailedToOpen(String),
    WriteFailed,
}

pub fn open_mpsse(vid: u16, pid: u16, f: u32) -> Result<FtdiMpsse, FtdiError> {
    let device_info = nusb::list_devices()
        .unwrap()
        .find(|dev| dev.vendor_id() == vid && dev.product_id() == pid);
    let device_info = match device_info {
        Some(device_info) => device_info,
        None => return Err(FtdiError::USBDeviceNotFound),
    };

    let device = ftdaye::ftdaye::Builder::new()
        .with_interface(ftdaye::ftdaye::Interface::A)
        .with_read_timeout(Duration::from_secs(5))
        .with_write_timeout(Duration::from_secs(5))
        .usb_open(device_info);
    let device = match device {
        Ok(device) => device,
        Err(val) => return Err(FtdiError::USBDeviceFailedToOpen(format!("{:?}", val))),
    };

    let mut ft = FtdiMpsse::new(device, f);
    ft.reset_and_to_rti();

    Ok(ft)
}

pub fn write_bytes(mpsse: &mut FtdiMpsse, bytes: &[u8]) {
    mpsse.reset_and_to_rti();
    println!("Writing {} bytes to device", bytes.len());
    mpsse.write_register(IR_USER3, bytes);
    mpsse.assert_ftdi_buffer_empty();
    println!("Finished");
    mpsse.reset_and_to_rti();
}
