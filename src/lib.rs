extern crate dumb_crypto;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn derive(r: usize, n: usize, p: usize, passphrase: &str, salt: &str,
              out_size: usize) -> Vec<u8> {
    let mut out: Vec<u8> = vec![0; out_size];

    let s = dumb_crypto::scrypt::Scrypt::new(r, n, p).unwrap();

    s.derive(passphrase.as_bytes(), salt.as_bytes(), &mut out);

    out
}
