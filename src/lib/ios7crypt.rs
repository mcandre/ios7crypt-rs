//! Legacy IOS7Crypt encryptor/decryptor library

extern crate rand;

#[cfg(test)]
extern crate quickcheck;

use std::str;
use std::u8;
use std::iter;
use std::slice;

/// Finite IOS7Crypt constant key
static XLAT_PRIME : [u8; 53] = [
  0x64, 0x73, 0x66, 0x64, 0x3b, 0x6b, 0x66, 0x6f,
  0x41, 0x2c, 0x2e, 0x69, 0x79, 0x65, 0x77, 0x72,
  0x6b, 0x6c, 0x64, 0x4a, 0x4b, 0x44, 0x48, 0x53,
  0x55, 0x42, 0x73, 0x67, 0x76, 0x63, 0x61, 0x36,
  0x39, 0x38, 0x33, 0x34, 0x6e, 0x63, 0x78, 0x76,
  0x39, 0x38, 0x37, 0x33, 0x32, 0x35, 0x34, 0x6b,
  0x3b, 0x66, 0x67, 0x38, 0x37
];

/// Wraparound IOS7Crypt constant key
pub fn xlat<'a>(offset : &'a usize) -> iter::Skip<iter::Cycle<slice::Iter<'a, u8>>> {
  return XLAT_PRIME.iter().cycle().skip(*offset);
}

/// Encrypt an ASCII password with IOS7Crypt using a given seed
pub fn encrypt_with_seed(seed : usize, password : &str) -> String {
  let hexpairs : Vec<String> = password
    .bytes()
    .zip(xlat(&seed).map(|key| *key))
    .map(|(x, y)| x ^ y)
    .map(|cipherbyte| format!("{:02x}", cipherbyte))
    .collect();

  format!("{:02}{}", seed, hexpairs.concat())
}

/// Encode an ASCII password with IOS7Crypt using a random seed
pub fn encrypt<R: rand::Rng>(rng : &mut R, password : &str) -> String {
  let seed = rng.gen_range(0, 16);

  encrypt_with_seed(seed, password)
}

/// Attempt to parse an array of ASCII hexadecimal digits
/// to their corresponding numeric values.
fn parse_hex(s : &[u8]) -> Option<u8> {
  return match str::from_utf8(s) {
    Ok(v) => match u8::from_str_radix(v, 16) {
      Ok(w) => Some(w),
      Err(_) => None
    },
    Err(_) => None
  };
}

/// Decrypt valid IOS7Crypt hashes
pub fn decrypt(hash : &str) -> Option<String> {
  if hash.len() < 2 {
    return None
  }

  let (seed_str, hash_str) : (&str, &str) = hash.split_at(2);

  let seed = match usize::from_str_radix(seed_str, 10) {
    Ok(v) => v,
    _ => return None
  };

  let plainbytes_options : Vec<Option<u8>> = hash_str.as_bytes()
                                                     .chunks(2)
                                                     .map(|hexpair| parse_hex(hexpair))
                                                     .collect();

  if plainbytes_options.iter().any(|plainbyte_option| plainbyte_option.is_none()) {
    return None
  }

  let plainbytes : iter::Map<_, _> = plainbytes_options.iter()
                                              .map(|plainbytes_option| plainbytes_option.unwrap())
                                              .zip(xlat(&seed).map(|key| *key))
                                              .map(|(x, y)| x ^ y);

  return String::from_utf8(plainbytes.collect()).ok();
}

/// Check whether IOS7Crypt is symmetric
#[cfg(test)]
static PROP_REVERSIBLE : fn(usize, String) -> bool = |seed, password| {
  decrypt(&encrypt_with_seed(seed, &password)).unwrap() == password
};

#[test]
fn smoketest() {
  assert_eq!(decrypt("1308181c00091d"), Some("monkey".to_string()));

  quickcheck::quickcheck(PROP_REVERSIBLE)
}
