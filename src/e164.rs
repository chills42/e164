#[derive(RustcEncodable, RustcDecodable, Show)]
pub struct NationalCode {
    pub code_length: i8,
    pub strict: bool,
    pub known_codes: Vec<String>
}

impl PartialEq for NationalCode {
    fn eq(&self, other: &NationalCode) -> bool {
        return self.code_length == other.code_length && self.strict == other.strict && self.known_codes == other.known_codes;
    }
}

#[derive(RustcEncodable, RustcDecodable, Show)]
pub struct CountryCode {
    pub code: String,
    pub national_destination_codes: NationalCode,
}

impl PartialEq for CountryCode {
    fn eq(&self, other: &CountryCode) -> bool {
        return self.code == other.code && self.national_destination_codes == other.national_destination_codes
    }
}

pub fn load() -> Vec<CountryCode>
{
    return vec!({CountryCode {code: "1".to_string(), national_destination_codes: NationalCode { code_length: 3, strict: false, known_codes: vec!({"".to_string()})}}});
}

pub fn possible(phone_number: &str) -> bool
{
    let length = phone_number.len();
    return length < 16 && length > 3;
}

pub fn split(phone_number: &str) -> [&str; 3]
{
    let length = phone_number.len();
    let country_code_section = phone_number[0..3].as_slice();
    return [phone_number[0..1].as_slice(), phone_number[1..4].as_slice(), phone_number[4..length].as_slice()];
}
