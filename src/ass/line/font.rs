use crate::byte_helpers::trim_start;

#[derive(Debug, PartialEq)]
pub struct FontName<'a> {
    bytes: &'a [u8],
    name: &'a [u8],
}

#[derive(Debug, PartialEq)]
pub struct Font<'a> {
    bytes: &'a [u8],
}

impl<'a> FontName<'a> {
    pub(crate) fn get_new(bytes: &'a [u8]) -> Option<FontName<'a>> {
        bytes.strip_prefix(b"fontname:").map(|name| FontName {
            bytes,
            name: trim_start(name),
        })
    }

    pub fn as_bytes(&self) -> &'a [u8] {
        self.bytes
    }

    pub fn name(&self) -> &'a [u8] {
        self.name
    }
}

impl<'a> Font<'a> {
    pub(crate) fn new(bytes: &'a [u8]) -> Font<'a> {
        Font { bytes }
    }

    pub fn as_bytes(&self) -> &'a [u8] {
        self.bytes
    }
}
