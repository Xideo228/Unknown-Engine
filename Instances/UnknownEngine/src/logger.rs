pub enum LevelOfLog {
    Info,
    Warning,
    Error,
    Critical
}

pub fn print(warn: LevelOfLog, text: String) {
    let level = match warn {
        LevelOfLog::Info => "[INFO]",
        LevelOfLog::Warning => "[WARN]",
        LevelOfLog::Error => "[ERR!]",
        LevelOfLog::Critical => "[CRIT]"
    };
    
    println!("{} | {}", level, text);
}