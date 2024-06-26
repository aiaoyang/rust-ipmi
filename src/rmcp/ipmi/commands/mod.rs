pub use app::channel::*;
pub use app::cipher::*;
pub use chassis::*;
pub use sdr::*;
pub use sel::*;

mod app;
mod chassis;
mod sdr;
mod sel;

use crate::{
    err::Error,
    rmcp::{request::ReqPayload, Payload},
    IpmiCommand,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandCode {
    Raw(u8),
    Chassis,
    GetChannelAuthCapabilities,
    SetSessionPrivilegeLevel,
    CloseSession,
    GetChannelCipherSuites,
}

impl From<u8> for CommandCode {
    fn from(val: u8) -> Self {
        match val {
            0x38 => CommandCode::GetChannelAuthCapabilities,
            0x54 => CommandCode::GetChannelCipherSuites,
            0x3b => CommandCode::SetSessionPrivilegeLevel,
            0x3c => CommandCode::CloseSession,
            0x01 => CommandCode::Chassis,
            x => CommandCode::Raw(x),
        }
    }
}

impl From<CommandCode> for u8 {
    fn from(val: CommandCode) -> Self {
        match val {
            CommandCode::Chassis => 0x01,
            CommandCode::GetChannelAuthCapabilities => 0x38,
            CommandCode::GetChannelCipherSuites => 0x54,
            CommandCode::SetSessionPrivilegeLevel => 0x3b,
            CommandCode::CloseSession => 0x3c,
            CommandCode::Raw(x) => x,
        }
    }
}

pub struct CloseSessionCMD(u32);
impl CloseSessionCMD {
    pub fn new(session_id: u32) -> Self {
        Self(session_id)
    }
}
impl IpmiCommand for CloseSessionCMD {
    type Output = ();

    fn netfn(&self) -> crate::NetFn {
        crate::NetFn::App
    }

    fn command(&self) -> CommandCode {
        CommandCode::CloseSession
    }

    fn payload(&self) -> Payload {
        Payload::IpmiReq(ReqPayload::new(
            self.netfn(),
            self.command(),
            vec![
                self.0 as u8,
                (self.0 >> 8) as u8,
                (self.0 >> 16) as u8,
                (self.0 >> 24) as u8,
            ],
        ))
    }

    fn parse(&self, _data: &[u8]) -> Result<Self::Output, Error> {
        Ok(())
    }
}
