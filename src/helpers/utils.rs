use aes::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use bitvec::prelude::*;
use hmac::{Hmac, Mac};
use sha2::Sha256;

pub fn join_two_bits_to_byte(first: u8, second: u8, split_index: usize) -> u8 {
    let mut bv: BitVec<u8, Msb0> = bitvec![u8, Msb0; 0;8];
    bv[..split_index].store::<u8>(first);
    bv[split_index..].store::<u8>(second);
    bv[..].load::<u8>()
}

fn pad_payload_bytes(data: &mut Vec<u8>) -> Vec<u8> {
    let length = &data.len();
    if length % 16 == 0 {
        data.to_vec()
    } else {
        let padding_needed = 16 - (length % 16);
        for i in 1..padding_needed {
            data.push(i.try_into().unwrap());
        }
        data.push((padding_needed - 1).try_into().unwrap());
        data.to_vec()
    }
}

pub fn hash_hmac_sha_256(key: Vec<u8>, data: Vec<u8>) -> [u8; 32] {
    type HmacSha256 = Hmac<Sha256>;
    let mut mac =
        HmacSha256::new_from_slice(key.as_slice()).expect("HMAC can take key of any size");
    mac.update(data.as_slice());
    let result = mac.finalize();
    let mut vec_bytes = [0; 32];
    let mut index = 0;
    for i in result.into_bytes() {
        vec_bytes[index] = i;
        index += 1;
    }
    vec_bytes
}

pub fn aes_128_cbc_encrypt(key: [u8; 16], mut payload_bytes: Vec<u8>) -> Vec<u8> {
    type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
    let mut iv = [0; 16];
    for i in 0..iv.len() {
        iv[i] = rand::random::<u8>();
    }
    let binding = pad_payload_bytes(&mut payload_bytes);
    let plaintext = binding.as_slice();

    // encrypt in-place
    // buffer must be big enough for padded plaintext
    let mut buf = [0u8; 48];
    let pt_len = plaintext.len();
    buf[..pt_len].copy_from_slice(&plaintext);
    let mut binding = buf.clone();
    let ct = Aes128CbcEnc::new(&key.into(), &iv.into())
        .encrypt_padded_mut::<aes::cipher::block_padding::NoPadding>(&mut binding, pt_len)
        .unwrap();
    ct.to_vec()
}

pub fn aes_128_cbc_decrypt(key: [u8; 16], iv: [u8; 16], mut encrypted_bytes: Vec<u8>) -> Vec<u8> {
    type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;
    let ct = Aes128CbcDec::new(&key.into(), &iv.into())
        .decrypt_padded_mut::<aes::cipher::block_padding::NoPadding>(&mut encrypted_bytes)
        .unwrap()
        .to_vec();
    // structure of these packets is [[payload x bytes],[padding (1, 2, 3, 4, ...)], padding_length]
    let number_of_padded_bytes: usize = ct[ct.len() - 1].into();
    ct[..(ct.len() - (number_of_padded_bytes + 1))].to_vec()
}