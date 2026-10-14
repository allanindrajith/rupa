use media::export::{ExportConfig, ExportJob, ExportStatus};
use media::reader::VideoFrame;

#[test]
fn test_export_job_lifecycle() {
    let temp_out = tempfile::NamedTempFile::new().unwrap();
    let config = ExportConfig {
        width: 1920,
        height: 1080,
        fps: 30.0,
        bitrate_kbps: 8000,
        format: "mp4".to_string(),
        output_path: temp_out.path().to_path_buf(),
    };

    // 1000ms duration at 30fps = 30 frames
    let mut job = ExportJob::new(config, 1000).expect("job must create");
    assert_eq!(*job.status(), ExportStatus::Queued);
    assert_eq!(job.progress(), 0.0);

    let frame_bytes = 1920 * 1080 * 4;
    let frame = VideoFrame {
        width: 1920,
        height: 1080,
        pts_ms: 0,
        rgba: vec![0u8; frame_bytes],
    };

    job.write_frame(&frame).expect("frame must write");
    assert!(job.progress() > 0.0);

    job.finish().expect("finish must succeed");
    assert_eq!(*job.status(), ExportStatus::Completed);
}

#[test]
fn test_invalid_export_config_rejected() {
    let temp_out = tempfile::NamedTempFile::new().unwrap();
    let bad_config = ExportConfig {
        width: 0,
        height: 1080,
        fps: 30.0,
        bitrate_kbps: 8000,
        format: "mp4".to_string(),
        output_path: temp_out.path().to_path_buf(),
    };

    let res = ExportJob::new(bad_config, 1000);
    assert!(res.is_err(), "must reject zero dimensions");
}
