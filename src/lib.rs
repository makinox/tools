mod utils;

// use regex::Regex;
use std::string::ToString;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn sum(a: i8, b: i8) -> i8 {
    return a + b;
}

#[wasm_bindgen]
pub fn add_heading() -> Result<web_sys::Element, JsValue> {
    let window = web_sys::window().expect("no window found");
    let document = window.document().expect("no document on window");
    let body = document.body().expect("no body on document");

    let heading = document.create_element("h1")?;
    heading.set_inner_html("This heading was created from Rust!");

    body.append_child(&heading)?;

    Ok(heading)
}

#[derive(strum_macros::Display)]
pub enum DeviceType {
    Mobile,
    Tablet,
    Desktop,
}

#[wasm_bindgen]
pub fn get_device_type(navigator_string: String) -> Result<String, String> {
    // let mobile_regex =
    //     Regex::new("/Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i").unwrap();
    // let tablet_regex = Regex::new("/(tablet|ipad|playbook|silk)|(android(?!.*mobi))/i").unwrap();
    dbg!(navigator_string);

    // if mobile_regex.is_match(navigator_string.as_str()) {
    //     return Ok(DeviceType::Mobile.to_string());
    // }
    // if tablet_regex.is_match(navigator_string.as_str()) {
    //     return Ok(DeviceType::Tablet.to_string());
    // }
    return Ok(DeviceType::Desktop.to_string());
}
