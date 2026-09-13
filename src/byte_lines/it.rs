use super::{ByteLines, ByteLinesTy};
use std::io::BufRead;

impl<T: BufRead> crate::StreamingIterator for ByteLines<'_, T> {
    type Item<'a>
        = &'a [u8]
    where
        Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        match &mut self.ty {
            ByteLinesTy::ByteSlice(bytes) => get_slice_line(bytes, &mut self.pos).map(|(bs, _)| bs),
            ByteLinesTy::BufReader(reader) => {
                reader.consume(self.pos);
                self.pos = 0;

                let reader_ptr = reader as *mut T;
                let internal_buf = unsafe { (*reader_ptr).fill_buf().ok()? };

                let (line, complete) = get_slice_line(internal_buf, &mut self.pos)?;

                // do not fill self.buf if line <= reader internal buf
                if complete {
                    return Some(line);
                }

                self.buf.clear();
                self.buf.extend_from_slice(line);

                loop {
                    reader.consume(self.pos);
                    self.pos = 0;

                    let internal_buf = reader.fill_buf().ok()?;

                    if internal_buf.is_empty() {
                        return if self.buf.is_empty() {
                            None
                        } else {
                            Some(&self.buf)
                        };
                    }

                    let (line, complete) = get_slice_line(internal_buf, &mut self.pos)?;
                    self.buf.extend_from_slice(line);

                    if complete {
                        return Some(&self.buf);
                    }
                }
            }
        }
    }
}

fn get_slice_line<'a>(data: &'a [u8], pos: &mut usize) -> Option<(&'a [u8], bool)> {
    if *pos >= data.len() {
        return None;
    }

    let start = *pos;

    while *pos < data.len() {
        match data[*pos] {
            b'\r' => {
                let end = *pos;
                *pos += 1;

                if data.get(*pos).is_some_and(|b| matches!(b, b'\n')) {
                    *pos += 1;
                }

                return Some((&data[start..end], true));
            }
            b'\n' => {
                let end = *pos;
                *pos += 1;

                return Some((&data[start..end], true));
            }
            _ => *pos += 1,
        }
    }

    Some((&data[start..], false))
}
