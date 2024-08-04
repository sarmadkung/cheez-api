use bcrypt::{hash, verify, DEFAULT_COST};

pub fn hash_password(password: &str) -> String {
    let password_hash = hash(password, DEFAULT_COST).expect("Failed to hash password");
    password_hash
}
pub fn verify_password(hash: &str, password: &str) -> bool {
    let is_valid = verify(password, &hash).expect("Failed to verify password");
    is_valid
}
