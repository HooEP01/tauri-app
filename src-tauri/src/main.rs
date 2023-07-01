// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, SystemTrayEvent};

// Window Menu
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

// Window
use tauri::WindowBuilder;

// Multiple Windows

// System Tray
use tauri::{SystemTray, SystemTrayMenu, SystemTrayMenuItem};

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

#[tauri::command]
fn open_docs(handle: tauri::AppHandle) {
    let docs_window = tauri::WindowBuilder::new(
        &handle,
        "wonderland", /* the unique window label */
        tauri::WindowUrl::External("https://tauri.app/".parse().unwrap()),
    )
    .build()
    .unwrap();
}

#[tauri::command]
async fn close_splashscreen(window: tauri::Window) {
    // Close splashscreen
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    // Show main window
    window.get_window("main").unwrap().show().unwrap();
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

    let tray_quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(tray_quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(tray_hide);
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
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
                menu_handle.get_item("quit").set_title("New title").unwrap();
            });

            // Mutliple Windows
            let docs_window = tauri::WindowBuilder::new(
                app,
                "external_app", /* the unique window label */
                tauri::WindowUrl::External("https://tauri.app/".parse().unwrap()),
            )
            .build()?;

            let local_window = tauri::WindowBuilder::new(
                app,
                "local_app",
                tauri::WindowUrl::App("index.html".into()),
            )
            .build()?;

            let handle = app.handle();

            // let value= open_docs(handle.clone());

            std::thread::spawn(move || {
                let local_window_2 = tauri::WindowBuilder::new(
                    &handle,
                    "local",
                    tauri::WindowUrl::App("index.html".into()),
                )
                .build();
            });

            let splashscreen_window = app.get_window("splashscreen").unwrap();
            let main_window = app.get_window("main").unwrap();
            // we perform the initialization code on a new task so the app doesn't freeze
            tauri::async_runtime::spawn(async move {
                // initialize your app here instead of sleeping :)
                println!("Initializing...");
                std::thread::sleep(std::time::Duration::from_secs(2));
                println!("Done initializing.");

                // After it's done, close the splashscreen and display the main window
                splashscreen_window.close().unwrap();
                main_window.show().unwrap();
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
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a left click");
            }
            SystemTrayEvent::RightClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a right click");
            }
            SystemTrayEvent::DoubleClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a double click");
            }
            SystemTrayEvent::MenuItemClick { id, .. } => {
                let item_handle = app.tray_handle().get_item(&id);
                match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    "hide" => {
                        let window = app.get_window("main").unwrap();
                        window.hide().unwrap();
                        item_handle.set_title("Show").unwrap();
                    }
                    "show" => {
                        print!("showing window");
                    }
                    _ => {}
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![close_splashscreen])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });

    // let docs_window = tauri::WindowBuilder::new(
    //     &app,
    //     "external_outside", /* the unique window label */
    //     tauri::WindowUrl::External("https://tauri.app/".parse().unwrap()),
    // )
    // .build()
    // .expect("failed to build window");

    // let local_window =
    //     tauri::WindowBuilder::new(
    //         &app,
    //         "local_outside",
    //         tauri::WindowUrl::App("index.html".into())
    //     )
    //         .build()
    //         .expect("failed to build window");
}
