pub enum Packet {
    Exit,
    Ping,
    MouseMove { dx: f32, dy: f32 },
    LMBDown,
    LMBUp,
    Unknown,
}

impl Packet {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        match bytes[0] {
            0 => Packet::Exit,
            1 => Packet::Ping,
            2 => {
                assert!(bytes.len() == 9);
                Packet::MouseMove {
                    dx: f32::from_be_bytes(bytes[1..5].try_into().unwrap()),
                    dy: f32::from_be_bytes(bytes[5..9].try_into().unwrap()),
                }
            },
            3 => Packet::LMBDown,
            4 => Packet::LMBUp,
            _ => Packet::Unknown,
        }
    }
}
