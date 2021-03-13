#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;
use anyhow::{Context, Result};
use cmd::{Apk, Response};

use regex::bytes::Regex;
use std::process::{Command};
use rayon::prelude::*;
use std::time::{Instant};

#[macro_use]
extern crate lazy_static;

lazy_static! {
  static ref VERSION_NAME_REGEX: Regex = Regex::new("versionName=(.+)").unwrap();
  static ref VERSION_CODE_REGEX: Regex = Regex::new("versionCode=(.+)").unwrap();
}

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

                let get_apk_info = |name:&str| -> Result<Apk> {
                  let name = name.trim_start_matches("package:").to_string();
                  let dumpsys = get_dumpsys(&name)?;
                  let version_name = get_version_name(&dumpsys);
                  let version_code = get_version_code(&dumpsys);
                  Ok(Apk::new(name, version_name, version_code))
                };

                let stdout = get_list_apks()?;
                let apks = String::from_utf8(stdout)?.par_lines()
                .map(get_apk_info).collect::<Result<Vec<Apk>>>()?;

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

fn get_dumpsys(package_name: &str) -> Result<Vec<u8>> {
  Command::new("adb")
    .args(&["shell", "dumpsys package", package_name])
    .output()
    .with_context(|| format!("could not dumpsys {}", package_name)).map(|f| f.stdout)
}

fn get_list_apks() -> Result<Vec<u8>> {
  Command::new("adb")
                  .args(&["shell", "pm list packages -3"])
                  .output()
                  .with_context(|| format!("could not run cmd")).map(|f| f.stdout)
}

fn get_version_name(dumpsys: &[u8]) -> Option<String> {
  let captures = VERSION_NAME_REGEX.captures(dumpsys)?;
  let version_name = captures.get(1)?;
  String::from_utf8(version_name.as_bytes().to_vec()).ok()
}

fn get_version_code(dumpsys: &[u8]) -> Option<String> {
  let captures = VERSION_NAME_REGEX.captures(dumpsys)?;
  let version_code = captures.get(1)?;
  String::from_utf8(version_code.as_bytes().to_vec()).ok()
}
