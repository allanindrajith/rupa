use speech::models::ModelManager;
use std::io::Write;

#[test]
fn test_cache_dir_discovery() {
    let dir = ModelManager::cache_dir().expect("cache_dir must resolve");
    assert!(dir.to_str().unwrap().contains("speech"));
}

#[test]
fn test_checksum_verification() {
    let mut temp = tempfile::NamedTempFile::new().expect("create temp file");
    temp.write_all(b"test speech model binary data").expect("write temp data");

    // SHA256 of "test speech model binary data"
    let expected_hash = "ca4fdfc9d6f357ad8f11813e6dbecbd528b86ad0dc7c8f4ea00926d4c39850f9";

    let valid = ModelManager::verify_checksum(temp.path(), expected_hash).expect("verify must succeed");
    assert!(valid, "valid hash must match");

    let invalid = ModelManager::verify_checksum(temp.path(), "0000000000000000000000000000000000000000000000000000000000000000").expect("verify must succeed");
    assert!(!invalid, "invalid hash must not match");
}

#[test]
fn test_missing_model_returns_error() {
    let missing_path = std::path::Path::new("/non/existent/model.bin");
    let res = ModelManager::verify_checksum(missing_path, "anyhash");
    assert!(res.is_err());
}
