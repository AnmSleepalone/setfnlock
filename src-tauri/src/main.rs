// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use setfn::set_fn;
mod setfn;

#[tauri::command]
fn toggle_switch( can_fn:bool ) -> String {
    return  set_fn(can_fn);
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![toggle_switch])
        .run(tauri::generate_context!())
        .expect("启动时发生异常!");
}
