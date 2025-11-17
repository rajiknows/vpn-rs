use ring::{agreement::{EphemeralPrivateKey, X25519}, rand::SystemRandom};



pub struct KeyPair{
    pub public_key:[u8;32],
    pub private_key:EphemeralPrivateKey
}

impl KeyPair{
    pub fn generate() -> Self{
        let rng =SystemRandom::new();
        let private = EphemeralPrivateKey::generate(&X25519, &rng).unwrap();
        let public =private.compute_public_key().unwrap();

        let mut public_key = [0u8;32];

        public_key.copy_from_slice(public.as_ref());
        KeyPair { public_key, private_key:private }
    }
}
