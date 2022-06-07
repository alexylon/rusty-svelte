#[macro_use]
extern crate serde_derive;
extern crate js_sys;

mod actions;
mod utils;

use wasm_bindgen::prelude::*;
use serde::{Serialize};
use std::sync::atomic::{AtomicI32, AtomicUsize};
use std::sync::Mutex;
use js_sys::{JsString, Promise};
use once_cell::sync::Lazy;
use actions::{AddNameAction, UpdateNameAction};


static SUM: AtomicI32 = AtomicI32::new(0);
static J: AtomicUsize = AtomicUsize::new(0);

static STORE: Lazy<Mutex<Store>> = Lazy::new(|| { Mutex::new(Store::new()) });

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct Name {
    pub id: u32,
    first_name: String,
}

impl Name {
    pub fn new(id: u32, first_name: String) -> Name {
        Name { id, first_name }
    }
}

#[derive(Serialize, Clone)]
#[wasm_bindgen]
pub struct State {
    names: Vec<Name>,
}

impl State {
    fn new(names: Vec<Name>) -> State {
        State { names }
    }
}

#[wasm_bindgen]
pub struct Store {
    // listeners: Vec<&js_sys::Function>,
    // prev_states: Vec<State>,
    state: State,
}

#[wasm_bindgen]
impl Store {
    pub fn new() -> Store {
        utils::set_panic_hook();

        let mut names = Vec::new();
        let name1 = Name::new(1, "Marcus".to_string());
        let name2 = Name::new(2, "Gregor".to_string());
        let name3 = Name::new(3, "Rishitha".to_string());
        names.push(name1);
        names.push(name2);
        names.push(name3);

        Store {
            // listeners: Vec::new(),
            // prev_states: Vec::new(),
            state: State::new(names),
        }
    }
}

impl Store {
    pub fn get_state(&self) -> JsValue {
        utils::set_panic_hook();
        JsValue::from_serde(&self.state).unwrap()
    }

    pub fn add_name(&self, action: &JsValue) -> State {
        log("Calling add_name");
        utils::set_panic_hook();
        let action: AddNameAction = action.into_serde().unwrap();
        let mut names: Vec<Name> = self.state.names.clone();
        let id = (names.len() + 1) as u32;
        let name: Name = Name::new(id, action.first_name);
        names.push(name);
        State::new(names)
    }

    pub fn update_name(&self, action: &JsValue) -> State {
        utils::set_panic_hook();
        let action: UpdateNameAction = action.into_serde().unwrap();
        let names: Vec<Name> = self.state.names.iter()
            .map(|name| {
                if name.id == action.id {
                    Name::new(name.id, name.first_name.clone())
                } else {
                    name.clone()
                }
            })
            .collect();
        State::new(names)
    }


    // pub fn subscribe(&mut self, f: &js_sys::Function) {
    //     self.listeners.push(f);
    // }
}

#[wasm_bindgen]
pub async fn dispatch(action_type: JsString, action: JsValue) -> Result<JsValue, JsValue> {
    utils::set_panic_hook();
    let mut store = STORE.lock().unwrap();

    let action_t: String = action_type.into();
    // Get the new state
    let new_state: State = match action_t.as_str() {
        "AddName" => store.add_name(&action),
        "UpdateName" => store.update_name(&action),
        _ => todo!()
    };

    log(&format!("new_state: {}", &serde_json::to_string(&new_state).unwrap()));

    // Update the states in the store itself
    // self.prev_states.push(self.state.clone());
    store.state = new_state;

    log(&format!("state: {}", &serde_json::to_string(&store.state).unwrap()));

    // TODO: wasm-bindgen currently does not allow the wasm_bindgen trait for generic structs
    //       reimplement this when it does
    // Inform any subscribers
    // for listener in &self.listeners {
    //     let this = JsValue::NULL;
    //     log("Calling listener");
    //     match listener.call0(&this) {
    //         Ok(_) => log("Ok"),
    //         Err(e) => log("Err"),
    //     }
    // }

    // Test delay for 5 seconds
    //sleep(tokio::time::Duration::from_millis(5000)).await;
    let body: String = reqwest::get("https://www.rust-lang.org")
        .await.unwrap()
        .text()
        .await.unwrap();

    for i in 0..100000000 {
        let b = (i as f64).sqrt();
    }

    let json_value = JsValue::from_str(&body);
    let promise: Promise = js_sys::Promise::resolve(&json_value);
    let result = wasm_bindgen_futures::JsFuture::from(promise).await?;
    // Return OK for then and Err for promise catch in JavaScript
    Ok(result)
}



// #[wasm_bindgen]
// fn get_names() -> Vec<Name> {
//     utils::set_panic_hook();
//     let store = STORE.lock().unwrap();
//     store.state.names.clone()
// }
//
// #[wasm_bindgen]
// pub fn get_name_next() -> String {
//     utils::set_panic_hook();
//     let store = STORE.lock().unwrap();
//     let length = store.get_names().len();
//     let names = store.get_names();
//
//     J.fetch_add(1, Ordering::SeqCst);
//     let j = J.load(Ordering::SeqCst);
//
//     if j > length - 1 {
//         J.store(length - 1, Ordering::SeqCst);
//     }
//
//     let k = J.load(Ordering::SeqCst);
//
//     names.get(k).unwrap().clone().first_name
// }
//
// #[wasm_bindgen]
// pub fn get_name_previous() -> String {
//     utils::set_panic_hook();
//     let store = STORE.lock().unwrap();
//     let names = store.get_names();
//
//     let j = J.load(Ordering::SeqCst);
//     if j > 0 {
//         J.fetch_sub(1, Ordering::SeqCst);
//     }
//     let k = J.load(Ordering::SeqCst);
//
//     names.get(k).unwrap().clone().first_name
// }

#[wasm_bindgen]
pub fn get_state() -> JsValue {
    utils::set_panic_hook();
    let store = STORE.lock().unwrap();
    Store::get_state(&store)
}

