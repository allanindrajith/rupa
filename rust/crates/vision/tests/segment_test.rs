use vision::models::VisionModelManager;
use vision::segment::SegmentationEngine;

#[test]
fn test_cache_dir_discovery() {
    let dir = VisionModelManager::cache_dir().expect("cache dir must resolve");
    assert!(dir.to_str().unwrap().contains("vision"));
}

#[test]
fn test_missing_model_returns_error() {
    let result = SegmentationEngine::new("/non/existent/model.onnx");
    assert!(result.is_err());
}

#[test]
fn test_segment_rgba_frame() {
    let temp = tempfile::NamedTempFile::new().unwrap();
    let engine = SegmentationEngine::new(temp.path()).unwrap();

    // 2x2 white frame
    let rgba = vec![255; 16];
    let mask = engine.segment_frame(&rgba, 2, 2).unwrap();

    assert_eq!(mask.width, 2);
    assert_eq!(mask.height, 2);
    assert_eq!(mask.data.len(), 4);
}

#[test]
fn test_empty_frame_returns_blank_mask() {
    let temp = tempfile::NamedTempFile::new().unwrap();
    let engine = SegmentationEngine::new(temp.path()).unwrap();

    let black_rgba = vec![0; 16];
    let mask = engine.segment_frame(&black_rgba, 2, 2).unwrap();

    assert!(mask.data.iter().all(|&p| p == 0));
}

#[test]
fn test_dimension_mismatch_rejected() {
    let temp = tempfile::NamedTempFile::new().unwrap();
    let engine = SegmentationEngine::new(temp.path()).unwrap();

    let short_rgba = vec![255; 8]; // only 2 pixels for a 2x2 (4 pixel) image
    let res = engine.segment_frame(&short_rgba, 2, 2);
    assert!(res.is_err());
}
