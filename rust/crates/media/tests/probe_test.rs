use media::probe::MediaProbe;
use std::io::Write;

#[test]
fn test_missing_file_returns_error() {
    let res = MediaProbe::probe("/non/existent/video.mp4");
    assert!(res.is_err());
}

#[test]
fn test_probe_mp4_file() {
    let mut temp = tempfile::NamedTempFile::new().unwrap();
    // Write MP4 magic header (length=8, 'ftyp', 'isom')
    temp.write_all(b"\0\0\0\x18ftypisom\0\0\x02\0isomiso2mp41").unwrap();

    let meta = MediaProbe::probe(temp.path()).expect("must probe valid mp4 header");
    assert!(meta.video.is_some());
    let video = meta.video.unwrap();
    assert_eq!(video.width, 1920);
    assert_eq!(video.height, 1080);
    assert!(video.codec.contains("h264"));
}

#[test]
fn test_probe_webm_file() {
    let mut temp = tempfile::NamedTempFile::new().unwrap();
    // Write WebM magic bytes \x1a\x45\xdf\xa3
    temp.write_all(b"\x1a\x45\xdf\xa3\x9f\x42\x86\x81\x01\x42\xf7\x81\x01").unwrap();

    let meta = MediaProbe::probe(temp.path()).expect("must probe valid webm header");
    assert!(meta.video.is_some());
    let video = meta.video.unwrap();
    assert_eq!(video.codec, "vp9");
}
