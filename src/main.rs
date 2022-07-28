use std::env;
use std::str;
use chrono::Utc;
use chrono::DateTime;
use openssl::ec::{EcKey,EcGroup, EcPoint};
use openssl::nid::Nid;
use openssl::symm::Cipher;

fn main() {
    // pass in a UID for the transaction ID or interaction marker
    // generate a private and public EC key pair
    //
    let args: Vec<String> = env::args().collect();
    let bid = args.clone().into_iter().nth(1);
    let group = EcGroup::from_curve_name(Nid::SECP384R1).unwrap();
    let key = EcKey::generate(&group).unwrap();
    let mut ctx = openssl::bn::BigNumContext::new().unwrap();
    let bytes = key.public_key().to_bytes(&group,
        openssl::ec::PointConversionForm::COMPRESSED, &mut ctx).unwrap();
    let public_key = EcPoint::from_bytes(&group, &bytes, &mut ctx).unwrap();
    let ec_key = EcKey::from_public_key(&group, &public_key).unwrap();
    let pem = ec_key.public_key_to_pem().unwrap();
    let private_pem = key.private_key_to_pem_passphrase(Cipher::aes_128_cbc(), b"CHANGEME").unwrap();
    let datas = match str::from_utf8(&pem) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence for public key: {}", e),
    };
    let datap = match str::from_utf8(&private_pem) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence for private key: {}", e),
    };

    let gen_date: DateTime<Utc> = Utc::now();
    println!("{} - {:?} - Thistle Field: Milk Thistle Microservice - secp384r1 generation >-> {:?}, {:?}", gen_date, bid, datap, datas);

}
