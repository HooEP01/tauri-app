// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

// Window Menu
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

// Window
use tauri::WindowBuilder;

// System Tray
use tauri::{SystemTray, SystemTrayMenu};

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

fn main() {
    // Menu Instance
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(submenu);

    let window_menu = Menu::new().add_item(CustomMenuItem::new("quit".to_string(), "King"));

    let tray_menu = SystemTrayMenu::new();
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .setup(|app| {
            let window = WindowBuilder::new(
                app,
                "main-window".to_string(),
                tauri::WindowUrl::App("index.html".into()),
            )
            .menu(window_menu)
            .build()?;

            let window_ = window.clone();
            window.on_menu_event(move |event| match event.menu_item_id() {
                "quit" => {
                    std::process::exit(0);
                }
                "close" => {
                    window_.close().unwrap();
                }
                _ => {}
            });

            let main_window = app.get_window("main").unwrap();
            let menu_handle = main_window.menu_handle();
            std::thread::spawn(move || {
                // you can also `set_selected`, `set_enabled` and `set_native_image` (macOS only).
                menu_handle.get_item("item_id").set_title("New title");
            });

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
        .menu(menu)
        .on_menu_event(|event| match event.menu_item_id() {
            "quit" => {
                std::process::exit(0);
            }
            "close" => {
                event.window().close().unwrap();
            }
            _ => {}
        })
        .system_tray(system_tray)
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
