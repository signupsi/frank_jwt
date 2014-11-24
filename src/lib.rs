extern crate serialize;
extern crate time;
extern crate "rust-crypto" as rust_crypto;

use serialize::base64;
use serialize::base64::{ToBase64, URLSAFE_CHARS};
use serialize::hex::{FromHex, ToHex};
use serialize::json::ToJson;
use serialize::json;
use std::collections::TreeMap;
use rust_crypto::sha2::Sha256;
use rust_crypto::hmac::Hmac;

struct JwtHeader {
 alg: &'static str,
 typ: &'static str
}

struct JwtClaims {
  iss: &'static str,
  iat: int,
  exp: int,
  qsh: &'static str,
  sub: &'static str
}

impl ToJson for JwtHeader {
  fn to_json(&self) -> json::Json {
    let mut d = TreeMap::new();
    d.insert("alg".to_string(), self.alg.to_json());
    d.insert("typ".to_string(), self.typ.to_json());
    json::Object(d)
  }
}

impl ToJson for JwtClaims {
  fn to_json(&self) -> json::Json {
    let mut d = TreeMap::new();
    d.insert("iss", self.iss.to_json());
    d.insert("iat", self.iat.to_json());
    d.insert("exp", self.exp.to_json());
    d.insert("qsh", self.qsh.to_json());
    d.insert("sub", self.sub.to_json());
    json::Object(d)
  } 
}


fn generate_jwt_token(request_url: &str, canonical_url: &str, key: &str, shared_secret: &str) -> &'static str {
  let iat = time::now().tm_nsec * 1000;
  let exp = iat + 180 * 1000;
  let qsh = get_query_string_hash(canonical_url);
  let claims = JwtClaims { iss: key, iat: iat, exp: exp, qsh: qsh };
  sign(claims, shared_secret)
}

fn sign(claims: JwtClaims, shared_secret: &str) -> &str {
  let signing_input = get_signing_input(claims, shared_secret);
  let signed256 = sign_hmac256(signing_input, shared_secret);
  signing_input.to_string() + "." + signed256.to_string()
}

fn get_signing_input(claims: JwtClaims, shared_secret: &str) -> &str {
  let header = JwtHeader { alg: "HS256", typ: "JWT" };
  
  let header_json_str = header.to_json();
  let claims_json_str = claims.to_json();

  let hb64_url_e_str = base64_url_encode(header_json_str.to_string().into_bytes()).to_string();
  let cb64_url_e_str = base64_url_encode(claims_json_str.to_string().into_bytes()).to_string();
  hb64_url_e_str + "." + cb64_url_e_str
}


fn sign_hmac256(signing_input: &str, shared_secret: &str) -> &'static str {
  let hmac = Hmac::new(Sha256::new(), shared_secret);
  hmac.input(signing_input);
  let result = hmac.result().code;
  base64_url_encode(result)
}

fn get_query_string_hash(canonical_url: &str) -> &str {
  let mut sh = Sha256::new();
  sh.input_str(canonical_url);
  sh.result_str()
}

fn base64_url_encode(bytes: [u8]) -> &'static str {
  bytes.to_base64(base64::URLSAFE_CHARS).as_slice()
}

// fn encode_hex_string(input: &str) -> &str {
//   input.to_string().into_bytes().to_hex() //as_slice()
// }