pub fn print(warn: u8, text: &str) {
    let level = match warn {
        0 => "[INFO]",
        1 => "[WARN]",
        2 => "[ERR!]",
        3 => "[CRIT]",
        _ => "[NULL]"
    };
    
    println!("{} | {}", level, text);
}