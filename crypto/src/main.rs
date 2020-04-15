use crypto::{digest::Digest, sha3::Sha3};

fn main() {
    let mut hasher = Sha3::sha3_256();
    hasher.input_str("hello wolrd");
    let hash = hasher.result_str();
    println!("{}",hash);
}
