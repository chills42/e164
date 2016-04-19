//! The e164 crate provides parsing and validation of e164 formatted phone
//! numbers.
//!
//! # Examples
//!
//! ```
//! use e164::*;
//! assert!(e164::Validator::default().possible("12341234"));
//! ```
//!
//! ```
//! use e164::*;
//! let validator = e164::Validator::default();
//! assert_eq!(validator.split("18002345678"), ["1", "800", "2345678"]);
//! ```

use std::io::Write;
use std::path::Path;
use std::fs::File;
use rustc_serialize::json;

#[derive(RustcEncodable, RustcDecodable, Debug, PartialEq)]
pub struct NationalCode {
    pub code_length: usize,
    pub strict: bool,
    pub known_codes: Vec<String>,
}

#[derive(RustcEncodable, RustcDecodable, Debug, PartialEq)]
pub struct CountryCode {
    pub code: String,
    pub national_destination_codes: NationalCode,
}

#[derive(RustcEncodable, RustcDecodable, Debug, PartialEq)]
pub struct Validator {
    pub country_codes: Vec<CountryCode>,
}

impl Validator {

    /// Loads a default validator
    ///
    /// # Examples
    ///
    /// ```
    /// use e164::*;
    ///
    /// e164::Validator::default();
    /// ```
    pub fn default() -> Validator {
        return Validator {
            country_codes: vec![{
                                    CountryCode {
                                        code: "1".to_string(),
                                        national_destination_codes: NationalCode {
                                            code_length: 3,
                                            strict: false,
                                            known_codes: vec![{
                                                                  "".to_string()
                                                              }],
                                        },
                                    }
                                }],
        };
    }

    pub fn possible<'a>(&'a self, phone_number: &'a str) -> bool {
        let length = phone_number.len();
        return length < 16 && length > 3;
    }

    pub fn split<'a>(&'a self, phone_number: &'a str) -> [&str; 3] {
        let length = phone_number.len();
        let mut cc_end = 0;
        let mut nc_end = 0;
        for code in self.country_codes.iter() {
            let code_length = code.code.len();
            if code.code == phone_number[0..code_length] {
                let destination_code_end = code_length +
                                           code.national_destination_codes.code_length;
                cc_end = code_length;
                nc_end = destination_code_end;
                break;
            }
        }
        if cc_end == 0 || nc_end == 0 {
            panic!("Unable to split the given phone number using the available country code list");
        }
        [&phone_number[0..cc_end], &phone_number[cc_end..nc_end], &phone_number[nc_end..length]]
    }

    /// Export the current Validator to the specified path
    ///
    /// ```
    /// use e164::*;
    /// use std::path::Path;
    ///
    /// e164::Validator::default().export_path(&Path::new("export_file.json"));
    /// ```
    pub fn export_path(&self, path: &Path) {
        let mut file = File::create(path).unwrap();
        self.export(&mut file);
    }

    pub fn export<T>(&self, writer: &mut T)
        where T: Write
    {
        writer.write_all(&json::encode(self).unwrap().into_bytes()).ok();
    }
}
