use adb_cmd::list_apks;

fn main() {
  match list_apks() {
    Ok(apks) => println!("{:#?}", apks),
    Err(e) => println!("{:#?}", e),
  }
}
