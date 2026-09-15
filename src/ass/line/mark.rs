#[derive(Clone, Debug, PartialEq)]
pub struct SectionMark<'a> {
    pub(crate) bytes: &'a [u8],
    pub(crate) id: SectionMarkId,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SectionMarkId {
    ScriptInfo,
    V4Styles,
    V4StylesPlus,
    Events,
    Fonts,
    Graphics,
    Unrecognized,
}

impl<'a> SectionMark<'a> {
    pub fn as_bytes(&self) -> &[u8] {
        if self.bytes.is_empty() {
            self.id.as_bytes()
        } else {
            self.bytes
        }
    }

    pub(crate) fn get_new(bytes: &'a [u8]) -> Option<SectionMark<'a>> {
        if !matches!(bytes[0], b'[') || !matches!(bytes.last().unwrap(), b']') {
            return None;
        }

        let id = match &bytes[1..bytes.len() - 1] {
            b"Script Info" => SectionMarkId::ScriptInfo,
            b"v4 Styles" => SectionMarkId::V4Styles,
            b"v4 Styles+" | b"V4+ Styles" => SectionMarkId::V4StylesPlus,
            b"Events" => SectionMarkId::Events,
            b"Fonts" => SectionMarkId::Fonts,
            b"Graphics" => SectionMarkId::Graphics,
            _ => SectionMarkId::Unrecognized,
        };
        Some(Self { bytes, id })
    }
}

impl SectionMarkId {
    const fn as_bytes(&self) -> &'static [u8] {
        match self {
            SectionMarkId::ScriptInfo => b"[Script Info]",
            SectionMarkId::V4Styles => b"[v4 Styles]",
            SectionMarkId::V4StylesPlus => b"[v4 Styles+]",
            SectionMarkId::Events => b"[Events]",
            SectionMarkId::Fonts => b"[Fonts]",
            SectionMarkId::Graphics => b"[Fonts]",
            SectionMarkId::Unrecognized => b"",
        }
    }
}
