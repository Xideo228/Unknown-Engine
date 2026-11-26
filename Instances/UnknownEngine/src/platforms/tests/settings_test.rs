use crate::platforms::settings::WindowSettings;

#[test]
fn test_correction() {
    let set = WindowSettings::default();

    assert!(!set.name.is_empty());
    assert!(!set.title.is_empty());
    assert!(set.height == 720);
    assert!(set.width == 1280);
}

#[test]
fn test_duplication() {
    let set_original = WindowSettings::default();
    let set_clone = set_original.clone();

    assert_eq!(set_original.name, set_clone.name);
}