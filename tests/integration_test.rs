use rust_hash_finder::{compute_hash, hash_ends_with_zeros};

#[test]
fn test_integration_hash_computation() {
    let hash = compute_hash(12345);
    assert_eq!(hash.len(), 64);
}

#[test]
fn test_integration_known_value() {
    let hash = compute_hash(4163);
    assert!(hash_ends_with_zeros(&hash, 3));
}

#[test]
fn test_integration_hash_consistency() {
    for i in 1..100 {
        let hash1 = compute_hash(i);
        let hash2 = compute_hash(i);
        assert_eq!(hash1, hash2, "Hash для {} должен быть консистентным", i);
    }
}
