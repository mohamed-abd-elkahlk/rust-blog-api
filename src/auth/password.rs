use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
pub fn verify_password(
    password: &String,
    password_hash: &String,
) -> Result<(), argon2::password_hash::Error> {
    // compare user input password to user stored password hash
    let password_hash = PasswordHash::new(&password_hash)?;
    let password = &password;
    Argon2::default().verify_password(password.as_bytes(), &password_hash)
}
pub fn hash_password(password: &String) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_bytes = password.as_bytes();

    let password_hash = argon2.hash_password(password_bytes, &salt).unwrap();
    password_hash.to_string()
}
