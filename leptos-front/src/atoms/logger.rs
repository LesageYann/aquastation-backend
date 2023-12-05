use js_sys::Array;
use sycamore::rt::JsValue;
use web_sys::console::log;

fn vec_to_js_array(vec: Vec<String>) -> Array {
    vec.into_iter().map(JsValue::from).collect()
}

pub struct Console {}

impl Console {
    pub fn log(vec: Vec<String>) {
        log(&vec_to_js_array(vec));
    }
}