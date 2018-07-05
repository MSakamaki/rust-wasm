extern crate wasm_rust;
use wasm_rust::*;

#[test]
fn add_1_2_to_3() {
    assert_eq!(3, add(1,2));
}
