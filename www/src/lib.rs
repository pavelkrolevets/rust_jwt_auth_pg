mod utils;

use core::str;
use wasm_bindgen::prelude::*;
use web_sys::Document;
use web_sys::Element;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;
use web_sys::HtmlButtonElement;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // // Login element
    // let log_text = document.create_element("p")?;
    // log_text.set_inner_html("Login");
    // body.append_child(&log_text)?;
    // let log_inp = document.create_element("input")?;
    // log_inp.id = "window_login";
    // body.append_child(&log_inp)?;


    // // Passord element
    // let pswd_text = document.create_element("p")?;
    // pswd_text.set_inner_html("Password");
    // body.append_child(&pswd_text)?;
    // let pswd_inp = document.create_element("input")?;
    // body.append_child(&pswd_inp)?;
    
    // let element1 = document.get_element_by_id("window_login");
    // let inp_val =  element1.onchange("value")?;
    // set_login(inp_val);

    let input_box = create_input_box(&document);
    body.append_child(&input_box)?;
    let input_value = document
    .get_element_by_id("name")
    .unwrap()
    .dyn_into::<HtmlInputElement>()
    .unwrap()
    .value();
    
    let ok_button = create_button(&document);
    body.append_child(&ok_button)?;
    // set_login(&input_value);

    Ok(())
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[wasm_bindgen]
pub fn set_login(a: &str) {
    alert(a)
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

fn create_input_box(document: &Document) -> Element {
    let input_box = document.create_element("input").unwrap();
    input_box.set_attribute("name", "name");
    input_box.set_attribute("value", "Delhi");
    input_box.set_attribute("type", "text");
    input_box.set_attribute("placeholder", "Type city name here");
    input_box.set_id("name");
    input_box.set_class_name("ReportStyles-search");
    input_box
}

fn create_button(document: &Document) -> Element {
    let button = document.create_element("input").unwrap();
    button.set_attribute("name", "ok_button");
    button.set_attribute("value", "OK");
    button.set_attribute("type", "button");
    button.set_id("ok_button");
    button.set_class_name("ReportStyles-search");
    button
}