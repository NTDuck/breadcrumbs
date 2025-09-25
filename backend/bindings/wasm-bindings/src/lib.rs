use ::wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn addTwoNumbers(lhs: u32, rhs: u32) -> u32 {
    lhs + rhs
}
