mod api;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            api::validate_license,
            api::start_installation,
            api::prepare_after_activation,
            api::list_after_effects,
            api::is_after_effects_running,
            api::get_runtime_info
        ])
        .run(tauri::generate_context!())
        .expect("tauri failed");
}
