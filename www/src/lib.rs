mod utils;

extern crate reqwest;

use core::str;
use wasm_bindgen::prelude::*;
use web_sys::Document;
use web_sys::Element;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;
use web_sys::HtmlButtonElement;
use gloo::events::EventListener;
use wasm_bindgen_futures::spawn_local;
use json::JsonValue;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // // Login element
    let first_div = document.create_element("div")?;
    let second_div = create_div(&document, "second_div", "col-md-6");
    

    let input_box = create_input_box(&document, &String::from("login"));
    first_div.append_child(&input_box)?;
    
    let ok_button = create_button(&document);
    first_div.append_child(&ok_button)?;

    
    first_div.append_child(&second_div)?;
    body.append_child(&first_div)?;

    let on_click = EventListener::new(&ok_button, "click", move |_event| {
        let temp_d = second_div.clone();
        let input_value = document
            .get_element_by_id("login")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value();
        let input_value: &'static _ = Box::leak(Box::new(input_value));
       

        let response = get_response(&input_value);
        spawn_local(async move {
            let parsed = response.await;
            temp_d.set_inner_html(&parsed);
        }

        
           
        
    });
    
    on_click.forget();

    Ok(())
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

fn create_input_box(document: &Document, id: &str) -> Element {
    let input_box = document.create_element("input").unwrap();
    input_box.set_attribute("name", &id);
    input_box.set_attribute("value", &id);
    input_box.set_attribute("type", "text");
    input_box.set_attribute("placeholder", "Type city name here");
    input_box.set_id(&id);
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

fn create_div(document: &Document, id: &str, class: &str) -> Element {
    let div = document.create_element("div").unwrap();
    div.set_id(id);
    div.set_class_name(class);
    div
}

// Get response from api
async fn get_response(login: &str, password: &str) -> JsonValue {
    let url = "http://127.0.0.1:8001/auth/login";

    let params = [("email", &login), ("password", &password)];

    let resp = reqwest::post(&url)
        .form(&params)
        .await.unwrap()
        .text()
        .await.unwrap();

    json::parse(&resp).unwrap()
}