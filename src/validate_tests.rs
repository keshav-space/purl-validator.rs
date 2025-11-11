use super::*;
use std::path::Path;

#[test]
fn test_validate_with_custom_file() {
    let test_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/test_purls.fst");
    let validator = load_fst(&test_path);

    assert!(validator.contains("pkg:nuget/FluentUtils.EnumExtensions"));
    assert!(!validator.contains("pkg:example/nonexistent"));
}
