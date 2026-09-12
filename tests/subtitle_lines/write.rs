use super::*;
use std::fs;

#[test]
fn write_ass() {
    let src = data("ass.ass");
    let dst = temp("subtitle_ass_write.ass");
    let verif = temp("subtitle_ass_write_verification.ass");
    SubtitleLines::open_file(&src).unwrap().write(&dst).unwrap();
    AssLines::open_file(&src).unwrap().write(&verif).unwrap();
    assert_eq!(fs::read(&dst).unwrap(), fs::read(&verif).unwrap());
}

#[test]
fn write_srt() {
    let src = data("srt.srt");
    let dst = temp("subtitle_srt_write.srt");
    let verif = temp("subtitle_srt_write_verification.srt");
    SubtitleLines::open_file(&src).unwrap().write(&dst).unwrap();
    SrtLines::open_file(&src).unwrap().write(&verif).unwrap();
    assert_eq!(fs::read(&dst).unwrap(), fs::read(&verif).unwrap());
}

#[test]
fn write_vtt() {
    let src = data("vtt.vtt");
    let dst = temp("subtitle_vtt_write.vtt");
    let verif = temp("subtitle_vtt_write_verification.vtt");
    SubtitleLines::open_file(&src).unwrap().write(&dst).unwrap();
    VttLines::open_file(&src).unwrap().write(&verif).unwrap();
    assert_eq!(fs::read(&dst).unwrap(), fs::read(&verif).unwrap());
}
