//! Legacy IOS7Crypt encryptor/decryptor library

extern crate rand;

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
pub fn encrypt<R: rand::Rng>(rng : &mut R, password : &str) -> String {
  let seed = rng.gen_range(0, 16);

  let hexpairs : Vec<String> = password.bytes()
                                       .zip(xlat(&seed))
                                       .map(|pair| xor(pair))
                                       .map(|cipherbyte| format!("{:02x}", cipherbyte))
                                       .collect();

  return format!("{:02}{}", seed, hexpairs.concat());
}

// Attempt to parse an array of ASCII hexadecimal digits
// to their corresponding numeric values.
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
pub fn decrypt(hash : &str) -> Option<String> {
  if hash.len() < 2 {
    return None
  }

  let (seed_str, hash_str) : (&str, &str) = hash.split_at(2);

  let seed = seed_str.parse().expect("Invalid seed");

  let codepoints : Vec<u8> = String::from(hash_str).bytes().collect();

  let plainbytes : iter::Map<_, _> = codepoints.chunks(2)
                                               .map(|hexpair| parse_hex(hexpair))
                                               .zip(xlat(&seed))
                                               .map(|pair| xor(pair));

  return match String::from_utf8(plainbytes.collect()) {
    Ok(password) => Some(password),
    _ => None
  };
}

#[test]
fn smoketest() {
  assert_eq!(decrypt("1308181c00091d"), Some("monkey".to_string()));
}
