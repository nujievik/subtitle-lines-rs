pub mod line;

mod it;
mod new;
mod write;

use crate::{ByteLines, ConversionLines, FromBytes, Result, SourceLines};
use it::{IterState, TransIterState};
use line::mark::SectionMarkId;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Debug)]
pub struct AssLines<'a, T: BufRead> {
    pub(crate) source: SourceLines<'a, T>,
    buf: Vec<u8>,
    trans_state: TransIterState,
}

#[derive(Debug)]
pub(crate) struct RegularAssLines<'a, T: BufRead> {
    pub(crate) lines: ByteLines<'a, T>,
    pub(crate) state: IterState,
    section_state: SectionMarkId,
}

pub fn open_file<'a, P: AsRef<Path>>(path: P) -> Result<AssLines<'a, BufReader<File>>> {
    AssLines::open_file(path)
}

impl<'a, T: BufRead> ConversionLines<'a, T> for AssLines<'a, T> {}
