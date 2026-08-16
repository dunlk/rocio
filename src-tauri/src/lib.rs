mod database;
mod features;

use database::create_pool;
use database::migrations::run_migrations;
use tauri::Manager;

use features::students::commands::{create_student, delete_student, get_students, update_student};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("No se pudo obtener el directorio de datos");

            std::fs::create_dir_all(&app_data_dir)
                .expect("No se pudo crear el directorio de datos");

            let database_path = app_data_dir.join("rocio.db");

            println!("Database: {}", database_path.display());

            let pool = tauri::async_runtime::block_on(async {
                println!("Database: {}", database_path.display());
                let pool = create_pool(&database_path)
                    .await
                    .expect("Error al conectarse a SQLite");

                run_migrations(&pool)
                    .await
                    .expect("No se pudieron ejecutar las migraciones");

                pool
            });

            app.manage(pool);

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_students,
            create_student,
            update_student,
            delete_student
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
