use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

const SPINNER_FRAMES: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
const SPINNER_MESSAGE: &str = "Processing clipboard image";

fn color(code: &str, content: &str) -> String {
    format!("\x1b[{}m{}\x1b[0m", code, content)
}

fn bold(content: &str) -> String {
    color("1", content)
}

fn green(content: &str) -> String {
    color("32", content)
}

fn red(content: &str) -> String {
    color("31", content)
}

fn yellow(content: &str) -> String {
    color("33", content)
}

fn cyan(content: &str) -> String {
    color("36", content)
}

pub fn path(p: &str) -> String {
    bold(&cyan(p))
}

pub fn loading_bar() -> Arc<AtomicBool> {
    let stop_flag = Arc::new(AtomicBool::new(false));
    let stop_flag_clone = Arc::clone(&stop_flag);

    draw_spinner_frame(0);

    std::thread::spawn(move || {
        let mut frame = 1;

        while !stop_flag_clone.load(Ordering::Relaxed) {
            std::thread::sleep(std::time::Duration::from_millis(120));
            if stop_flag_clone.load(Ordering::Relaxed) {
                break;
            }

            draw_spinner_frame(frame % SPINNER_FRAMES.len());
            frame += 1;
        }

        clear_loading();
    });

    stop_flag
}

fn draw_spinner_frame(frame: usize) {
    print!(
        "\r{} {}",
        cyan(SPINNER_FRAMES[frame]),
        bold(SPINNER_MESSAGE)
    );
    std::io::stdout().flush().expect("failed to flush stdout");
}

pub fn success(msg: &str) {
    println!("{} {}", green("✔"), msg);
}

pub fn error(msg: &str) {
    eprintln!("{} {}", red("✖"), msg);
}

pub fn warn(msg: &str) {
    eprintln!("{} {}", yellow("!"), msg);
}

pub fn clear_loading() {
    print!("\r\x1b[K");
    std::io::stdout().flush().expect("failed to flush stdout");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_styling_contains_original_path() {
        assert!(path("/tmp").contains("/tmp"));
    }

    #[test]
    fn spinner_stop_flag_can_be_set() {
        let flag = loading_bar();
        flag.store(true, Ordering::Relaxed);
        clear_loading();
    }
}
