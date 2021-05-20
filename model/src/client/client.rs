use data_encoding::HEXUPPER;
use ring::error::Unspecified;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;
use uuid::Uuid;

pub struct Client {
    uuid: String,
    name: String,
    pass: Pass,
}

struct Pass {
    encrypted_pass: String,
    salt: String,
}

const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;

impl Client {
    pub fn uuid(&self) -> &String {
        &self.uuid
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn same_pass(&self, non_encrypted_pass: &str) -> bool {
        let salt = HEXUPPER.decode(self.pass.salt.as_bytes()).unwrap();
        let pbkdf2_hash = HEXUPPER.decode(self.pass.encrypted_pass.as_bytes()).unwrap();

        let n_iter = NonZeroU32::new(100_000).unwrap();

        let should_succeed = pbkdf2::verify(
            pbkdf2::PBKDF2_HMAC_SHA512,
            n_iter,
            &salt,
            non_encrypted_pass.as_bytes(),
            &pbkdf2_hash,
        );
        should_succeed.is_ok()
    }

    fn encrypt(non_encrypted_pass: &str) -> Result<Pass, Unspecified> {
        let rng = rand::SystemRandom::new();

        let mut salt = [0u8; CREDENTIAL_LEN];
        rng.fill(&mut salt)?;

        let n_iter = NonZeroU32::new(100_000).unwrap();
        let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA512,
            n_iter,
            &salt,
            non_encrypted_pass.as_bytes(),
            &mut pbkdf2_hash
        );

        Ok(Pass {
            encrypted_pass: HEXUPPER.encode(&pbkdf2_hash),
            salt: HEXUPPER.encode(&salt),
        })
    }

    pub fn new(name: &str, non_encrypted_pass: &str) -> Result<Client, Unspecified> {
        let uuid = Uuid::new_v4().to_string();
        let pass = Client::encrypt(non_encrypted_pass)?;

        Ok(Client {
            uuid,
            name: String::from(name),
            pass,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::client::client::Client;

    #[test]
    fn user_has_name() {
        let user_name = "my_new_user";
        let user_pass = "my_super_pass";

        let user : Client = Client::new(user_name, user_pass).unwrap();

        assert_eq!(user_name, user.name());
        assert_ne!(user.uuid(), "");
    }

    #[test]
    fn user_same_pass_is_ok_when_same() {
        let user_name = "my_new_user";
        let user_pass = "my_super_pass";

        let user : Client = Client::new(user_name, user_pass).unwrap();

        assert!(user.same_pass(user_pass));
    }

    #[test]
    fn user_same_pass_is_nok_when_diff() {
        let user_name = "my_new_user";
        let user_pass = "my_super_pass";

        let user : Client = Client::new(user_name, user_pass).unwrap();

        assert!(!user.same_pass("another_super_pass"));
    }
}