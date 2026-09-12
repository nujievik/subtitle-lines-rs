use crate::{
    ConversionLines, Error, FromBytes, RegularAssLines, RegularSrtLines, RegularVttLines, Result,
    SourceLines, WriteLines, WriteOptions,
};
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
};

/// Convenient wrapper to open, convert, and write a supported subtitle file.
pub struct SubtitleLines<'a, T: BufRead> {
    pub(crate) source: SourceLines<'a, T>,
}

pub fn open_file<'a, P: AsRef<Path>>(path: P) -> Result<SubtitleLines<'a, BufReader<File>>> {
    SubtitleLines::open_file(path)
}

impl<'a> SubtitleLines<'a, BufReader<File>> {
    pub fn open_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let ext = path
            .extension()
            .and_then(|ext| ext.to_str())
            .ok_or(Error::ValueValidation("unsupported file extension"))?;
        let new_reader = || File::open(path).map(|f| BufReader::new(f));

        let source = match ext {
            "ass" | "ASS" | "ssa" | "SSA" => {
                SourceLines::Ass(RegularAssLines::from_reader(new_reader()?))
            }
            "srt" | "SRT" => SourceLines::Srt(RegularSrtLines::from_reader(new_reader()?)),
            "vtt" | "VTT" => SourceLines::Vtt(RegularVttLines::from_reader(new_reader()?)),
            _ => return Err(Error::ValueValidation("unsupported file extension")),
        };

        Ok(SubtitleLines { source })
    }
}

impl<'a, T: BufRead> ConversionLines<'a, T> for SubtitleLines<'a, T> {}

impl<'a, T: BufRead> SubtitleLines<'a, T> {
    pub fn write<P>(self, path: &P) -> Result<()>
    where
        P: AsRef<Path> + ?Sized,
    {
        let ofile = File::create(path.as_ref())?;
        let mut writer = BufWriter::new(ofile);
        self.write_to_writer(&mut writer)
    }

    pub fn write_with<P>(self, path: &P, opts: &WriteOptions) -> Result<()>
    where
        P: AsRef<Path> + ?Sized,
    {
        let ofile = File::create(path.as_ref())?;
        let mut writer = BufWriter::new(ofile);
        self.write_to_writer_with(&mut writer, opts)
    }

    pub fn write_to_writer<W>(self, writer: &mut W) -> Result<()>
    where
        W: Write + ?Sized,
    {
        self.write_to_writer_with(writer, &crate::DEFAULT_OPTIONS)
    }

    pub fn write_to_writer_with<W>(self, writer: &mut W, opts: &WriteOptions) -> Result<()>
    where
        W: Write + ?Sized,
    {
        match &self.source {
            SourceLines::Ass(_) => self.into_ass().write_to_writer_with(writer, opts),
            SourceLines::Srt(_) => self.into_srt().write_to_writer_with(writer, opts),
            SourceLines::Vtt(_) => self.into_vtt().write_to_writer_with(writer, opts),
        }
    }
}
