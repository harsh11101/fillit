// use clipboard_rs::{Clipboard, ClipboardContext, formats::Html};
// use std::error::Error;

// struct ClipboardHandler {
//     ctx: ClipboardContext,
// }

// impl ClipboardHandler {
//     /// Create a new ClipboardHandler
//     fn new() -> Result<Self, Box<dyn Error>> {
//         Ok(Self {
//             ctx: ClipboardContext::new()?,
//         })
//     }

//     /// Set plain text to the clipboard
//     fn set_text(&mut self, text: &str) -> Result<(), Box<dyn Error>> {
//         self.ctx.set_text(text)?;
//         Ok(())
//     }

//     /// Set HTML content to the clipboard
//     fn set_html(&mut self, html: &str) -> Result<(), Box<dyn Error>> {
//         let html_content = Html {
//             html: html.to_string(),
//             base_url: None,
//         };
//         self.ctx.set(html_content)?;
//         Ok(())
//     }
// }