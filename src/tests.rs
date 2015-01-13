use e164;
use rustc_serialize::json;

#[test]
fn it_works() {
    let parsed = e164::split("13176376397".as_slice());
    assert!(parsed[0] == "1");
    assert!(parsed[1] == "317");
}

#[test]
fn possible_check_works() {
    assert!(e164::possible("447330153833333"));
}

#[test]
#[should_fail]
fn possible_check_catches_long() {
    assert!(e164::possible("4473300011177788"));
}

#[test]
#[should_fail]
fn possible_check_catches_short() {
    assert!(e164::possible("123"));
}

#[test]
fn serialize_country_code() {
    let cc = e164::CountryCode { code: "1".to_string(), national_destination_codes: e164::NationalCode {code_length: 3, strict: false, known_codes: vec!({"".to_string()})}};
    let encoded = json::encode(&cc);
}

#[test]
fn reserialize_country_code() {
    let cc = e164::CountryCode { code: "1".to_string(), national_destination_codes: e164::NationalCode {code_length: 3, strict: false, known_codes: vec!({"".to_string()})}};
    let reserialized = json::decode(json::encode(&cc).as_slice()).unwrap();
    assert_eq!(cc, reserialized);
}
