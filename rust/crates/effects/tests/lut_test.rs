use effects::CubeLut;

#[test]
fn test_parse_cube_lut() {
    let cube_content = r#"
TITLE "Test LUT"
LUT_3D_SIZE 2
DOMAIN_MIN 0.0 0.0 0.0
DOMAIN_MAX 1.0 1.0 1.0

# 2x2x2 = 8 entries
0.0 0.0 0.0
1.0 0.0 0.0
0.0 1.0 0.0
1.0 1.0 0.0
0.0 0.0 1.0
1.0 0.0 1.0
0.0 1.0 1.0
1.0 1.0 1.0
"#;

    let lut = CubeLut::parse(cube_content).expect("must parse valid .cube content");
    assert_eq!(lut.title, Some("Test LUT".to_string()));
    assert_eq!(lut.size, 2);
    assert_eq!(lut.data.len(), 8);

    // Sample origin (black)
    let black = lut.sample(0.0, 0.0, 0.0);
    assert_eq!(black, [0.0, 0.0, 0.0]);

    // Sample opposite corner (white)
    let white = lut.sample(1.0, 1.0, 1.0);
    assert_eq!(white, [1.0, 1.0, 1.0]);
}

#[test]
fn test_invalid_cube_missing_size() {
    let bad_cube = r#"
TITLE "Bad LUT"
0.0 0.0 0.0
"#;
    let res = CubeLut::parse(bad_cube);
    assert!(res.is_err());
}
