use std::borrow::Cow;

use rustyline::config::Configurer;
use rustyline::highlight::Highlighter;
use rustyline::{ColorMode, Editor, Result};
use rustyline_derive::{Completer, Helper, Hinter, Validator};

#[derive(Completer, Helper, Hinter, Validator)]
struct Mask {
    apply: bool,
}

impl Highlighter for Mask {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        use unicode_width::UnicodeWidthStr;
        if self.apply {
            Cow::Owned("*".repeat(line.width()))
        } else {
            Cow::Borrowed(line)
        }
    }

    fn highlight_char(&self, _line: &str, _pos: usize) -> bool {
        self.apply
    }
}

/// Ask for information typed by the user after showing the prompt`, like `"Username: `.
pub fn openly(prompt: &str) -> Result<String> {
    editor(false)?.readline(prompt)
}

/// Ask for information _securely_ after showing the `prompt` (like `"password: "`) by not showing what's typed.
///
/// # Security
///
/// The current implementation does only mask the typed characters, they could still be read by an attacker.
/// To do this correctly, echoing has to be disabled entirely. See [this issue](https://github.com/kkawakam/rustyline/issues/58)
/// for progress on this.
pub fn securely(prompt: &str) -> Result<String> {
    let mut ed = editor(true)?;
    ed.set_color_mode(ColorMode::Forced); // force masking, otherwise disabling color won't trigger the highlighter.
    ed.set_auto_add_history(false);
    ed.readline(prompt)
}

fn editor(masked: bool) -> Result<Editor<Mask>> {
    let h = Mask { apply: masked };
    let mut ed = Editor::new()?;
    ed.set_helper(Some(h));
    Ok(ed)
}
