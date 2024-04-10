pub mod db;
fn main() {
    println!("Hello, world!");
    // queryを実行する
    db::query().unwrap();
}
