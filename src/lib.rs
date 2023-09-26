use std::io::Cursor;
use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;
use base64::*;
use image::load_from_memory;
use image::ImageOutputFormat::Png;

#[wasm_bindgen]
pub fn grayscale(encoded_image:&str) -> String{
    log(&"the encoded image came".into());
    let _base64_to_vector:Vec<u8> =decode(encoded_image).unwrap();
    log(&"converted to vec succefully".into());
    let mut _img =load_from_memory(&_base64_to_vector).unwrap();
    log(&"loaded into memory successfully".into());
    _img=_img.grayscale();
    log(&"became gray successfully".into());
    let mut buffer=Vec::new();
    _img.write_to(&mut Cursor::new(&mut buffer),Png).unwrap();
    log(&"bufferd into vec again successfully".into());
    let encoded= encode(buffer);
    let data_url =format!(
        "data:image/png;base64,{}",
        encoded
    );
    return data_url;
}
#[wasm_bindgen]
pub fn blur(encoded_image:&str) -> String{
    log(&"the encoded image came".into());
    let _base64_to_vector:Vec<u8> =decode(encoded_image).unwrap();
    log(&"converted to vec succefully".into());
    let mut _img =load_from_memory(&_base64_to_vector).unwrap();
    log(&"loaded into memory successfully".into());
    _img=_img.blur(25.0);
    log(&"became gray successfully".into());
    let mut buffer=Vec::new();
    _img.write_to(&mut Cursor::new(&mut buffer),Png).unwrap();
    log(&"bufferd into vec again successfully".into());
    let encoded= encode(buffer);
    let data_url =format!(
        "data:image/png;base64,{}",
        encoded
    );
    return data_url;
}