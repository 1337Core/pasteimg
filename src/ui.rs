// terminal ui helpers with ansi color codes
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

// wrap text with ansi color code
fn c(code: &str, s: &str) -> String {
    format!("\x1b[{}m{}\x1b[0m", code, s)
}

// ansi text styling helpers
fn bold(s: &str) -> String {
    c("1", s)
}

fn green(s: &str) -> String {
    c("32", s)
}

fn red(s: &str) -> String {
    c("31", s)
}

fn yellow(s: &str) -> String {
    c("33", s)
}

fn cyan(s: &str) -> String {
    c("36", s)
}

// style file paths in bold cyan
pub fn path(p: &str) -> String {
    bold(&cyan(p))
}

// show animated progress bar while processing runs concurrently
pub fn loading_bar() -> Arc<AtomicBool> {
    let stop_flag = Arc::new(AtomicBool::new(false));
    let stop_flag_clone = Arc::clone(&stop_flag);
    
    // elegant braille spinner animation
    let spinner_chars = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    
    // show first frame immediately to avoid pause
    print!("\r{} {}", 
           cyan(spinner_chars[0]), 
           bold("Processing clipboard image"));
    std::io::stdout().flush().unwrap();
    
    std::thread::spawn(move || {
        let mut frame = 1;
        
        while !stop_flag_clone.load(Ordering::Relaxed) {
            std::thread::sleep(std::time::Duration::from_millis(120));
            if stop_flag_clone.load(Ordering::Relaxed) { break; }
            
            print!("\r{} {}", 
                   cyan(spinner_chars[frame % spinner_chars.len()]), 
                   bold("Processing clipboard image"));
            std::io::stdout().flush().unwrap();
            frame += 1;
        }
        
        // clear the entire line when done
        print!("\r\x1b[K");
        std::io::stdout().flush().unwrap();
    });
    
    stop_flag
}

// success message with green checkmark
pub fn success(msg: &str) {
    println!("{} {}", green("✔"), msg);
}

// error message with red x mark
pub fn error(msg: &str) {
    eprintln!("{} {}", red("✖"), msg);
}

// warning message with yellow exclamation
pub fn warn(msg: &str) {
    eprintln!("{} {}", yellow("!"), msg);
}

// immediately clear the loading line
pub fn clear_loading() {
    print!("\r\x1b[K");
    std::io::stdout().flush().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn styles_work() {
        // basic smoke test for ui functions
        assert!(path("/tmp").contains("/tmp"));
        success("ok");
        warn("warn");
        error("err");
    }
}