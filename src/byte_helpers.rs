// Returns a byte slice with leading and trailing whitespace removed.
pub fn trim(data: &[u8]) -> &[u8] {
    let mut start = 0;
    let mut end = data.len();

    while start < end && is_ascii_whitespace(data[start]) {
        start += 1;
    }

    while end > start && is_ascii_whitespace(data[end - 1]) {
        end -= 1;
    }

    &data[start..end]
}

pub fn trim_start(data: &[u8]) -> &[u8] {
    let mut pos = 0;
    while pos < data.len() && is_ascii_whitespace(data[pos]) {
        pos += 1;
    }
    &data[pos..]
}

pub fn trim_end(data: &[u8]) -> &[u8] {
    let mut end = data.len();
    while end > 0 && is_ascii_whitespace(data[end - 1]) {
        end -= 1;
    }
    &data[0..end]
}

pub fn trim_bom(data: &[u8]) -> &[u8] {
    data.strip_prefix(crate::BOM).unwrap_or(data)
}

pub fn words<'a>(data: &'a [u8]) -> impl Iterator<Item = &'a [u8]> {
    data.split(|b| is_ascii_whitespace(*b))
        .filter(|word| !word.is_empty())
}

macro_rules! get_an_u_number {
    ($fn:ident, $u:ident, $buf_u:ident) => {
        pub fn $fn(data: &[u8]) -> Option<$u> {
            if data.is_empty() {
                return None;
            }
            let mut v: $buf_u = 0;
            for &b in data {
                if !b.is_ascii_digit() {
                    return None;
                }
                v = v * 10 + (b - b'0') as $buf_u;
                if v > $u::MAX as $buf_u {
                    return None;
                }
            }
            Some(v as $u)
        }
    };
}

get_an_u_number!(get_u8, u8, u16);
get_an_u_number!(get_u16, u16, u32);

pub fn trim_prefix<'a, B>(data: &'a [u8], prefix: &B) -> &'a [u8]
where
    B: AsRef<[u8]> + ?Sized,
{
    let prefix = prefix.as_ref();
    if data.starts_with(prefix) {
        if data.len() == prefix.len() {
            &[]
        } else {
            &data[prefix.len()..]
        }
    } else {
        data
    }
}

// as std u8::is_ascii_whitespace + U+000B VERTICAL TAB
#[inline]
const fn is_ascii_whitespace(byte: u8) -> bool {
    matches!(byte, b'\t' | b'\n' | b'\x0C' | b'\r' | b' ' | b'\x0B')
}
