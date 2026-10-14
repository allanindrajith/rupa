use effects::{EffectCatalogue, EffectCategory};

#[test]
fn test_catalogue_builtins() {
    let catalogue = EffectCatalogue::new();
    let effects = catalogue.list();
    assert!(effects.len() >= 6, "must have built-in effects");

    // Check presence of key effects
    assert!(catalogue.get("rupa.adjust").is_some());
    assert!(catalogue.get("rupa.chromatic-aberration").is_some());
    assert!(catalogue.get("rupa.vignette").is_some());
    assert!(catalogue.get("rupa.film-grain").is_some());
    assert!(catalogue.get("rupa.rgb-split").is_some());
    assert!(catalogue.get("rupa.cross-dissolve").is_some());
}

#[test]
fn test_catalogue_filter_by_category() {
    let catalogue = EffectCatalogue::new();
    let color_effects = catalogue.by_category(EffectCategory::Color);
    assert!(!color_effects.is_empty());
    assert!(color_effects.iter().all(|e| e.category == EffectCategory::Color));

    let distort_effects = catalogue.by_category(EffectCategory::Distort);
    assert!(!distort_effects.is_empty());
    assert!(distort_effects.iter().all(|e| e.category == EffectCategory::Distort));
}

#[test]
fn test_effect_manifest_serialization() {
    let catalogue = EffectCatalogue::new();
    let adjust = catalogue.get("rupa.adjust").unwrap();

    let json = serde_json::to_string(adjust).expect("must serialize");
    assert!(json.contains("rupa.adjust"));
    assert!(json.contains("Brightness"));
}
