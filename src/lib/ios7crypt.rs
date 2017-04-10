//! Legacy IOS7Crypt encryptor/decryptor library

extern crate rand;
use rand::Rng;

use std::str;
use std::i64;
use std::iter::Iterator;

//! Finite IOS7Crypt constant key
pub fn xlat_prime() -> [u8; 53] {
  return [
    0x64, 0x73, 0x66, 0x64, 0x3b, 0x6b, 0x66, 0x6f,
    0x41, 0x2c, 0x2e, 0x69, 0x79, 0x65, 0x77, 0x72,
    0x6b, 0x6c, 0x64, 0x4a, 0x4b, 0x44, 0x48, 0x53,
    0x55, 0x42, 0x73, 0x67, 0x76, 0x63, 0x61, 0x36,
    0x39, 0x38, 0x33, 0x34, 0x6e, 0x63, 0x78, 0x76,
    0x39, 0x38, 0x37, 0x33, 0x32, 0x35, 0x34, 0x6b,
    0x3b, 0x66, 0x67, 0x38, 0x37
  ];
}

//! Wraparound IOS7Crypt constant key
pub fn xlat(i : usize, len : usize) -> Vec<u8> {
  let xs : [u8; 53] = xlat_prime();
  let xs_len : usize = xs.len();

  if len < 1 {
    return Vec::new();
  }
  else {
    let mut head : Vec<u8> = Vec::new();
    head.push(xs[i % xs_len]);

    let mut tail : Vec<u8> = xlat(i + 1, len - 1);

    head.append(&mut tail);
    return head;
  }
}

//! Bitwise XOR convenience function
pub fn xor(tp : (&u8, &u8)) -> u8 {
  let (a, b) : (&u8, &u8) = tp;
  return (*a) ^ (*b);
}

//! Encode an ASCII password with IOS7Crypt
pub fn encrypt(password : &str) -> String {
  let mut rng = rand::thread_rng();

  let seed : usize = rng.gen_range(0, 16);

  let plaintext : &[u8] = password.as_bytes();

  let keys : Vec<u8> = xlat(seed, password.len());

  assert_eq!(plaintext.len(), keys.len());

  let zipped : Vec<(&u8, &u8)> = plaintext.iter().zip(keys.iter()).collect();

  let ciphertext : Vec<u8> = zipped.iter().map(|pair| xor(*pair)).collect();

  let hexpairs : Vec<String> = ciphertext.iter().map(|cipherbyte| format!("{:02x}", cipherbyte)).collect();

  let hexdata : String = hexpairs.concat();

  return format!("{:02}{}", seed, hexdata);
}

//! Decrypt valid IOS7Crypt hashes
pub fn decrypt(hash : &str) -> String {
  if hash.len() < 2 {
    return "".to_string();
  }
  else {
    let (seed_str, hash_str) : (&str, &str) = hash.split_at(2);

    let seed : usize = seed_str.parse().expect("Invalid seed");

    let codepoints : Vec<u8> = String::from(hash_str).bytes().collect();

    let hexpairs : Vec<&[u8]> = codepoints.chunks(2).collect();

    let ciphertext : Vec<u8> = hexpairs.iter().map(|hexpair| match str::from_utf8(*hexpair) {
      Ok(v) => match i64::from_str_radix(v, 16) {
        Ok(w) => w as u8,
        Err(err) => panic!(err)
      },
      Err(err) => panic!(err)
    }).collect();

    let keys : Vec<u8> = xlat(seed, ciphertext.len());

    assert_eq!(ciphertext.len(), keys.len());

    let zipped : Vec<(&u8, &u8)> = ciphertext.iter().zip(keys.iter()).collect();

    let plainbytes : Vec<u8> = zipped.iter().map(|pair| xor(*pair)).collect();

    match String::from_utf8(plainbytes) {
      Ok(password) => return password,
      Err(err) => panic!(err)
    };
  }
}

#[test]
fn smoketest() {
  assert_eq!(decrypt("1308181c00091d"), "monkey");
}
