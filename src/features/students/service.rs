use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

use crate::features::students::models::{CreateStudent, Student};

#[derive(Serialize)]
struct CreateStudentArgs {
    data: CreateStudent,
}

#[derive(Serialize)]
struct DeleteStudentArgs {
    id: i64,
}

#[derive(Serialize)]
pub struct UpdateStudent {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize)]
struct UpdateStudentArgs {
    id: i64,
    data: UpdateStudent,
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

pub async fn create_student(first_name: String, last_name: String) -> Result<Student, String> {
    let data = CreateStudent {
        first_name,
        last_name,
    };
    let args = CreateStudentArgs { data };

    let args = to_value(&args).map_err(|error| error.to_string())?;

    let result = JsFuture::from(invoke("create_student", args))
        .await
        .map_err(|error| format!("{error:?}"))?;

    from_value(result).map_err(|error| error.to_string())
}

pub async fn delete_student(id: i64) -> Result<(), String> {
    let args = DeleteStudentArgs { id };

    let args = serde_wasm_bindgen::to_value(&args).map_err(|error| error.to_string())?;

    JsFuture::from(invoke("delete_student", args))
        .await
        .map_err(|error| format!("{error:?}"))?;
    Ok(())
}

pub async fn update_student(id: i64, data: UpdateStudent) -> Result<Student, String> {
    let args = UpdateStudentArgs { id, data };

    let args = to_value(&args).map_err(|error| error.to_string())?;

    let result = JsFuture::from(invoke("update_student", args))
        .await
        .map_err(|error| format!("{error:?}"))?;

    from_value(result).map_err(|error| error.to_string())
}
