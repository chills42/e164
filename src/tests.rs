use e164;
use rustc_serialize::json;

#[test]
fn us_number_works() {
    let validator = e164::Validator::default();
    let parsed = validator.split(&"13176376397");
    assert!(parsed[0] == "1");
    assert!(parsed[1] == "317");
}

#[test]
#[should_panic]
fn unassigned_number_fails() {
    let validator = e164::Validator::default();
    // We are trying to split an 895 country code, which is in a block reserved for future country
    // code expansion
    validator.split(&"8951234567890");
}

#[test]
fn possible_check_works() {
    let validator = e164::Validator::default();
    assert!(validator.possible("447330153833333"));
}

#[test]
#[should_panic]
fn possible_check_catches_long() {
    let validator = e164::Validator::default();
    assert!(validator.possible("4473300011177788"));
}

#[test]
#[should_panic]
fn possible_check_catches_short() {
    let validator = e164::Validator::default();
    assert!(validator.possible("123"));
}

#[test]
fn serialize_country_code() {
    let cc = e164::CountryCode { code: "1".to_string(), national_destination_codes: e164::NationalCode {code_length: 3, strict: false, known_codes: vec!({"".to_string()})}};
    json::encode(&cc).unwrap();
}

#[test]
fn reserialize_country_code() {
    let cc = e164::CountryCode { code: "1".to_string(), national_destination_codes: e164::NationalCode {code_length: 3, strict: false, known_codes: vec!({"".to_string()})}};
    let reserialized = json::decode(&json::encode(&cc).unwrap()).unwrap();
    assert_eq!(cc, reserialized);
}

#[test]
fn load_validator_with_data()
{
    let validator = e164::Validator::default();
    let encoded = json::encode(&validator).unwrap();
    let decoded : e164::Validator = json::decode(&encoded).unwrap();
    assert_eq!(validator, decoded);
}
