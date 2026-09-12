use crate::{AssLines, ByteLines, Result, SrtLines, VttLines, WriteOptions};
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Empty, Write},
    path::Path,
};

pub trait StreamingIterator {
    type Item<'a>
    where
        Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;

    fn find<'a, F>(&'a mut self, mut predicate: F) -> Option<Self::Item<'a>>
    where
        F: FnMut(&Self::Item<'a>) -> bool,
    {
        let this = self as *mut Self;

        while let Some(item) = unsafe { (&mut *this).next() } {
            if predicate(&item) {
                return Some(item);
            }
        }
        None
    }

    fn find_map<'a, B, F>(&'a mut self, mut f: F) -> Option<B>
    where
        F: FnMut(Self::Item<'a>) -> Option<B>,
    {
        let this = self as *mut Self;

        while let Some(item) = unsafe { (&mut *this).next() } {
            if let Some(result) = f(item) {
                return Some(result);
            }
        }
        None
    }
}

/// A trait for constructs a subtitle lines from **regular** bytes.
/// If conversion is required, use [`ConversionLines`] after construct.
pub trait FromBytes<'a> {
    fn from_bytes<B>(bytes: &'a B) -> Self
    where
        Self: From<ByteLines<'a, Empty>>,
        B: AsRef<[u8]> + ?Sized,
    {
        ByteLines::from_bytes(bytes).into()
    }

    fn from_str<S>(s: &'a S) -> Self
    where
        Self: From<ByteLines<'a, Empty>>,
        S: AsRef<str> + ?Sized,
    {
        Self::from_bytes(s.as_ref())
    }

    fn from_reader<R: BufRead>(reader: R) -> Self
    where
        Self: From<ByteLines<'a, R>>,
    {
        ByteLines::from_reader(reader).into()
    }

    fn open_file<P: AsRef<Path>>(path: P) -> Result<Self>
    where
        Self: From<ByteLines<'a, BufReader<File>>>,
    {
        let f = File::open(path)?;
        Ok(Self::from_reader(BufReader::new(f)))
    }
}

pub trait ConversionLines<'a, T: BufRead> {
    fn from_ass(ass: AssLines<'a, T>) -> Self
    where
        Self: From<AssLines<'a, T>>,
    {
        Self::from(ass)
    }

    fn from_srt(srt: SrtLines<'a, T>) -> Self
    where
        Self: From<SrtLines<'a, T>>,
    {
        Self::from(srt)
    }

    fn from_vtt(vtt: VttLines<'a, T>) -> Self
    where
        Self: From<VttLines<'a, T>>,
    {
        Self::from(vtt)
    }

    fn into_ass(self) -> AssLines<'a, T>
    where
        AssLines<'a, T>: From<Self>,
        Self: 'a,
        Self: Sized,
    {
        AssLines::from(self)
    }

    fn into_srt(self) -> SrtLines<'a, T>
    where
        SrtLines<'a, T>: From<Self>,
        Self: 'a,
        Self: Sized,
    {
        SrtLines::from(self)
    }

    fn into_vtt(self) -> VttLines<'a, T>
    where
        VttLines<'a, T>: From<Self>,
        Self: 'a,
        Self: Sized,
    {
        VttLines::from(self)
    }
}

pub trait WriteLines {
    fn write_to_writer_with<W>(&mut self, writer: &mut W, opts: &WriteOptions) -> Result<()>
    where
        W: Write + ?Sized;

    fn write<P>(&mut self, path: &P) -> Result<()>
    where
        P: AsRef<Path> + ?Sized,
    {
        let ofile = File::create(path.as_ref())?;
        let mut writer = BufWriter::new(ofile);
        self.write_to_writer(&mut writer)
    }

    fn write_with<P>(&mut self, path: &P, opts: &WriteOptions) -> Result<()>
    where
        P: AsRef<Path> + ?Sized,
    {
        let ofile = File::create(path.as_ref())?;
        let mut writer = BufWriter::new(ofile);
        self.write_to_writer_with(&mut writer, opts)
    }

    fn write_to_writer<W>(&mut self, writer: &mut W) -> Result<()>
    where
        W: Write + ?Sized,
    {
        static DEFAULT_OPTIONS: WriteOptions = WriteOptions::new();
        self.write_to_writer_with(writer, &DEFAULT_OPTIONS)
    }
}
