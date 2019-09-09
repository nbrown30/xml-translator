// For JSON parsing
extern crate serde;
// For named command line arguments
extern crate docopt;
// For web API requests
extern crate reqwest;

use reqwest::Error;
use docopt::Docopt;
use serde::{Deserialize};
use std::string::String as String;

mod googletranslate2;
pub type TranslateResult = Result<googletranslate2::Response, reqwest::Error>;

const GOOGLE_TRANSLATE_URL: &'static str = "https://translation.googleapis.com/language/translate/v2";
const USAGE: &'static str = r#"
NAME:
  Google Translate - Translate a phrase using the Google Translate API

USAGE:
  ./translate <phrase> <destination-language> <api-key>
  ./translate -h | --help

OPTIONS:
phrase
  what you want to translate

destination-language
  the language you are translating to

api-key
  your Google Translate API key (go to console.cloud.google.com to generate one)

-h --help
  displays this information

EXAMPLE:
  ./translate "Hello World" Spanish abc
    > Hola Mundo

  ./translate "Everything was beautiful and nothing hurt" japanese abc
    > すべてが美しく、何も傷つかなかった。
"#;

#[derive(Deserialize)]
struct Args {
    arg_phrase: String,
    arg_destination_language: String,
    arg_api_key: String
}


fn main() {
	let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let phrase = args.arg_phrase;
    let destination_language = args.arg_destination_language;
    let api_key = args.arg_api_key;

    // println!("{}", format!("Supplied {p} and {l} and {key}", p = phrase, l = destination_language, key = api_key));
    let response_body = translate(&phrase, "english", &destination_language, &api_key);
    match response_body {
    	Ok(b) => println!("{s} => {r}", s = phrase, r = b.data.translations[0].translated_text),
    	Err(e) => eprintln!("Error: {}", e),
    }
}

fn translate(phrase: &str, source_language: &str, destination_language: &str, api_key: &str) -> Result<googletranslate2::Response, Error> {
	let url: String = get_request_url_with_source_and_destination(&phrase, &source_language, &destination_language, &api_key);
    // println!("Sending request to: {}", url);
    let mut res = reqwest::get(&url)?;

    if !(res.status().is_success()) {
    	eprintln!("There was a problem getting results: {}", res.status());
    }

    Ok(res.json()?)
}


fn get_request_url_with_source_and_destination(phrase: &str, source_language: &str, destination_language: &str, api_key: &str) -> String {
	let mut url = GOOGLE_TRANSLATE_URL.to_owned();
	url.push_str("?");
	url.push_str(&format!("&key={}", api_key));
	url.push_str(&format!("&q={}", phrase));
	url.push_str(&format!("&target={}", googletranslate2::get_iso_language_code(destination_language)));
	url.push_str(&format!("&source={}", googletranslate2::get_iso_language_code(source_language)));

	println!("{}", url);
	return url;
}

