#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

mod filters;

/// この関数は２つの数字を足した結果を返却する
///
/// # Examples
///
/// ```
/// assert_eq!(5, wasm_rust::add(2, 3));
/// ```
#[wasm_bindgen]
pub fn add(x: i32, y:i32) -> i32 {
    x + y
}

/// この関数は名前に対して返事を返す
///
/// # Examples
///
/// ```
/// assert_eq!("my name is TARO!", wasm_rust::get_name("TARO"));
/// ```
#[wasm_bindgen]
pub fn get_name(name: &str) -> String {
    (&format!("my name is {}!", name)).to_string()
}

/// この関数は名前に対して返事を返す
///
/// # Examples
///
/// ```
/// assert!( ! wasm_rust::has_name_langth_file_over("12345"));
/// assert!(wasm_rust::has_name_langth_file_over("123456"));
/// ```
#[wasm_bindgen]
pub fn has_name_langth_file_over(name: &str) -> bool {
    filters::has_five_length(name)
}
