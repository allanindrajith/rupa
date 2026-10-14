use vision::CutoutMask;

#[test]
fn test_mask_creation_and_dimensions() {
    let mask = CutoutMask::new(2, 2, vec![0, 128, 200, 255]).unwrap();
    assert_eq!(mask.width, 2);
    assert_eq!(mask.height, 2);
    assert_eq!(mask.data.len(), 4);

    let mismatch = CutoutMask::new(2, 2, vec![0, 128]);
    assert!(mismatch.is_err(), "must reject size mismatch");
}

#[test]
fn test_blank_and_full_masks() {
    let blank = CutoutMask::blank(4, 4);
    assert!(blank.data.iter().all(|&p| p == 0));

    let full = CutoutMask::full(4, 4);
    assert!(full.data.iter().all(|&p| p == 255));
}

#[test]
fn test_mask_inversion() {
    let mut mask = CutoutMask::new(2, 1, vec![0, 255]).unwrap();
    mask.invert();
    assert_eq!(mask.data, vec![255, 0]);
}

#[test]
fn test_mask_threshold() {
    let mut mask = CutoutMask::new(4, 1, vec![50, 127, 128, 200]).unwrap();
    mask.threshold(128);
    assert_eq!(mask.data, vec![0, 0, 255, 255]);
}

#[test]
fn test_apply_mask_to_rgba_buffer() {
    let mask = CutoutMask::new(2, 1, vec![0, 255]).unwrap();
    // Two pixels RGBA: [255, 255, 255, 255], [100, 100, 100, 200]
    let mut rgba = vec![255, 255, 255, 255, 100, 100, 100, 200];
    mask.apply_to_rgba(&mut rgba).unwrap();

    // First pixel alpha multiplied by 0 -> 0
    assert_eq!(rgba[3], 0);
    // Second pixel alpha multiplied by 255/255 -> 200
    assert_eq!(rgba[7], 200);
}

#[test]
fn test_png_roundtrip_serialization() {
    let original = CutoutMask::new(2, 2, vec![10, 50, 150, 250]).unwrap();
    let png_bytes = original.to_png_bytes().expect("must encode PNG");
    assert!(!png_bytes.is_empty());

    let decoded = CutoutMask::from_png_bytes(&png_bytes).expect("must decode PNG");
    assert_eq!(original.width, decoded.width);
    assert_eq!(original.height, decoded.height);
    assert_eq!(original.data, decoded.data);
}
