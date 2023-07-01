// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use std::{thread, time};
use tauri::Manager;
use tauri::{App, CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu};

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    use {
        std::thread::{sleep, spawn},
        std::time::Duration,
    };

    // let msg = String::from("Hello, world!");
    // println!("Message from Rust: {}", msg);
    let tray_menu = SystemTrayMenu::new();
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .setup(|app| {
            let app_ = app.handle();

            // listen to the `event-name` (emitted on any window)
            app.listen_global("send-message", move |event| {
                println!("got event-name with payload {:?}", event.payload());
                app_.emit_all(
                  "message-back-end",
                  Payload {
                      message: "Tauri is awesome! (coming from back-end)".into(),
                  },
              )
              .unwrap();
            });


            // 3 --- emit event to frontend
            

            // app.unlisten(id);

            Ok(())
        })
        .system_tray(system_tray)
        .invoke_handler(tauri::generate_handler![greet, background_function_1,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn background_function_1() {
    println!("Hello from background function 1");

    let ten_millis = time::Duration::from_millis(10);
    // let now = time::Instant::now();
    thread::sleep(ten_millis);
    println!("Hello from background function 1");
}
