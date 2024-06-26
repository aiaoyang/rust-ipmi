use std::num::NonZeroU16;

use crate::{request::ReqPayload, ECommand, ECommandCode, Error, IpmiCommand, Payload};

#[derive(Clone, Copy, Debug)]
pub struct GetAllocInfo;

#[derive(Debug, Clone)]
pub struct AllocInfo {
    pub num_alloc_units: Option<NonZeroU16>,
    pub alloc_unit_size: Option<NonZeroU16>,
    pub num_free_units: u16,
    pub largest_free_blk: u16,
    pub max_record_size: u8,
}

impl IpmiCommand for GetAllocInfo {
    type Output = AllocInfo;

    fn netfn(&self) -> crate::NetFn {
        crate::NetFn::Storage
    }

    fn command(&self) -> crate::commands::CommandCode {
        crate::commands::CommandCode::Raw(0x21)
    }

    fn payload(&self) -> crate::Payload {
        Payload::IpmiReq(ReqPayload::new(self.netfn(), self.command(), Vec::new()))
    }
    fn parse(&self, data: &[u8]) -> Result<Self::Output, Error> {
        if data.len() < 8 {
            Err(ECommand::NotEnoughData(ECommandCode::new(
                self.command(),
                8,
                data.len(),
                data.into(),
            )))?
        }

        let num_alloc_units = NonZeroU16::new(u16::from_le_bytes([data[0], data[1]]));
        let alloc_unit_size = NonZeroU16::new(u16::from_le_bytes([data[2], data[3]]));
        let num_free_units = u16::from_le_bytes([data[4], data[5]]);
        let largest_free_blk = u16::from_le_bytes([data[6], data[7]]);
        let max_record_size = data[8];

        Ok(Self::Output {
            num_alloc_units,
            alloc_unit_size,
            num_free_units,
            largest_free_blk,
            max_record_size,
        })
    }
}
