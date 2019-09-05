// For HTTP requests
extern crate hyper;
// For JSON parsing
extern crate serde;
// For named command line arguments
extern crate docopt;
use docopt::Docopt;
use serde::Deserialize;
use std::string::String as String;


const GOOGLE_TRANSLATE_URL: &'static str = "https://translation.googleapis.com/language/translate/v2";
const USAGE: &'static str = r#"
Google Translate

Usage:
  ./translate <phrase> <destination-language> <api-key>
  ./translate -h | --help

Options:
  -h --help
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
    let language = args.arg_destination_language;
    let api_key = args.arg_api_key;

    println!("{}", format!("Supplied {p} and {l} and {key}", p = phrase, l = language, key = api_key));
    let url: String = get_request_url(&phrase, &language, &api_key);
}

fn get_request_url(phrase: &str, destination_language: &str, api_key: &str) -> String {
	let mut url = GOOGLE_TRANSLATE_URL.to_owned();
	url.push_str("?");
	url.push_str(&format!("&key={}", api_key));
	url.push_str(&format!("&q={}", phrase));
	url.push_str(&format!("&target={}", get_iso_language_code(destination_language)));
	url.push_str(&format!("&source=en"));

	println!("{}", url);
	return url;
}

fn get_iso_language_code(lang: &str) -> String {
	let lower_lang = lang.to_ascii_lowercase();
	let language = lower_lang.as_str();
	match language {
		"spanish" => "es".to_string(),
		"english" => "en".to_string(),
		"japanese" => "ja".to_string(),
		"french" => "fr".to_string(),
		"korean" => "ko".to_string(),
		"mandarin" => "zh-cn".to_string(),
		"german" => "de".to_string(),
		_ => panic!("Unexpected language type")
	}
}