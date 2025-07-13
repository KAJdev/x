use console::{Key, Term, style};
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct StreamingSpinner {
    current_text: Arc<Mutex<String>>,
    running: Arc<Mutex<bool>>,
    frames: Vec<&'static str>,
}

impl StreamingSpinner {
    pub fn new() -> Self {
        Self {
            current_text: Arc::new(Mutex::new(String::new())),
            running: Arc::new(Mutex::new(false)),
            frames: vec!["⣷", "⣯", "⣟", "⡿", "⢿", "⣻", "⣽", "⣾"],
        }
    }

    pub fn start(&self) -> Arc<Mutex<String>> {
        *self.running.lock().unwrap() = true;

        let current_text = Arc::clone(&self.current_text);
        let running = Arc::clone(&self.running);
        let frames = self.frames.clone();

        let text_clone = Arc::clone(&current_text);

        thread::spawn(move || {
            let mut frame_index = 0usize;
            let mut last_lines = 0;

            loop {
                let current = current_text.lock().unwrap().clone();
                let frame = frames[frame_index % frames.len()];

                // clear the current line
                print!("\r\x1b[2K");

                // clear previous lines if any
                if last_lines > 1 {
                    for _ in 1..last_lines {
                        print!("\x1b[1A\x1b[2K");
                    }
                }

                if !*running.lock().unwrap() {
                    // we're done here, and we've already cleaned up our previous lines,
                    // so just exit the loop and kill the thread
                    break;
                }

                let visual_content = format!("{} {}", frame, current);
                let visual_width = visual_content.chars().count(); // This counts actual visible characters

                let term_width = Term::stdout()
                    .size_checked()
                    .map(|size| size.1 as usize)
                    .unwrap_or(80);

                let current_lines = if term_width > 0 && !current.is_empty() {
                    std::cmp::max(1, (visual_width + term_width - 1) / term_width)
                        + visual_content.chars().filter(|&c| c == '\n').count()
                } else {
                    1
                };

                print!("{} {}", style(frame).cyan().bold(), style(&current).white());

                io::stdout().flush().unwrap();

                last_lines = current_lines;
                thread::sleep(Duration::from_millis(50));
                frame_index += 1;
            }
        });

        text_clone
    }

    pub fn stop(&self) {
        *self.running.lock().unwrap() = false;
        thread::sleep(Duration::from_millis(150)); // give a moment for cleanup
    }

    pub fn update_text(&self, text: &str) {
        *self.current_text.lock().unwrap() = text.to_string();
    }
}

pub fn prompt_single_char(prompt: &str) -> io::Result<bool> {
    let term = Term::stdout();
    print!("{}", prompt);
    io::stdout().flush()?;

    loop {
        if let Ok(key) = term.read_key() {
            match key {
                Key::Char('y') | Key::Char('Y') => {
                    print!("y");
                    io::stdout().flush()?;
                    return Ok(true);
                }
                Key::Char('n') | Key::Char('N') | Key::Escape => {
                    print!("n");
                    io::stdout().flush()?;
                    return Ok(false);
                }
                Key::Enter => {
                    print!("n");
                    io::stdout().flush()?;
                    return Ok(false);
                }
                _ => {
                    continue;
                }
            }
        }
    }
}
