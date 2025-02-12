use std::num::NonZeroU32;

use ring::{digest, pbkdf2};
use sha2::{Digest, Sha256};

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
pub type Credential = [u8; CREDENTIAL_LEN];

#[derive(Clone)]
pub struct Password {
    pbkdf2_iterations: NonZeroU32,
    db_salt_component: [u8; 16],
}

impl Password {
    pub fn new(pbkdf2_iterations: NonZeroU32, salt: [u8; 16]) -> Self {
        Self {
            pbkdf2_iterations,
            db_salt_component: salt,
        }
    }
    pub fn generate(&self, email: &[u8], pass: &[u8]) -> [u8; CREDENTIAL_LEN] {
        let salt = self.salf(email);
        let mut to_store = [0; CREDENTIAL_LEN];
        pbkdf2::derive(
            PBKDF2_ALG,
            self.pbkdf2_iterations,
            &salt,
            pass,
            &mut to_store,
        );
        to_store
    }
    pub fn verify_password(&self, email: &[u8], actual: &[u8], attempted_password: &[u8]) -> bool {
        let salt = self.salf(email);
        pbkdf2::verify(
            PBKDF2_ALG,
            self.pbkdf2_iterations,
            &salt,
            attempted_password,
            actual,
        )
        .is_ok()
    }
    fn salf(&self, email: &[u8]) -> Vec<u8> {
        // The salt should have a user-specific component so that an attacker
        // cannot crack one password for multiple users in the database. It
        // should have a database-unique component so that an attacker cannot
        // crack the same user's password across databases in the unfortunate
        // but common case that the user has used the same password for
        // multiple systems.
        let mut hasher = Sha256::new();
        hasher.update(self.db_salt_component);
        hasher.update(email);
        hasher.finalize().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU32;

    use super::Password;

    #[test]
    fn match_password() {
        let email = "useremail@example.com";
        let pass = "@74d7]404j|W}6u";
        let v = Password::new(
            NonZeroU32::new(100_000).unwrap(),
            [
                0xd6, 0x26, 0x98, 0xda, 0xf4, 0xdc, 0x50, 0x52, 0x24, 0xf2, 0x27, 0xd1, 0xfe, 0x39,
                0x01, 0x8a,
            ],
        );
        let pass_hash = v.generate(email.as_bytes(), pass.as_bytes());
        assert!(!v.verify_password(email.as_bytes(), &pass_hash, "my-password".as_bytes()));
        assert!(!v.verify_password(
            "unknown@gmail.com".as_bytes(),
            &pass_hash,
            "@74d7]404j|W}6u".as_bytes()
        ));
        assert!(v.verify_password(email.as_bytes(), &pass_hash, "@74d7]404j|W}6u".as_bytes()));
    }
}
