#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            // Build the main window with navigation interception
            let _webview = tauri::WebviewWindowBuilder::new(
                app,
                "main",
                tauri::WebviewUrl::External("https://gbc.chat".parse().unwrap()),
            )
            .title("gbc.chat")
            .inner_size(1200.0, 800.0)
            .min_inner_size(800.0, 500.0)
            .resizable(true)
            .on_navigation(|url| {
                let host = url.host_str().unwrap_or("");
                if host == "gbc.chat"
                    || host == "www.gbc.chat"
                    || host.ends_with(".gbc.chat")
                    || host == "localhost"
                    || host.is_empty()
                {
                    return true;
                }
                let _ = open::that(url.as_str());
                false
            })
            .build()?;

            // Check for updates silently in the background
            let handle = app.handle().clone();
            std::thread::spawn(move || {
                // Small delay so the window loads first
                std::thread::sleep(std::time::Duration::from_secs(3));

                tauri::async_runtime::block_on(async move {
                    match tauri_plugin_updater::UpdaterExt::updater(&handle) {
                        Ok(updater) => {
                            match updater.check().await {
                                Ok(Some(update)) => {
                                    println!("[Updater] Update found: v{}", update.version);
                                    match update.download_and_install(|_, _| {}, || {}).await {
                                        Ok(_) => {
                                            println!("[Updater] Update installed, restarting...");
                                            handle.restart();
                                        }
                                        Err(e) => eprintln!("[Updater] Install failed: {}", e),
                                    }
                                }
                                Ok(None) => println!("[Updater] App is up to date"),
                                Err(e) => eprintln!("[Updater] Check failed: {}", e),
                            }
                        }
                        Err(e) => eprintln!("[Updater] Init failed: {}", e),
                    }
                });
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running gbc.chat");
}
