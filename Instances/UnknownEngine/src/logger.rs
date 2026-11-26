use crate::platforms::get_timestamp;

const RESET: &str = "\x1b[0m";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LevelOfLog {
    Info,
    Warning,
    Error,
    Critical
}

impl LevelOfLog {
    fn label(self) -> &'static str {
        match self {
            LevelOfLog::Info => "[INFO]",
            LevelOfLog::Warning => "[WARN]",
            LevelOfLog::Error => "[ERR!]",
            LevelOfLog::Critical => "[CRIT]"
        }
    }

    fn color(self) -> &'static str {
        match self {
            LevelOfLog::Info => "\x1b[36m",
            LevelOfLog::Warning => "\x1b[33m",
            LevelOfLog::Error => "\x1b[31m",
            LevelOfLog::Critical => "\x1b[1;31m"   
        }
    }
}

#[track_caller]
pub fn log(level: LevelOfLog, text: String) {
    let location = std::panic::Location::caller();
    
    let file = std::path::Path::new(location.file()).file_name()
    .and_then(|name| name.to_str()).unwrap_or("unknown");

    let timestamp = get_timestamp();

    println!("{}{} {}{} | {}: {}", level.color(), timestamp, level.label(), RESET, file, text);
    if level == LevelOfLog::Critical {
        panic!("{} {} | {}: {}", timestamp, level.label(), file, text);
    }
}