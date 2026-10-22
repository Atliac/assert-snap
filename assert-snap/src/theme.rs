//! Terminal styling and theming for snapshot assertion output.

use std::io::IsTerminal;

use anstyle::{AnsiColor, Color, Style};

/// The visual theme applied to snapshot assertion outputs.
#[derive(Clone, Copy, Debug)]
pub struct Theme {
    pub brand: Style,
    pub location: Style,
    pub actual_header: Style,
    pub redacted_header: Style,
    pub expected_header: Style,
    pub diff_header: Style,
    pub diff_del: Style,
    pub diff_ins: Style,
    pub diff_hunk: Style,
    pub diff_equal: Style,
    pub pass: Style,
    pub fail: Style,
}

impl Theme {
    /// Returns the default theme using standard 16 ANSI colors that adapt
    /// naturally to both dark and light terminal palettes.
    pub const fn default() -> Self {
        Self {
            brand: Style::new().bold(),
            location: Style::new().dimmed().underline(),
            actual_header: Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Yellow))),
            redacted_header: Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Magenta))),
            expected_header: Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Cyan))),
            diff_header: Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Yellow))),
            diff_del: Style::new().fg_color(Some(Color::Ansi(AnsiColor::Red))),
            diff_ins: Style::new().fg_color(Some(Color::Ansi(AnsiColor::Green))),
            diff_hunk: Style::new().fg_color(Some(Color::Ansi(AnsiColor::Cyan))),
            diff_equal: Style::new(),
            pass: Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Green))),
            fail: Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Red))),
        }
    }

    /// Returns a plain (unstyled) theme.
    pub const fn plain() -> Self {
        Self {
            brand: Style::new(),
            location: Style::new(),
            actual_header: Style::new(),
            redacted_header: Style::new(),
            expected_header: Style::new(),
            diff_header: Style::new(),
            diff_del: Style::new(),
            diff_ins: Style::new(),
            diff_hunk: Style::new(),
            diff_equal: Style::new(),
            pass: Style::new(),
            fail: Style::new(),
        }
    }

    /// Detects environment and terminal capabilities to select the appropriate theme.
    pub fn current() -> Self {
        if should_colorize() {
            Self::default()
        } else {
            Self::plain()
        }
    }
}

/// Determines whether color styling should be enabled.
///
/// Follows the standard `NO_COLOR` specification (<https://no-color.org/>):
/// if `NO_COLOR` is present and not an empty string, color is disabled.
/// Otherwise, color is enabled if stdout is an interactive terminal.
pub fn should_colorize() -> bool {
    if let Some(val) = std::env::var_os("NO_COLOR")
        && !val.is_empty()
    {
        return false;
    }

    std::io::stdout().is_terminal()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plain_theme_emits_no_ansi() {
        let plain = Theme::plain();
        let formatted = format!("{style}hello{style:#}", style = plain.brand);
        assert_eq!(formatted, "hello");
    }

    #[test]
    fn test_default_theme_emits_ansi_when_styled() {
        let default = Theme::default();
        let formatted = format!("{style}hello{style:#}", style = default.pass);
        assert!(formatted.contains("\x1b["));
    }
}
