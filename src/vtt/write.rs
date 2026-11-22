use super::{VttLines, line::CueId};
use crate::{
    Result, SourceLines, StreamingIterator, VttLine, WriteLines, WriteOptions,
    time::bufs::VttTimeBuf,
};
use std::io::{BufRead, Write};

impl<'a, T: BufRead> WriteLines for VttLines<'a, T> {
    fn write_to_writer_with<W>(&mut self, writer: &mut W, opts: &WriteOptions) -> Result<()>
    where
        W: Write + ?Sized,
    {
        if opts.bom {
            writer.write_all(crate::BOM)?;
        }
        writer.write_all(b"WEBVTT\n\n")?;

        let is_regular_source = matches!(&self.source, SourceLines::Vtt(_));

        let is_setted_time = opts.start.is_some()
            || opts.end.is_some()
            || opts.add_time.is_some()
            || opts.sub_time.is_some();

        let capacity = if is_setted_time { 32 } else { 0 };
        let mut cue_id_buf = Vec::with_capacity(capacity);
        let mut is_written_blank = true;
        let mut is_written_cue_time = false;
        let mut buf = VttTimeBuf::new();

        while let Some(line) = self.next() {
            let bytes: &[u8] = match &line {
                VttLine::VttFileMark(_) => continue,
                VttLine::Blank => {
                    if !is_written_blank {
                        writer.write_all(b"\n")?;
                        cue_id_buf.clear();
                        is_written_blank = true;
                        is_written_cue_time = false;
                    }
                    continue;
                }
                VttLine::CueId(CueId { bytes }) if is_setted_time => {
                    cue_id_buf.clear();
                    cue_id_buf.extend_from_slice(bytes);
                    continue;
                }

                VttLine::Comment(c) => {
                    writer.write_all(c.prefix())?;
                    c.text()
                }

                VttLine::TimeRangeAndStyle(tr) if !is_regular_source || is_setted_time => {
                    let mut start = tr.start;
                    let mut end = tr.end;

                    if opts.start.is_some_and(|t| end <= t) || opts.end.is_some_and(|t| start >= t)
                    {
                        cue_id_buf.clear();
                        continue;
                    }
                    if let Some(add) = opts.add_time {
                        start += add;
                        end += add;
                    }
                    if let Some(sub) = opts.sub_time {
                        start -= sub;
                        end -= sub;
                    }

                    if !cue_id_buf.is_empty() {
                        writer.write_all(&cue_id_buf)?;
                        writer.write_all(b"\n")?;
                        cue_id_buf.clear();
                    }

                    writer.write_all(buf.0.format_time(start))?;
                    writer.write_all(b" --> ")?;
                    writer.write_all(buf.0.format_time(end))?;
                    writer.write_all(b"\n")?;

                    is_written_blank = false;
                    is_written_cue_time = true;
                    continue;
                }
                VttLine::Text(_) if is_setted_time && !is_written_cue_time => continue,
                VttLine::Metadata(_) if is_setted_time && !is_written_cue_time => continue,
                line => line.as_bytes(),
            };

            writer.write_all(bytes)?;
            writer.write_all(b"\n")?;
            is_written_blank = false;
        }

        writer.flush()?;
        Ok(())
    }
}
