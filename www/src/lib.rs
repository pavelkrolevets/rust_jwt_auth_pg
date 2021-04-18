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
use serde::{Deserialize, Serialize};
// use json::*;
use reqwest::Client;

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    email: String,
    password: String,
}

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
    

    let login_box = create_input_box(&document, &String::from("login"));
    first_div.append_child(&login_box)?;

    let password_box = create_input_box(&document, &String::from("password"));
    first_div.append_child(&password_box)?;
    
    let ok_button = create_button(&document);
    first_div.append_child(&ok_button)?;

    first_div.append_child(&second_div)?;
    body.append_child(&first_div)?;

    let on_click = EventListener::new(&ok_button, "click", move |_event| {
        let temp_d = second_div.clone();
        let login_value = document
            .get_element_by_id("login")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value();
        let login_value: &'static _ = Box::leak(Box::new(login_value));
        
        let password_value = document
            .get_element_by_id("password")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value();
        let password_value: &'static _ = Box::leak(Box::new(password_value));

        // temp_d.set_inner_html(&password_value);
        // temp_d.set_inner_html(&login_value);

        let response = login(login_value.to_string(), password_value.to_string());
        spawn_local(async move {
            let parsed = response.await;
            let lat = (&parsed["token"]).to_owned().to_string();
            temp_d.set_inner_html(&lat);
        });
           
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
async fn login(login: String, password: String) -> JsonValue {
    let url = "http://127.0.0.1:8001/auth/login";

    let new_post = Post {
        email: login,
        password: password,
    };
    
    let client = reqwest::Client::new();
    let req = client
        .post(url)
        .json(&new_post)
        .header("Accepts", "application/json");
   
    let resp = req.send().await.unwrap();
    let body = resp.text().await.unwrap();
    json::parse(&body).unwrap()
}