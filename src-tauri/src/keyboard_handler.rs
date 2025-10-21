// use crate::clipboard_handler::ClipboardHandler;
// use crate::config::Database;
// use enigo::{Enigo, Key, KeyboardControllable};
// use rdev::{listen, Event, EventType, Key as RdevKey};
// use std::sync::{Arc, Mutex};
// use std::thread;
// use std::time::Duration;

// const MAX_TRIGGER_LENGTH: usize = 100;

// pub struct KeyboardHandler {
//     buffer: Arc<Mutex<String>>,
//     db: Arc<Database>,
//     clipboard: Arc<Mutex<ClipboardHandler>>,
//     enigo: Arc<Mutex<Enigo>>,
// }

// impl KeyboardHandler {
//     pub fn new(db: Arc<Database>) -> Self {
//         Self {
//             buffer: Arc::new(Mutex::new(String::new())),
//             db,
//             clipboard: Arc::new(Mutex::new(ClipboardHandler::new())),
//             enigo: Arc::new(Mutex::new(Enigo::new())),
//         }
//     }

//     /// Start listening to keyboard events
//     pub fn start_listening(self: Arc<Self>) {
//         println!("Keyboard listener started...");

//         thread::spawn(move || {
//             if let Err(error) = listen(move |event| {
//                 self.handle_event(event);
//             }) {
//                 eprintln!("Error in keyboard listener: {:?}", error);
//             }
//         });
//     }

//     /// Handle keyboard events
//     fn handle_event(&self, event: Event) {
//         match event.event_type {
//             EventType::KeyRelease(key) => {
//                 self.handle_key_release(key);
//             }

//             EventType::ButtonPress(_) => {
//                 self.buffer.lock().unwrap().clear();
//             }

//             _ => {}
//         }
//     }

//     /// Handle individual key releases
//     fn handle_key_release(&self, key: RdevKey) {
//         let mut buffer = self.buffer.lock().unwrap();

//         match key {
//             // Handle printable characters
//             RdevKey::KeyA => buffer.push('a'),
//             RdevKey::KeyB => buffer.push('b'),
//             RdevKey::KeyC => buffer.push('c'),
//             RdevKey::KeyD => buffer.push('d'),
//             RdevKey::KeyE => buffer.push('e'),
//             RdevKey::KeyF => buffer.push('f'),
//             RdevKey::KeyG => buffer.push('g'),
//             RdevKey::KeyH => buffer.push('h'),
//             RdevKey::KeyI => buffer.push('i'),
//             RdevKey::KeyJ => buffer.push('j'),
//             RdevKey::KeyK => buffer.push('k'),
//             RdevKey::KeyL => buffer.push('l'),
//             RdevKey::KeyM => buffer.push('m'),
//             RdevKey::KeyN => buffer.push('n'),
//             RdevKey::KeyO => buffer.push('o'),
//             RdevKey::KeyP => buffer.push('p'),
//             RdevKey::KeyQ => buffer.push('q'),
//             RdevKey::KeyR => buffer.push('r'),
//             RdevKey::KeyS => buffer.push('s'),
//             RdevKey::KeyT => buffer.push('t'),
//             RdevKey::KeyU => buffer.push('u'),
//             RdevKey::KeyV => buffer.push('v'),
//             RdevKey::KeyW => buffer.push('w'),
//             RdevKey::KeyX => buffer.push('x'),
//             RdevKey::KeyY => buffer.push('y'),
//             RdevKey::KeyZ => buffer.push('z'),
//             RdevKey::Num0 => buffer.push('0'),
//             RdevKey::Num1 => buffer.push('1'),
//             RdevKey::Num2 => buffer.push('2'),
//             RdevKey::Num3 => buffer.push('3'),
//             RdevKey::Num4 => buffer.push('4'),
//             RdevKey::Num5 => buffer.push('5'),
//             RdevKey::Num6 => buffer.push('6'),
//             RdevKey::Num7 => buffer.push('7'),
//             RdevKey::Num8 => buffer.push('8'),
//             RdevKey::Num9 => buffer.push('9'),
//             RdevKey::Space => buffer.push(' '),
//             RdevKey::Minus => buffer.push('-'),
//             RdevKey::Equal => buffer.push('='),
//             RdevKey::LeftBracket => buffer.push('['),
//             RdevKey::RightBracket => buffer.push(']'),
//             RdevKey::SemiColon => buffer.push(';'),
//             RdevKey::Quote => buffer.push('\''),
//             RdevKey::BackSlash => buffer.push('\\'),
//             RdevKey::Comma => buffer.push(','),
//             RdevKey::Dot => buffer.push('.'),
//             RdevKey::Slash => buffer.push('/'),
//             RdevKey::Grave => buffer.push('`'),

//             // Handle special keys
//             RdevKey::Backspace => {
//                 buffer.pop();
//             }
//             RdevKey::Return | RdevKey::Tab => {
//                 buffer.clear();
//             }
//             RdevKey::Escape => {
//                 buffer.clear();
//             }

//             RdevKey::LeftArrow
//             | RdevKey::RightArrow
//             | RdevKey::UpArrow
//             | RdevKey::DownArrow
//             | RdevKey::Home
//             | RdevKey::End
//             | RdevKey::PageUp
//             | RdevKey::PageDown
//             | RdevKey::Delete => {
//                 buffer.clear();
//             }

//             _ => {
//                 // Ignore other keys (modifiers, function keys, etc.)
//                 return;
//             }
//         }

//         // Limit buffer size
//         if buffer.len() > MAX_TRIGGER_LENGTH {
//             buffer.remove(0);
//         }

//         // Check for snippet matches
//         let buffer_str = buffer.clone();
//         drop(buffer); // Release lock before potentially blocking operations

//         self.check_and_replace_snippet(&buffer_str);
//     }

//     /// Check if buffer contains a snippet trigger and replace it
//     fn check_and_replace_snippet(&self, buffer: &str) {
//         // Get all snippets from database
//         let snippets = match self.db.get_all_snippets() {
//             Ok(snippets) => snippets,
//             Err(e) => {
//                 eprintln!("Failed to get snippets: {}", e);
//                 return;
//             }
//         };

//         // Check if any snippet trigger matches the end of the buffer
//         for snippet in snippets {
//             if buffer.ends_with(&snippet.trigger) {
//                 println!("Snippet triggered: {}", snippet.trigger);

//                 // Perform replacement
//                 self.replace_trigger_with_content(&snippet.trigger, &snippet.content, &snippet.id);

//                 // Clear buffer after successful replacement
//                 self.buffer.lock().unwrap().clear();
//                 break;
//             }
//         }
//     }

//     /// Replace trigger text with snippet content
//     fn replace_trigger_with_content(&self, trigger: &str, content: &str, snippet_id: &str) {
//         // Small delay to ensure the last key press is processed
//         thread::sleep(Duration::from_millis(50));

//         // Delete the trigger text by simulating backspace
//         let mut enigo = self.enigo.lock().unwrap();
//         for _ in 0..trigger.len() {
//             enigo.key_click(Key::Backspace);
//             thread::sleep(Duration::from_millis(10));
//         }

//         // Release the enigo lock before clipboard operations
//         drop(enigo);

//         // Copy content to clipboard
//         {
//             let mut clipboard = self.clipboard.lock().unwrap();
//             if let Err(e) = clipboard.set_text(content) {
//                 eprintln!("Failed to set clipboard: {}", e);
//                 return;
//             }
//         }

//         // Small delay to ensure clipboard is set
//         thread::sleep(Duration::from_millis(50));

//         // Paste from clipboard using Ctrl+V (Cmd+V on macOS)
//         let mut enigo = self.enigo.lock().unwrap();
        
//         #[cfg(target_os = "macos")]
//         {
//             enigo.key_down(Key::Meta);
//             enigo.key_click(Key::Layout('v'));
//             enigo.key_up(Key::Meta);
//         }

//         #[cfg(not(target_os = "macos"))]
//         {
//             enigo.key_down(Key::Control);
//             enigo.key_click(Key::Layout('v'));
//             enigo.key_up(Key::Control);
//         }

//         drop(enigo);

//         // Increment usage count
//         if let Err(e) = self.db.increment_usage(snippet_id) {
//             eprintln!("Failed to increment usage count: {}", e);
//         }

//         println!("Snippet replaced successfully: {} -> {} chars", trigger, content.len());
//     }

//     /// Get current buffer content (for debugging)
//     #[allow(dead_code)]
//     pub fn get_buffer(&self) -> String {
//         self.buffer.lock().unwrap().clone()
//     }

//     /// Clear the buffer
//     #[allow(dead_code)]
//     pub fn clear_buffer(&self) {
//         self.buffer.lock().unwrap().clear();
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_buffer_management() {
//         let db = Arc::new(Database::new().unwrap());
//         let handler = KeyboardHandler::new(db);
        
//         handler.buffer.lock().unwrap().push_str("test");
//         assert_eq!(handler.get_buffer(), "test");
        
//         handler.clear_buffer();
//         assert_eq!(handler.get_buffer(), "");
//     }

//     #[test]
//     fn test_max_buffer_length() {
//         let db = Arc::new(Database::new().unwrap());
//         let handler = KeyboardHandler::new(db);
        
//         let mut buffer = handler.buffer.lock().unwrap();
//         for _ in 0..MAX_TRIGGER_LENGTH + 10 {
//             buffer.push('a');
//             if buffer.len() > MAX_TRIGGER_LENGTH {
//                 buffer.remove(0);
//             }
//         }
        
//         assert_eq!(buffer.len(), MAX_TRIGGER_LENGTH);
//     }
// }