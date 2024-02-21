use crate::{
    rmcp::commands::CommandType,
    rmcp::{AuthType, IpmiHeader, IpmiV2Header, Packet, Payload, PayloadType},
};

use super::{
    bmc::{SlaveAddress, SoftwareType},
    netfn_lun::{Lun, NetFn, NetfnLun, RqseqLun},
    response::Address,
};

pub struct IpmiRawRequest {
    pub netfn: NetFn,
    pub command_code: CommandType,
    pub data: Option<Vec<u8>>,
}

impl IpmiRawRequest {
    const PAYLOAD_LENGTH: u16 = 32;
    pub fn new(netfn: NetFn, command_code: CommandType, data: Option<Vec<u8>>) -> IpmiRawRequest {
        IpmiRawRequest {
            netfn,
            command_code,
            data,
        }
    }

    pub fn create_packet(&self, rmcp_plus_session_id: u32, session_seq_number: u32) -> Packet {
        let netfn = self.netfn;
        let cmd = self.command_code;
        let data = self.data.clone();
        Packet::new(
            IpmiHeader::V2_0(IpmiV2Header {
                auth_type: AuthType::RmcpPlus,
                payload_enc: true,
                payload_auth: true,
                payload_type: PayloadType::Ipmi,
                oem_iana: None,
                oem_payload_id: None,
                rmcp_plus_session_id,
                session_seq_number,
                payload_length: Self::PAYLOAD_LENGTH,
            }),
            Payload::IpmiReq(ReqPayload::new(netfn, cmd, data)),
        )
    }
}

#[derive(Clone, Debug)]
pub struct ReqPayload {
    pub rs_addr: Address,
    pub netfn_rslun: NetfnLun,
    // checksum 1
    pub rq_addr: Address,
    pub rqseq_rqlun: RqseqLun,
    pub command: CommandType,
    pub data: Option<Vec<u8>>,
    // checksum 2
}

const BMC_SLAVE_ADDRESS: u8 = 0x20;
const REMOTE_SOFTWARE_ID: u8 = 0x81;

impl From<ReqPayload> for Vec<u8> {
    fn from(val: ReqPayload) -> Self {
        let mut result: Vec<u8> = vec![];
        let rs_addr = BMC_SLAVE_ADDRESS;

        let checksum1 = checksum(&[rs_addr, val.netfn_rslun.0]);
        let rq_addr = REMOTE_SOFTWARE_ID;
        let command_code: u8 = (val.command).into();

        result.extend(&[
            rs_addr,
            val.netfn_rslun.0,
            checksum1,
            rq_addr,
            val.rqseq_rqlun.0,
            command_code,
        ]);

        if let Some(data) = &val.data {
            result.extend(data);
        }
        result.push(checksum(&result[3..]));
        result
    }
}

impl ReqPayload {
    pub fn new(net_fn: NetFn, command: CommandType, data: Option<Vec<u8>>) -> ReqPayload {
        ReqPayload {
            rs_addr: Address::Slave(SlaveAddress::Bmc),
            netfn_rslun: NetfnLun::new(net_fn, Lun::Bmc),
            rq_addr: Address::Software(SoftwareType::RemoteConsoleSoftware(1)),
            rqseq_rqlun: RqseqLun::new(0x08, Lun::Bmc),
            command,
            data,
        }
    }
}

impl Default for ReqPayload {
    fn default() -> Self {
        Self {
            rs_addr: Address::Slave(SlaveAddress::Bmc),
            netfn_rslun: NetfnLun::new(NetFn::App, Lun::Bmc),
            rq_addr: Address::Software(SoftwareType::RemoteConsoleSoftware(1)),
            rqseq_rqlun: RqseqLun::new(0x00, Lun::Bmc),
            command: CommandType::GetChannelAuthCapabilities,
            data: None,
        }
    }
}

// two's complement sum
pub fn checksum(input: &[u8]) -> u8 {
    let mut res = 0_i32;
    for val in input {
        res += *val as i32
    }
    (-res) as u8
}
