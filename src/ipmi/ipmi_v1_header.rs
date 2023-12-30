use super::{ipmi_header::AuthType, ipmi_v1_header_slice::IpmiV1HeaderSlice};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct IpmiV1Header {
    pub auth_type: AuthType,
    pub session_seq_number: u32,
    pub session_id: u32,
    pub auth_code: Option<u128>,
    pub payload_length: u8,
}

impl Default for IpmiV1Header {
    fn default() -> Self {
        Self {
            auth_type: AuthType::None,
            session_seq_number: 0x00,
            session_id: 0x00,
            auth_code: None,
            payload_length: 0,
        }
    }
}

impl IpmiV1Header {
    pub const MIN_LEN: usize = 10;

    pub const MAX_LEN: usize = 26;

    pub fn new(auth_type: AuthType, session_seq_number: u32, session_id: u32) -> IpmiV1Header {
        IpmiV1Header {
            auth_type,
            session_seq_number,
            session_id,
            auth_code: None,
            payload_length: 0,
        }
    }

    pub fn from_slice(slice: &[u8]) -> Result<(IpmiV1Header, &[u8]), std::io::ErrorKind> {
        let h = IpmiV1HeaderSlice::from_slice(slice)?;
        // println!("{:x?}", h);
        Ok((h.to_header(), &slice[h.slice().len()..]))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self.auth_type {
            AuthType::None => {
                let seq_be = self.session_seq_number.to_be_bytes();
                let ses_be = self.session_id.to_be_bytes();
                let mut result = Vec::new();
                result.extend([
                    self.auth_type.to_u8(),
                    seq_be[0],
                    seq_be[1],
                    seq_be[2],
                    seq_be[3],
                    ses_be[0],
                    ses_be[1],
                    ses_be[2],
                    ses_be[3],
                    self.payload_length,
                ]);
                unsafe { result.set_len(10) };
                result
            }
            _ => {
                let seq_be = self.session_seq_number.to_be_bytes();
                let ses_be = self.session_id.to_be_bytes();
                let auth_be = self.auth_code.unwrap().to_be_bytes();
                let mut result = Vec::new();
                result.extend([
                    self.auth_type.to_u8(),
                    seq_be[0],
                    seq_be[1],
                    seq_be[2],
                    seq_be[3],
                    ses_be[0],
                    ses_be[1],
                    ses_be[2],
                    ses_be[3],
                    auth_be[0],
                    auth_be[1],
                    auth_be[2],
                    auth_be[3],
                    auth_be[4],
                    auth_be[5],
                    auth_be[6],
                    auth_be[7],
                    auth_be[8],
                    auth_be[9],
                    auth_be[10],
                    auth_be[11],
                    auth_be[12],
                    auth_be[13],
                    auth_be[14],
                    auth_be[15],
                    self.payload_length,
                ]);
                unsafe { result.set_len(26) };
                result
            }
        }
    }
}