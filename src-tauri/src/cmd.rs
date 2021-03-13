use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
  ListApks { callback: String, error: String },
}

#[derive(Serialize)]
pub struct Response {
  pub apks: Vec<Apk>,
  pub elapsed: f64
}

#[derive(Serialize)]
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
      version_code
    }
  }
}

// #[derive(Debug, Clone)]
// pub struct CommandError<'a> {
//   message: &'a str,
// }

// impl<'a> CommandError<'a> {
//   pub fn new(message: &'a str) -> Self {
//     Self { message }
//   }
// }

// impl<'a> std::fmt::Display for CommandError<'a> {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     write!(f, "{}", self.message)
//   }
// }

// impl<'a> std::error::Error for CommandError<'a> {}
