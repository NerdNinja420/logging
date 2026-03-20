use std::time::{SystemTime, UNIX_EPOCH};

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const BLUE: &str = "\x1b[34m";
const YELLOW: &str = "\x1b[33m";
const RED: &str = "\x1b[91m";
const GREEN: &str = "\x1b[32m";

fn timestamp() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    // Manual conversion — no external crates needed
    let s = secs % 60;
    let m = (secs / 60) % 60;
    let h = (secs / 3600) % 24;

    let days = secs / 86400 + 719468; // days since epoch to civil date
    let era = days / 146097;
    let doe = days - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let mo = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if mo <= 2 { y + 1 } else { y };

    format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}", y, mo, d, h, m, s)
}

fn log(level: &str, level_color: &str, msg_color: &str, msg: &str) {
    eprintln!(
        "{DIM}[{}]{RESET} {level_color}{BOLD}[{level}]{RESET}: {msg_color}{msg}{RESET}",
        timestamp()
    );
}

pub fn info(msg: &str) {
    log("INFO", BLUE, RESET, msg)
}
pub fn warn(msg: &str) {
    log("WARN", YELLOW, YELLOW, msg)
}
pub fn err(msg: &str) {
    log("ERRO", RED, RED, msg)
}
pub fn rcmd(msg: &str) {
    log("RCMD", GREEN, RESET, msg)
}
