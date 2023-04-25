use base64;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[wasm_bindgen]
pub fn decode_base64(something: &str) -> usize {
    // let decoded_bytes = base64::decode(base64_string).unwrap();
    // String::from_utf8(decoded_bytes).unwrap()
    // base64_string.to_string()
    // conver the string to a byte array
    let bytes = something.as_bytes();
    // convert the byte array to a base64 string
    let base64_string = base64::encode(bytes);
    // return the base64 string
    base64_string.len()
    // 42
    // "{}", base64_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
