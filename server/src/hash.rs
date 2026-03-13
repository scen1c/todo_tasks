use argon2::{
    Argon2, PasswordHash, PasswordVerifier, password_hash::{
        PasswordHasher, SaltString, rand_core::OsRng
    }
};

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(password_hash)
}

pub fn verify_password(password_user: &str, password_hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(password_hash);

    if let Ok(hash) = parsed_hash {
        Argon2::default()
            .verify_password(password_user.as_bytes(), &hash)
            .is_ok()
    } else {
        false
    }
}