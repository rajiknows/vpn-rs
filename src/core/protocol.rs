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
    pub fn from_bytes(buf: &[u8]) -> Option<Self> {
        if buf.len() < 1 + 4 + 32 {
            return None;
        }

        let msg_type = buf[0];

        let client_id = u32::from_le_bytes([buf[1], buf[2], buf[3], buf[4]]);

        let mut ephem = [0u8; 32];
        ephem.copy_from_slice(&buf[5..37]);

        Some(Self {
            msg_type,
            client_id,
            ephemeral_public_key: ephem,
        })
    }
}

pub struct HandshakeResponse {
    pub msg_type: u8,
    pub receiver_index: u32,
    pub server_ephem_pub: [u8; 32],
}

impl HandshakeResponse {
    pub fn new(id: u32, public_key: &[u8; 32]) -> Self {
        Self {
            msg_type: 2,
            receiver_index: id,
            server_ephem_pub: *public_key,
        }
    }
    pub fn to_bytes(&self) -> [u8; 37] {
        let mut buf = [0u8; 37];
        buf[0] = 2;
        buf[1..5].copy_from_slice(&self.receiver_index.to_le_bytes());
        buf[5..37].copy_from_slice(&self.server_ephem_pub);
        buf
    }
    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        if data.len() < 37 || data[0] != 2 {
            return None;
        }
        let mut pubkey = [0u8; 32];
        pubkey.copy_from_slice(&data[5..37]);
        Some(HandshakeResponse {
            msg_type: 2,
            receiver_index: u32::from_le_bytes([data[1], data[2], data[3], data[4]]),
            server_ephem_pub: pubkey,
        })
    }
}
