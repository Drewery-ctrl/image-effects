use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;

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
    log(&encoded_file.into());
}
