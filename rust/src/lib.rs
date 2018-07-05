#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

// use std::thread;
// use std::time::Duration;

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    // alert(&format!("Hello, {}!", name));
    log(&format!("Hello WASM {}!", name));
}

// ############# main

#[wasm_bindgen]
pub fn main() {
    log("Hello Main() World");
}


#[wasm_bindgen]
pub fn add(x: i32, y:i32) -> i32 {
    x + y
}

static ARRAY_SHARE: [i8; 5] = [1, 2, 3, 4, 5];

#[wasm_bindgen]
pub fn array_mem() -> *const i8 {
    &ARRAY_SHARE[0]
}


#[wasm_bindgen]
pub fn get_name(name: &str) -> String {
    let r = &format!("name is {}!", name);
    return r.to_string();
}

#[wasm_bindgen]
pub fn fillter_array(name: &str) -> bool {
    name.len() > 5
}


// ############# tutorial 2

// struct Philosopher {
//     name: String,
// }

// impl Philosopher {
//     fn new (name: &str) -> Philosopher {
//         Philosopher {
//             name: name.to_string(),
//         }
//     }
//     fn eat (&self) {

//         log(&format!("{} is eating.", self.name));

//         // thread::sleep(Duration::from_millis(1000));

//         log(&format!("{} is done eating.", self.name));
        
//     }
// }

// #[wasm_bindgen]
// pub fn tutorial2() {
//     let philosophers = vec![
//         Philosopher::new("Judith Butler"),
//         Philosopher::new("Gilles Deleuze"),
//         Philosopher::new("Karl Marx"),
//         Philosopher::new("Emma Goldman"),
//         Philosopher::new("Michel Foucault"),
//     ];
//     for p in &philosophers {
//         p.eat();
//     }

// }
