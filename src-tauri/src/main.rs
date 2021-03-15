#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use anyhow::{Context, Result};

mod cmd;
use cmd::Response;

use std::time::Instant;

use adb_cmd::list_apks;

fn main() {
  tauri::AppBuilder::new()
    .invoke_handler(|_webview, arg| {
      use cmd::Cmd::*;
      match serde_json::from_str(arg) {
        Err(e) => Err(e.to_string()),
        Ok(command) => {
          match command {
            ListApks { callback, error } => tauri::execute_promise(
              _webview,
              move || {
                let now = Instant::now();
                let apks = list_apks()?;
                let elapsed = now.elapsed().as_secs_f64();
                Ok(Response { apks, elapsed })
              },
              callback,
              error,
            ),
          }
          Ok(())
        }
      }
    })
    .build()
    .run();
}
