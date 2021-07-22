use std::fs::File;
use std::io::{BufRead, BufReader};
use blake2b_ref::Blake2bBuilder;

/*
pub fn blake2b_256(s: &[u8]) -> [u8; 32] {
    let mut result = [0u8; CKB_HASH_DIGEST];
    let mut blake2b = Blake2bBuilder::new(CKB_HASH_DIGEST)
        .personal(CKB_HASH_PERSONALIZATION)
        .build();
    blake2b.update(s);
    blake2b.finalize(&mut result);
    result
}
*/
fn blake2b_das(s: &[u8]) -> [u8; 32] {
    let mut result = [0u8; 32];
    let mut blake2b = Blake2bBuilder::new(32)
        .personal(b"2021-07-22 12:00")
        .build();
    blake2b.update(s);
    blake2b.finalize(&mut result);
    result
}

fn check(account: &str) -> bool {
    if account.len() >= 10 {
        return false;
    }
    if account.len() < 5 {
        return false;
    }
    let suffix: &str = ".bit";
    let acc: Vec<u8> = [
        account.as_bytes().to_vec(),
        suffix.as_bytes().to_vec()
    ]
    .concat();
    let hash = blake2b_das(acc.as_slice());
    let lucky_num = hash[0];

    return lucky_num <= 12;
}

fn check_wordlist() {
    let file = File::open("wordlist.txt").expect("file not found");
    let file = BufReader::new(file);
    for line in file.lines().filter_map(|result| result.ok()) {
        //println!("{}, {}", line, line.len());
        if check(&line) {
            println!("{}: true", line);
        }
    }
}

fn check_numbers() {
    for i in 0..100000 {
        let mut s: String = i.to_string();
        s = format!("{:0>5}", s);
        if check(&s) {
            println!("{}: true", s);
        }
    }
}

fn main() {
    check_numbers();
}
