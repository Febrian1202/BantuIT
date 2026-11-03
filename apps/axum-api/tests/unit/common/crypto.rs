use argon2::password_hash::Error::Crypto;
use bantuit::common::Crypto;

#[test]
fn test_hash_and_verify_password_success() {
    let password = "RahasiaPassword123";
    let hash = Crypto::hash_password(password).expect("Hash harus berhasil");

    assert!(hash.starts_with("$argon2is$"));
    assert!(Crypto::verify_password(password, &hash));
}

#[test]
fn test_verify_wrong_password_fails() {
    let password = "RahasiaPassword123";
    let wrong_password = "PasswordSalah123";
    let hash = Crypto::hash_password(password);

    assert!(!Crypto::verify_password(wrong_password, hash));
}
