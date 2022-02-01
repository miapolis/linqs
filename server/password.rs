use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub fn hash(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

pub fn verify(password: &str, hash: &str) -> bool {
    let parsed = PasswordHash::new(&hash);
    if parsed.is_err() {
        return false;
    }
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed.unwrap())
        .is_ok()
}
