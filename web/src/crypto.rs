use crypto_hash::{Algorithm, hex_digest};

pub fn sha_256(input: &[u8]) -> String {
  let digest = hex_digest(Algorithm::SHA256, input);

  digest
}


// #[cfg(test)]
// mod tests {
//   use super::{sha_256};
  
//   #[test]
//   fn test_sha_256(){
//     let rs = sha_256("hello,jacky".as_bytes());
//     assert_eq!(rs, "E334A08F6CF24050364D550625473B5F2EE7BAF8C5ACAAEA26C79D80753F2028".to_lowercase());
//   }
// }
