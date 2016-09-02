extern crate yup_oauth2 as oauth2;
extern crate google_youtube3 as youtube3;
use youtube3::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use youtube3::YouTube;

fn main() {
    println!("Hello, world!");
}
