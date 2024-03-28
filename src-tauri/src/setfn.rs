use hidapi::HidApi;

const K380_VID: u16 = 0x46d;
const K380_PID: u16 = 0xb342;
const TARGET_USAGE: u16 = 1;
const TARGET_USAGE_PAGE: u16 = 65280;

const K380_SEQ_FKEYS_ON: [u8; 7] = [ 0x10, 0xff, 0x0b, 0x1e, 0x00, 0x00, 0x00];
const K380_SEQ_FKEYS_OFF: [u8; 7] = [ 0x10, 0xff, 0x0b, 0x1e, 0x01, 0x00, 0x00];


pub fn set_fn(set_media_keys: bool) -> String {
    let hidapi = HidApi::new().unwrap();
    for device in hidapi.device_list(){
        if device.vendor_id()== K380_VID && device.product_id() == K380_PID {
            if device.usage()==TARGET_USAGE && device.usage_page() == TARGET_USAGE_PAGE {
                let k380 = device.open_device(&hidapi).unwrap();
                   let res =  if set_media_keys {
                        match k380.write(&K380_SEQ_FKEYS_ON) {
                            Ok(_) => String::from("注入成功!"),
                            Err(err) => format!("注入失败: {:?}", err)
                        }
                    } else {
                        match k380.write(&K380_SEQ_FKEYS_OFF) {
                            Ok(_) => String::from("注入成功!"),
                            Err(err) => format!("注入失败: {:?}", err)
                        }
                    };
                    return  res;
            }
        }
    }
    return String::from("没有找到k380设备!");
}