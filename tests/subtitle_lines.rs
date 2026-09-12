mod common;
#[path = "subtitle_lines/write.rs"]
mod write;

use common::*;
use subtitle_lines::*;

#[test]
fn open_ass_file() {
    SubtitleLines::open_file(data("ass.ass")).unwrap();
}

#[test]
fn open_srt_file() {
    SubtitleLines::open_file(data("srt.srt")).unwrap();
}

#[test]
fn open_vtt_file() {
    SubtitleLines::open_file(data("vtt.vtt")).unwrap();
}

#[test]
fn eq_into_ass_with_ass_lines() {
    let mut from_sub = SubtitleLines::open_file(data("ass.ass"))
        .unwrap()
        .into_ass();
    let mut from_ass = AssLines::open_file(data("ass.ass")).unwrap();
    while let Some(l) = from_sub.next() {
        assert_eq!(l, from_ass.next().unwrap());
    }
    assert!(from_ass.next().is_none());
}

#[test]
fn eq_into_srt_with_srt_lines() {
    let mut from_sub = SubtitleLines::open_file(data("srt.srt"))
        .unwrap()
        .into_srt();
    let mut from_srt = SrtLines::open_file(data("srt.srt")).unwrap();
    while let Some(l) = from_sub.next() {
        assert_eq!(l, from_srt.next().unwrap());
    }
    assert!(from_srt.next().is_none());
}

#[test]
fn eq_into_vtt_with_vtt_lines() {
    let mut from_sub = SubtitleLines::open_file(data("vtt.vtt"))
        .unwrap()
        .into_vtt();
    let mut from_vtt = VttLines::open_file(data("vtt.vtt")).unwrap();
    while let Some(l) = from_sub.next() {
        assert_eq!(l, from_vtt.next().unwrap());
    }
    assert!(from_vtt.next().is_none());
}
