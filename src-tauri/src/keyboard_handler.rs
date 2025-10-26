use crate::clipboard_handler::{ClipboardHandler};
use crate::config::Database;
use enigo::{Enigo, Key, Keyboard, Settings};
use rdev::{listen, Event, EventType, Key as RdevKey};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

const MAX_TRIGGER_LENGTH: usize = 100;

pub struct KeyboardHandler {
    buffer: Arc<Mutex<String>>,
    last_key_time: Arc<Mutex<Instant>>,
    db: Arc<Database>,
    clipboard: Arc<Mutex<ClipboardHandler>>,
    enigo: Arc<Mutex<Enigo>>,
}

impl KeyboardHandler {
    pub fn new(db: Arc<Database>) -> Self {
        Self {
            buffer: Arc::new(Mutex::new(String::new())),
            last_key_time: Arc::new(Mutex::new(Instant::now())),
            db,
            clipboard: Arc::new(Mutex::new(ClipboardHandler::new().expect("Failed to init clipboard"))),
            enigo: Arc::new(Mutex::new(Enigo::new(&Settings::default()).unwrap())),
        }
    }

    pub fn start_listening(self: Arc<Self>) {
        thread::spawn(move || {
            if let Err(error) = listen(move |event| {
                self.handle_event(event);
            }) {
                eprintln!("Error in keyboard listener: {:?}", error);
            }
        });
    }

    fn handle_event(&self, event: Event) {
        match event.event_type {
            EventType::KeyRelease(key) => {
                self.handle_key_release(key);
            }
            EventType::ButtonPress(_) => {
                self.buffer.lock().unwrap().clear();
            }
            _ => {}
        }
    }

    fn key_to_char(&self, key: &RdevKey) -> Option<char> {
        match key {
            RdevKey::KeyA => Some('a'),
            RdevKey::KeyB => Some('b'),
            RdevKey::KeyC => Some('c'),
            RdevKey::KeyD => Some('d'),
            RdevKey::KeyE => Some('e'),
            RdevKey::KeyF => Some('f'),
            RdevKey::KeyG => Some('g'),
            RdevKey::KeyH => Some('h'),
            RdevKey::KeyI => Some('i'),
            RdevKey::KeyJ => Some('j'),
            RdevKey::KeyK => Some('k'),
            RdevKey::KeyL => Some('l'),
            RdevKey::KeyM => Some('m'),
            RdevKey::KeyN => Some('n'),
            RdevKey::KeyO => Some('o'),
            RdevKey::KeyP => Some('p'),
            RdevKey::KeyQ => Some('q'),
            RdevKey::KeyR => Some('r'),
            RdevKey::KeyS => Some('s'),
            RdevKey::KeyT => Some('t'),
            RdevKey::KeyU => Some('u'),
            RdevKey::KeyV => Some('v'),
            RdevKey::KeyW => Some('w'),
            RdevKey::KeyX => Some('x'),
            RdevKey::KeyY => Some('y'),
            RdevKey::KeyZ => Some('z'),
            RdevKey::Num0 => Some('0'),
            RdevKey::Num1 => Some('1'),
            RdevKey::Num2 => Some('2'),
            RdevKey::Num3 => Some('3'),
            RdevKey::Num4 => Some('4'),
            RdevKey::Num5 => Some('5'),
            RdevKey::Num6 => Some('6'),
            RdevKey::Num7 => Some('7'),
            RdevKey::Num8 => Some('8'),
            RdevKey::Num9 => Some('9'),
            RdevKey::Minus => Some('-'),
            RdevKey::Equal => Some('='),
            RdevKey::LeftBracket => Some('['),
            RdevKey::RightBracket => Some(']'),
            RdevKey::SemiColon => Some(';'),
            RdevKey::Quote => Some('\''),
            RdevKey::BackSlash => Some('\\'),
            RdevKey::Comma => Some(','),
            RdevKey::Dot => Some('.'),
            RdevKey::Slash => Some('/'),
            RdevKey::BackQuote => Some('`'),
            _ => None,
        }
    }

    fn handle_key_release(&self, key: RdevKey) {
        let mut buffer = self.buffer.lock().unwrap();
        let mut last_key_time = self.last_key_time.lock().unwrap();
        let now = Instant::now();

        let time_delay_ms = match self.db.get_snippet_settings() {
            Ok(delay) => delay.time_delay_ms as u64,
            Err(_) => 200,
        };
        let buffer_timeout = Duration::from_millis(time_delay_ms);

        if buffer.len() > 0 && now.duration_since(*last_key_time) >= buffer_timeout {
            let buffer_str = buffer.clone();
            drop(buffer);
            drop(last_key_time);
            
            self.check_and_replace_snippet(&buffer_str);
            
            self.buffer.lock().unwrap().clear();
            return;
        }

        *last_key_time = now;

        match key {
            RdevKey::KeyA | RdevKey::KeyB | RdevKey::KeyC | RdevKey::KeyD | RdevKey::KeyE
            | RdevKey::KeyF | RdevKey::KeyG | RdevKey::KeyH | RdevKey::KeyI | RdevKey::KeyJ
            | RdevKey::KeyK | RdevKey::KeyL | RdevKey::KeyM | RdevKey::KeyN | RdevKey::KeyO
            | RdevKey::KeyP | RdevKey::KeyQ | RdevKey::KeyR | RdevKey::KeyS | RdevKey::KeyT
            | RdevKey::KeyU | RdevKey::KeyV | RdevKey::KeyW | RdevKey::KeyX | RdevKey::KeyY
            | RdevKey::KeyZ | RdevKey::Num0 | RdevKey::Num1 | RdevKey::Num2 | RdevKey::Num3
            | RdevKey::Num4 | RdevKey::Num5 | RdevKey::Num6 | RdevKey::Num7 | RdevKey::Num8
            | RdevKey::Num9 | RdevKey::Minus | RdevKey::Equal | RdevKey::LeftBracket 
            | RdevKey::RightBracket | RdevKey::SemiColon | RdevKey::Quote
            | RdevKey::BackSlash | RdevKey::Comma | RdevKey::Dot | RdevKey::Slash
            | RdevKey::BackQuote => {
                if let Some(ch) = self.key_to_char(&key) {
                    buffer.push(ch);
                    *last_key_time = now;
                }
            }

            RdevKey::Backspace => {
                buffer.pop();
                *last_key_time = now;
            }

            RdevKey::Return | RdevKey::Tab | RdevKey::Escape | RdevKey::LeftArrow
            | RdevKey::RightArrow | RdevKey::UpArrow | RdevKey::DownArrow | RdevKey::Home
            | RdevKey::End | RdevKey::PageUp | RdevKey::PageDown | RdevKey::Delete => {
                buffer.clear();
                return;
            }

            _ => {
                return;
            }
        }

        if buffer.len() > MAX_TRIGGER_LENGTH {
            buffer.remove(0);
        }

        let buffer_str = buffer.clone();
        let last_time = *last_key_time;
        drop(buffer);
        drop(last_key_time);


        let handler = Arc::new(self.clone_for_timer());
        
        thread::spawn(move || {
            thread::sleep(buffer_timeout);
            
            let current_last_time = *handler.last_key_time.lock().unwrap();
            if current_last_time == last_time {
                handler.check_and_replace_snippet(&buffer_str);
            }
        });
    }

    fn clone_for_timer(&self) -> Self {
        Self {
            buffer: Arc::clone(&self.buffer),
            last_key_time: Arc::clone(&self.last_key_time),
            db: Arc::clone(&self.db),
            clipboard: Arc::clone(&self.clipboard),
            enigo: Arc::clone(&self.enigo),
        }
    }

    fn check_and_replace_snippet(&self, buffer: &str) {
        if buffer.is_empty() {
            return;
        }

        let snippets = match self.db.get_all_snippets() {
            Ok(snippets) => snippets,
            Err(e) => {
                eprintln!("Failed to get snippets: {}", e);
                return;
            }
        };

        for snippet in snippets {
            let trigger = &snippet.trigger;

            if buffer.ends_with(trigger) {
                self.replace_trigger_with_content(trigger, &snippet.content, &snippet.id, snippet.is_html);

                self.buffer.lock().unwrap().clear();
                break;
            }
        }
    }

    fn replace_trigger_with_content(&self, trigger: &str, content: &str, snippet_id: &str, is_html: bool) {
        thread::sleep(Duration::from_millis(100));

        {
            let mut enigo = self.enigo.lock().unwrap();
            for i in 0..trigger.len() {
                enigo.key(Key::Backspace, enigo::Direction::Click).unwrap();
                thread::sleep(Duration::from_millis(5));

                if (i + 1) % 10 == 0 {
                    thread::sleep(Duration::from_millis(10));
                }
            }
        }

        thread::sleep(Duration::from_millis(50));

        {
            let mut clipboard = self.clipboard.lock().unwrap();
            let result = if is_html {
                clipboard.set_html(content)
            } else {
                clipboard.set_text(content)
            };

            if let Err(e) = result {
                eprintln!("Failed to set clipboard: {}", e);
                return;
            }
        }

        thread::sleep(Duration::from_millis(10));

        {
            let mut enigo = self.enigo.lock().unwrap();

            #[cfg(target_os = "macos")]
            {
                enigo.key(Key::Meta, enigo::Direction::Press).unwrap();
                thread::sleep(Duration::from_millis(10));
                enigo.key(Key::V, enigo::Direction::Click).unwrap();
                thread::sleep(Duration::from_millis(10));
                enigo.key(Key::Meta, enigo::Direction::Release).unwrap();
            }

            #[cfg(not(target_os = "windows"))]
            {
                enigo.key(Key::Control, enigo::Direction::Press).unwrap();
                thread::sleep(Duration::from_millis(10));
                enigo.key(Key::V, enigo::Direction::Click).unwrap();
                thread::sleep(Duration::from_millis(10));
                enigo.key(Key::Control, enigo::Direction::Release).unwrap();
            }
        }

        thread::sleep(Duration::from_millis(50));

        let mut clipboard = self.clipboard.lock().unwrap();
        clipboard.clear().unwrap_or_else(|e| {
            eprintln!("Failed to clear clipboard: {}", e);
        });

        thread::sleep(Duration::from_millis(50));

        if let Err(e) = self.db.increment_usage(snippet_id) {
            eprintln!("Failed to increment usage count: {}", e);
        }
    }

    #[allow(dead_code)]
    pub fn get_buffer(&self) -> String {
        self.buffer.lock().unwrap().clone()
    }

    #[allow(dead_code)]
    pub fn clear_buffer(&self) {
        self.buffer.lock().unwrap().clear();
    }
}