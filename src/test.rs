use crate::prelude::{decode_ekey_util, encode_ekey_util, init_encrypt_conf};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file(path: &str) -> Vec<String> {
    let f1: BufReader<File> =
        BufReader::new(File::open(path).expect(&format!("Not able to read file : {}", path)));
    let mut lines = vec![];
    for it in f1.lines() {
        lines.push(it.unwrap())
    }
    lines
}

fn read_secret_key() -> String {
    let secret_key = read_file("./secret_key.txt");
    secret_key
        .get(0)
        .map(|x| x.to_string())
        .expect("Could not found secret key")
}

fn read_tests(path: &str) -> Vec<(u64, String)> {
    read_file(path)
        .into_iter()
        .map(|x| {
            let t = x.split(",").collect::<Vec<&str>>();
            (t[0].parse::<u64>().unwrap(), t[1].trim().to_string())
        })
        .collect::<Vec<(u64, String)>>()
}

#[test]
fn encrypted_id() {
    let secret_key: &str = &read_secret_key();
    init_encrypt_conf(secret_key);
    for (i, expected) in read_tests("./test.txt") {
        match decode_ekey_util(&expected, &format! {"{}", i}) {
            Ok(decoded) => assert_eq!(decoded, i),
            Err(err) => println!("{:?}", err),
        };
        match encode_ekey_util(i, &format! {"{}", i}) {
            Ok(encoded) => assert_eq!(encoded, expected), // println!("{}, {}", i, encoded),
            Err(_) => assert!(false),
        };
    }
}

#[test]
fn enc_test() {
    init_encrypt_conf("df(vh!3*8e21@qca#3)w#7ta*z#!bhsde43&#iez3sf5m1#h6l");
    let ekey = encode_ekey_util(5, "sub_key_foo").unwrap();
    let dkey = decode_ekey_util(&ekey, "sub_key_foo").unwrap();
    assert_eq!("E86VGQhfxb_9rxSfjnBqKg", ekey);
    assert_eq!(5, dkey);
}

// Took 30 seconds for 1 million keys
#[ignore]
#[test]
fn ency_performance() {
    let secret_key: &str = &read_secret_key();
    init_encrypt_conf(secret_key);
    let st = std::time::Instant::now();
    for i in 0..1000000 {
        assert!(match encode_ekey_util(i, &format! {"{}", i}) {
            Ok(_) => true,
            Err(_) => false,
        });
        // println!("{}, {}", i, t)
    }
    println!("Time taken: {:?}", std::time::Instant::now() - st);
}

// Took 33 Seconds
#[ignore]
#[test]
fn decy_performance() {
    let secret_key: &str = &read_secret_key();
    init_encrypt_conf(secret_key);
    let tests = read_tests("10_6_ency_keys.txt");
    let st = std::time::Instant::now();
    for (i, expected) in tests {
        match decode_ekey_util(&expected, &format! {"{}", i}) {
            Ok(decoded) => assert_eq!(decoded, i),
            Err(err) => println!("{:?}", err),
        };
    }
    println!("Time taken: {:?}", std::time::Instant::now() - st);
}

// Took around 70 seconds
#[ignore]
#[test]
fn enc_dec_performance() {
    let secret_key: &str = &read_secret_key();
    init_encrypt_conf(secret_key);
    let tests = read_tests("10_6_ency_keys.txt");
    let st = std::time::Instant::now();
    for (i, expected) in tests {
        match decode_ekey_util(&expected, &format! {"{}", i}) {
            Ok(decoded) => assert_eq!(decoded, i),
            Err(err) => println!("{:?}", err),
        };
        match encode_ekey_util(i, &format! {"{}", i}) {
            Ok(encoded) => assert_eq!(encoded, expected),
            Err(_) => assert!(false),
        };
    }
    println!("Time taken: {:?}", std::time::Instant::now() - st);
}
