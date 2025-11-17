pub struct HandshakeInitiation {
    pub msg_type: u8,
    pub client_id: u32,
    pub ephemeral_public_key: [u8; 32],
}

impl HandshakeInitiation {
    pub fn new(sender_id: u32, ephemeral_pub: [u8; 32]) -> Self {
        HandshakeInitiation {
            msg_type: 1,
            client_id: sender_id,
            ephemeral_public_key: ephemeral_pub,
        }
    }

    pub fn to_bytes(&self) -> [u8; 37] {
        let mut out = [0u8; 37];
        out[0] = self.msg_type;
        out[1..5].copy_from_slice(&self.client_id.to_le_bytes());
        out[5..37].copy_from_slice(&self.ephemeral_public_key);
        out
    }
}
