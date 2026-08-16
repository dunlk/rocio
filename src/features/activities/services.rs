use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

use super::models::*;

#[derive(Serialize)]
struct DeleteActivityArgs {
    id: i64,
}

#[derive(Serialize)]
struct CreateActivityArgs {
    data: CreateActivity,
}

#[wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
    #[wasm_bindgen::prelude::wasm_bindgen(
        js_namespace = ["window", "__TAURI__", "core"]
)]
    fn invoke(cmd: &str, args: JsValue) -> js_sys::Promise;
}

pub async fn get_activities() -> Result<Vec<Activity>, String> {
    let result = JsFuture::from(invoke("get_activities", JsValue::NULL))
        .await
        .map_err(|error| format!("{error:?}"))?;

    from_value(result).map_err(|error| error.to_string())
}

pub async fn create_activity(data: CreateActivity) -> Result<Activity, String> {
    let args = CreateActivityArgs { data };
    let args = to_value(&args).map_err(|error| error.to_string())?;

    let result = JsFuture::from(invoke("create_activity", args))
        .await
        .map_err(|error| format!("{error:?}"))?;

    from_value(result).map_err(|error| error.to_string())
}

pub async fn delete_activity(id: i64) -> Result<(), String> {
    let args = DeleteActivityArgs { id };

    let args = serde_wasm_bindgen::to_value(&args).map_err(|error| error.to_string())?;

    JsFuture::from(invoke("delete_activity", args))
        .await
        .map_err(|error| format!("{error:?}"))?;

    Ok(())
}
