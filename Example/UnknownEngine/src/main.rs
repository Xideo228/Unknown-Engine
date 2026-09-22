/*use UnknownEngine_Platform::{
    WindowManager,
    WindowSettings
};*/
use std::io::{self, Write};

fn main() {
    //let window = WindowManager::create(WindowSettings::default());
    print!("Нажмите Enter, чтобы продолжить...");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    println!("Продолжаем выполнение!");
}