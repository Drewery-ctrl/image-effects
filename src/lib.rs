use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;
use base64::decode as b64_decode;

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
}
