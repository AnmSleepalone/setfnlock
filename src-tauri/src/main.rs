// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use setfn::set_fn;
use std::process::CommandArgs;
mod setfn;

#[tauri::command]
fn toggle_switch( can_fn:bool ) -> String {
    return  set_fn(can_fn);
}

#[tauri::command]
fn handle_command() {
    let mut args = std::env::args();

    // 跳过第一个参数,因为它是程序的名称
    if let Some(_) = args.next() {
        if let Some(arg) = args.next() {
            match arg.as_str() {
                "-m" | "--mode" => {
                    if let Some(mode) = args.next() {
                        match mode.as_str() {
                            "lock" => {
                                // 执行 "lock" 模式的逻辑
                                println!("Locking the application...");
                                set_fn(true);
                            }
                            "unlock" => {
                                // 执行 "unlock" 模式的逻辑
                                println!("Unlocking the application...");
                                set_fn(false);
                            }
                            _ => {
                                println!("Unknown mode: {}", mode);
                            }
                        }
                    } else {
                        println!("Missing mode argument");
                    }
                }
                _ => {
                    println!("Unknown argument: {}", arg);
                }
            }
        }
    }
}



fn main() {
    let args = std::env::args();
        if args.len() > 1 {
            // 如果有命令行参数,则不展示界面
            println!("启动中");
            handle_command()
        } else {
            // 如果没有命令行参数,则展示界面
            tauri::Builder::default()
            .invoke_handler(tauri::generate_handler![toggle_switch])
            .run(tauri::generate_context!())
            .expect("启动时发生异常!");
        }
}
