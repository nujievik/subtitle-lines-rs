use crate::byte_helpers;

#[derive(Debug, PartialEq)]
pub struct Comment<'a> {
    pub(crate) bytes: &'a [u8],
    pub(crate) text: &'a [u8],
    pub(crate) prefix: CommentPrefix,
}
#[derive(Debug, PartialEq)]
pub enum CommentPrefix {
    /// `;`
    Semicolon,
    /// `!:`
    ExclamationPointWithColon,
}

impl<'a> Comment<'a> {
    pub fn as_bytes(&self) -> &[u8] {
        self.bytes
    }

    #[inline]
    pub const fn prefix(&self) -> &[u8] {
        match self.prefix {
            CommentPrefix::Semicolon => b"; ",
            CommentPrefix::ExclamationPointWithColon => b"!: ",
        }
    }

    #[inline]
    pub const fn text(&self) -> &[u8] {
        self.text
    }

    // requires non-empty line
    pub(crate) fn get_new(line: &'a [u8]) -> Option<Self> {
        let x = Self::new(line);
        if x.text.len() != x.bytes.len() {
            Some(x)
        } else {
            None
        }
    }

    // requires non-empty line
    pub(crate) fn new(line: &'a [u8]) -> Self {
        let (text, prefix) = if matches!(line[0], b';') {
            let text = if line.len() > 1 { &line[1..] } else { b"" };
            (text, CommentPrefix::Semicolon)
        } else {
            (
                byte_helpers::trim_prefix(line, "!:"),
                CommentPrefix::ExclamationPointWithColon,
            )
        };

        Self::new_with(line, byte_helpers::trim_start(text), prefix)
    }

    pub(crate) fn new_with(bytes: &'a [u8], text: &'a [u8], prefix: CommentPrefix) -> Self {
        Self {
            bytes,
            text,
            prefix,
        }
    }

    pub(crate) fn to_vtt(&self) -> crate::vtt::line::Comment<'a> {
        crate::vtt::line::Comment::new(&[], self.text, true)
    }
}
