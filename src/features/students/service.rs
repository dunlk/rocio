use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

use crate::features::students::models::{CreateStudent, Student};

#[derive(Serialize)]
struct CreateStudentArgs {
    data: CreateStudent,
}

#[wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
    #[wasm_bindgen::prelude::wasm_bindgen(
        js_namespace = ["window", "__TAURI__", "core"]
)]
    fn invoke(cmd: &str, args: JsValue) -> js_sys::Promise;
}

pub async fn get_students() -> Result<Vec<Student>, String> {
    let result = JsFuture::from(invoke("get_students", JsValue::NULL))
        .await
        .map_err(|error| format!("{error:?}"))?;

    from_value(result).map_err(|error| error.to_string())
}

pub async fn create_student(data: CreateStudent) -> Result<Student, String> {
    let args = CreateStudentArgs { data };

    let args = to_value(&args).map_err(|error| error.to_string())?;

    let result = JsFuture::from(invoke("create_student", args))
        .await
        .map_err(|error| format!("{error:?}"))?;

    from_value(result).map_err(|error| error.to_string())
}
