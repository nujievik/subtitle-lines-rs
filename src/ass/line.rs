mod comment;
mod event;
mod info;
pub(crate) mod mark;

pub use comment::Comment;
pub use event::{
    Event,
    format::{EventFormat, EventFormatPositions},
};
pub use info::{ScriptInfo, ScriptType, Title, WrapStyle};
pub use mark::SectionMark;

use crate::Time;
use mark::SectionMarkId;

#[derive(Debug, PartialEq)]
pub enum AssLine<'a> {
    Blank,
    SectionMark(SectionMark<'a>),
    Comment(Comment<'a>),
    ScriptInfo(ScriptInfo<'a>),
    EventFormat(EventFormat<'a>),
    Event(Event<'a>),
    Unrecognized(&'a [u8]),
}

impl<'a> AssLine<'a> {
    /// Returns the source bytes `as is`, correctness is not guaranteed.
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Self::Blank => &[],
            Self::SectionMark(x) => x.as_bytes(),
            Self::Comment(x) => x.as_bytes(),
            Self::ScriptInfo(x) => x.as_bytes(),
            Self::EventFormat(x) => x.as_bytes(),
            Self::Event(x) => x.bytes,
            Self::Unrecognized(bytes) => bytes,
        }
    }

    /// Gets subtitle start time if line is [`AssLine::Event`].
    pub fn get_start(&self) -> Option<Time> {
        match self {
            AssLine::Event(x) => Some(x.start()),
            _ => None,
        }
    }

    /// Gets subtitle end time if line is [`AssLine::Event`].
    pub fn get_end(&self) -> Option<Time> {
        match self {
            AssLine::Event(x) => Some(x.end()),
            _ => None,
        }
    }
}

impl<'a> AssLine<'a> {
    pub(crate) fn new_mark(bytes: &'a [u8], id: SectionMarkId) -> AssLine<'a> {
        AssLine::SectionMark(SectionMark { bytes, id })
    }
}
