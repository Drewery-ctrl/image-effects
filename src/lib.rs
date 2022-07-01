use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;
use base64::{encode, decode};
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
pub fn grayscale(encoded_file: &str) -> String {
    log(&"Grayscale called with".into());

    // Decode the base64 string into a Vec<u8> binary
    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"Image decoded".into());

    // Load the image from the Vec<u8>
    let mut image = load_from_memory(&base64_to_vector).unwrap();
    log(&"Image loaded".into());

    // Convert the image to grayscale
    image = image.grayscale();
    log(&"Image grayscale effect applied".into());

    // use buffer to store the image while the image is being converted back to binary data
    let mut buffer = Vec::new();
    // '&mut buffer' allows function to borrow this variable
    image.write_to(&mut buffer, Png).unwrap();
    log(&"New image converted to binary/written to buffer".into());

    // Convert the buffer to a base64 string
    let base64_string = encode(&buffer);
    let data_url = format!("data:image/png;base64,{}", base64_string);

    log(&"New image converted to base64".into());

    // Return the base64 string
    data_url
}
