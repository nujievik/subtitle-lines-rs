pub(crate) mod format;
mod new;
mod ty;

use crate::Time;
use ty::EventType;

#[derive(Debug, PartialEq)]
pub struct Event<'a> {
    // raw bytes
    pub(crate) bytes: &'a [u8],
    pub(crate) ty: EventType<'a>,
    // Subtitles having different layer number will be ignored during the collusion detection.
    // Higher numbered layers will be drawn over the lower numbered.
    pub layer: u16,
    pub start: Time,
    pub end: Time,
    pub style_name: &'a [u8],
    // Character name. This is the name of the character who speaks the dialogue. It is for
    // information only, to make the script is easier to follow when editing/timing.
    pub character_name: &'a [u8],
    pub margin_l: u16,
    pub margin_r: u16,
    pub margin_v: u16,
    pub effect: &'a [u8],
    // Subtitle Text. This is the actual text which will be displayed as a subtitle onscreen.
    // Everything after the 9th comma is treated as the subtitle text, so it can include commas.
    // The text can include \n codes which is a line break, and can include Style Override control
    // codes, which appear between braces { }.
    pub text: &'a [u8],
}

impl<'a> Event<'a> {
    /// Returns the source bytes `as is`, correctness is not guaranteed.
    pub fn as_bytes(&self) -> &[u8] {
        self.bytes
    }

    #[inline(always)]
    pub fn start(&self) -> Time {
        self.start
    }

    #[inline(always)]
    pub fn end(&self) -> Time {
        self.end
    }
}
