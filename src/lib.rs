#[path = "constants/regexConstants.rs"]
mod imported_constants;
mod utils;

use regex::Regex;
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
/// Returns user device type
///
/// # Arguments
///
/// * `navigator_string` - Current user navigator string ´navigator.userAgent´
///
/// # Examples
///
/// ```
/// // Usage
/// get_device_type(navigator.userAgent)
///
/// // Returns
/// "Mobile" | "Tablet" | "Desktop"
/// ```
pub fn get_device_type(navigator_string: String) -> String {
    if Regex::new(imported_constants::constants::MOBILE_REGEX)
        .unwrap()
        .is_match(navigator_string.as_str())
    {
        return DeviceType::Mobile.to_string();
    }
    if Regex::new(imported_constants::constants::TABLET_REGEX)
        .unwrap()
        .is_match(navigator_string.as_str())
    {
        return DeviceType::Tablet.to_string();
    }
    return DeviceType::Desktop.to_string();
}
