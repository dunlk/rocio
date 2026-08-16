use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

use super::models::*;

#[wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
    #[wasm_bindgen::prelude::wasm_bindgen(
        js_namespace = ["window", "__TAURI__", "core"]
)]
    fn invoke(cmd: &str, args: JsValue) -> js_sys::Promise;
}

pub async fn get_activities() -> Result<Vec<Activity>, String> {
    let result = JsFuture::from("get_activities", JsValue::NULL)
        .await
        .map_err(|error| format!("{error}"))?;

    from_value(result).map_err(|error| error.to_string())
}
