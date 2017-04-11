//! Legacy IOS7Crypt encryptor/decryptor library

extern crate rand;
use rand::Rng;

use std::str;
use std::u8;
use std::iter;
use std::slice;

// Finite IOS7Crypt constant key
pub static XLAT_PRIME : [u8; 53] = [
  0x64, 0x73, 0x66, 0x64, 0x3b, 0x6b, 0x66, 0x6f,
  0x41, 0x2c, 0x2e, 0x69, 0x79, 0x65, 0x77, 0x72,
  0x6b, 0x6c, 0x64, 0x4a, 0x4b, 0x44, 0x48, 0x53,
  0x55, 0x42, 0x73, 0x67, 0x76, 0x63, 0x61, 0x36,
  0x39, 0x38, 0x33, 0x34, 0x6e, 0x63, 0x78, 0x76,
  0x39, 0x38, 0x37, 0x33, 0x32, 0x35, 0x34, 0x6b,
  0x3b, 0x66, 0x67, 0x38, 0x37
];

// Wraparound IOS7Crypt constant key
pub fn xlat<'a>(offset : &'a usize) -> iter::Skip<iter::Cycle<slice::Iter<'a, u8>>> {
  return XLAT_PRIME.iter().cycle().skip(*offset);
}

// Bitwise XOR convenience function
pub fn xor(tp : (u8, &u8)) -> u8 {
  let (a, b) : (u8, &u8) = tp;
  return a ^ (*b);
}

// Encode an ASCII password with IOS7Crypt
pub fn encrypt(password : &str) -> String {
  let mut rng = rand::thread_rng();

  let seed = rng.gen_range(0, 16);

  let plaintext : str::Bytes = password.bytes();

  let keys : iter::Skip<iter::Cycle<slice::Iter<u8>>> = xlat(&seed);

  let zipped : iter::Zip<str::Bytes, iter::Skip<iter::Cycle<slice::Iter<u8>>>> = plaintext.zip(keys);

  let ciphertext : Vec<u8> = zipped.map(|pair| xor(pair)).collect();

  let hexpairs : Vec<String> = ciphertext.iter().map(|cipherbyte| format!("{:02x}", cipherbyte)).collect();

  return format!("{:02}{}", seed, hexpairs.concat());
}

pub fn parse_hex(s : &[u8]) -> u8 {
  match str::from_utf8(s) {
    Ok(v) => match u8::from_str_radix(v, 16) {
      Ok(w) => return w,
      Err(err) => panic!(err)
    },
    Err(err) => panic!(err)
  };
}

// Decrypt valid IOS7Crypt hashes
pub fn decrypt(hash : &str) -> String {
  if hash.len() < 2 {
    return "".to_string();
  }
  else {
    let (seed_str, hash_str) : (&str, &str) = hash.split_at(2);

    let seed = seed_str.parse().expect("Invalid seed");

    let codepoints : Vec<u8> = String::from(hash_str).bytes().collect();

    let hexpairs : slice::Chunks<u8> = codepoints.chunks(2);

    let ciphertext : iter::Map<slice::Chunks<u8>, _> = hexpairs.map(|hexpair| parse_hex(hexpair));

    let keys : iter::Skip<iter::Cycle<slice::Iter<u8>>> = xlat(&seed);

    let zipped : iter::Zip<iter::Map<slice::Chunks<u8>, _>, iter::Skip<iter::Cycle<slice::Iter<u8>>>> = ciphertext.zip(keys);

    let plainbytes : iter::Map<_, _> = zipped.map(|pair| xor(pair));

    match String::from_utf8(plainbytes.collect()) {
      Ok(password) => return password,
      Err(err) => panic!(err)
    };
  }
}

#[test]
fn smoketest() {
  assert_eq!(decrypt("1308181c00091d"), "monkey");
}
