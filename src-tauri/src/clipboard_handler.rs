use clipboard_rs::{Clipboard, ClipboardContext};
use std::error::Error;

pub struct ClipboardHandler {
    ctx: ClipboardContext,
}

impl ClipboardHandler {
    pub fn new() -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(Self {
            ctx: ClipboardContext::new()?,
        })
    }

    pub fn _set_text(&mut self, text: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.ctx.set_text(text.to_string())?;
        Ok(())
    }

    pub fn set_html(&mut self, html: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.ctx.set_html(html.to_string())?;
        Ok(())
    }

    pub fn clear(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.ctx.clear()?;
        Ok(())
    }
}