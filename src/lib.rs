use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;
use base64::decode as b64_decode;
use image::load_from_memory;
use image::ImageOutputFormat::Png;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) {
    log(&"Grayscale called with".into());

    let base64_to_vector = b64_decode(encoded_file).unwrap();
    log(&"Image decoded".into());

    let mut image = load_from_memory(&base64_to_vector).unwrap();
    log(&"Image loaded".into());

    image = image.grayscale();
    log(&"Image grayscale effect applied".into());

    // use buffer to store the image while the image is being converted back to binary data
    let mut buffer = Vec::new();
    // '&mut buffer' allows function to borrow this variable
    image.write_to(&mut buffer, Png).unwrap();
    log(&"New image converted to binary/written to buffer".into());
}
