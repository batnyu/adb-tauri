use anyhow::{bail, Context, Result};
use rayon::prelude::*;
use regex::bytes::Regex;
use serde::Serialize;
use std::process::{Command, Output};

#[macro_use]
extern crate lazy_static;

lazy_static! {
  static ref VERSION_NAME_REGEX: Regex = Regex::new("versionName=(.+)").unwrap();
  static ref VERSION_CODE_REGEX: Regex = Regex::new("versionCode=(.+)").unwrap();
}

#[derive(Serialize, Debug)]
pub struct Apk {
  package: String,
  version_name: Option<String>,
  version_code: Option<String>,
}

impl Apk {
  pub fn new(package: String, version_name: Option<String>, version_code: Option<String>) -> Apk {
    Apk {
      package,
      version_name,
      version_code,
    }
  }
}

pub fn list_apks() -> Result<Vec<Apk>> {
  let apks_names = get_apks_names()?;
  let apks = String::from_utf8_lossy(&apks_names)
    .par_lines()
    .map(create_apk)
    .collect::<Result<Vec<Apk>>>()?;

  Ok(apks)
}

fn create_apk(name: &str) -> Result<Apk> {
  let name = name.trim_start_matches("package:").to_string();
  let dumpsys = get_dumpsys(&name)?;
  let version_name = get_version_name(&dumpsys);
  let version_code = get_version_code(&dumpsys);
  Ok(Apk::new(name, version_name, version_code))
}

fn get_dumpsys(package_name: &str) -> Result<Vec<u8>> {
  Command::new("adb")
    .args(&["shell", "dumpsys package", package_name])
    .output()
    .with_context(|| format!("could not dumpsys {}", package_name))
    .map(|f| f.stdout)
}

fn get_apks_names() -> Result<Vec<u8>> {
  match Command::new("adb")
    .args(&["shell", "pm list packages -3"])
    .output()
    // .with_context(|| format!("could not run cmd"))
  {
    Ok(Output { stdout, stderr, status, .. }) => {
      if !status.success() {
        bail!("stderr: {}", String::from_utf8_lossy(&stderr));
      } else {
        Ok(stdout)
      }
    }
    Err(e) => bail!(e),
  }
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

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
