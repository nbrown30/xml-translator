// For JSON parsing
extern crate serde;
// For named command line arguments
extern crate docopt;
// For web API requests
extern crate reqwest;
// XML parsing
extern crate xml;
// XML representation
extern crate xmltree;

use reqwest::Error;
use docopt::Docopt;
use serde::{Deserialize};
use std::string::String as String;
use std::fs;
use xmltree::Element;
use xmltree::EmitterConfig;
use std::fs::File;
use std::io::Read;

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

	let xml_file = read_file_to_string().expect("Unable to open file");
	let mut content_tree = Element::parse(xml_file.as_bytes()).unwrap();
	println!("Running");

	let iso_destination_language : String = googletranslate2::get_iso_language_code(destination_language.as_str());

	for (_i, element) in content_tree.children.iter_mut().enumerate() {

		let mut field = match element.get_mut_child("field") {
			Some(x) => x,
			None => continue,
		};

		set_language_attribute(field, &iso_destination_language);

		let mut property = match field.get_mut_child("property") {
			Some(x) => x,
			None => continue,
		};

		if property.text.is_some() {
			property.text = Some("<![CDATA[<p>This is our translated content.</p>]]>".to_string());
		} else {
			property.text = Some("<![CDATA[]]>".to_string())
		}
	}

	write_xml_to_file(content_tree, "translated.xml");

	return ();

	// println!("{}", format!("Supplied {p} and {l} and {key}", p = phrase, l = destination_language, key = api_key));
	// let response_body = translate(&phrase, "english", &destination_language, &api_key);
	// match response_body {
	// 	Ok(b) => println!("{s} => {r}", s = phrase, r = b.data.translations[0].translated_text),
	// 	Err(e) => eprintln!("Error: {}", e),
	// }

	let mut phrases: Vec<String> = Vec::new();
	phrases.push(String::from("Hello everybody"));
	phrases.push(String::from("So it goes"));
	phrases.push(String::from(r#"<p>Ask her how her day was.

Ah, yes! John Quincy Adding Machine. He struck a chord with the voters when he pledged not to go on a killing spree. For one beautiful night I knew what it was like to be a grandmother. Subjugated, yet honored.

Bender, we're trying our best. Yeah, I do that with my stupidness. That's not soon enough! Well, thanks to the Internet, I'm now bored with sex. Is there a place on the web that panders to my lust for violence?
I had more, but you go ahead.

Look, everyone wants to be like Germany, but do we really have the pure strength of 'will'? When will that be? What kind of a father would I be if I said no? And when we woke up, we had these bodies. Shinier than yours, meatbag.

    I don't want to be rescued.
    I was having the most wonderful dream. Except you were there, and you were there, and you were there!
    Too much work. Let's burn it and say we dumped it in the sewer.

WINDMILLS DO NOT WORK THAT WAY! GOOD NIGHT!

Oh, I always feared he might run off like this. Why, why, why didn't I break his legs? You won't have time for sleeping, soldier, not with all the bed making you'll be doing. Anyhoo, your net-suits will allow you to experience Fry's worm infested bowels as if you were actually wriggling through them.

    Bite my shiny metal ass.
    I saw you with those two \"ladies of the evening\" at Elzars. Explain that.
    You guys realize you live in a sewer, right?</p>"#));

	let response_body = translate_multiple(&phrases, "english", &destination_language, &api_key);
	match response_body {
		Ok(b) => {
			for (i, trans) in b.data.translations.iter().enumerate() {
				println!("{0}: {1}", i, trans.translated_text);
			}
		},
		Err(e) => eprintln!("Error: {}", e),
	}
}

fn set_language_attribute(elem: &mut Element, iso_lang: &String) {
	if elem.attributes.contains_key("language") {
		elem.attributes.insert("language".to_owned(), iso_lang.to_string());
		assert_eq!(elem.attributes[&"language".to_string()], iso_lang.clone());
	}
}

fn write_xml_to_file(content_tree: Element, output_path: &str) {
	let xml_config = EmitterConfig {
		perform_escaping: false,
		perform_indent: true,
		..EmitterConfig::default()
	};

	// let mut buf = Vec::new();
	// content_tree.write_with_config(&mut buf, xml_config).unwrap();

	// let s = String::from_utf8(buf).unwrap();
 	// println!("{}", s);
	content_tree.write_with_config(File::create(output_path).unwrap(), xml_config);
}

fn read_file_to_string() -> Result<String, Box<dyn std::error::Error + 'static>>  {
	let foo = fs::read_to_string("en.xml")?.parse()?;
    Ok(foo)
}

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
             .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}

fn translate(phrase: &str, source_language: &str, destination_language: &str, api_key: &str) -> Result<googletranslate2::Response, Error> {
	let url: String = get_request_url(&phrase, &source_language, &destination_language, &api_key);
	// println!("Sending request to: {}", url);
	let mut res = reqwest::get(&url)?;

	if !(res.status().is_success()) {
		eprintln!("There was a problem getting results: {}", res.status());
	}

	Ok(res.json()?)
}


fn get_request_url(phrase: &str, source_language: &str, destination_language: &str, api_key: &str) -> String {
	let to_translate: Vec<String> = vec![phrase.to_string()];
	return get_request_url_for_multiple_translations(&to_translate, &source_language, &destination_language, &api_key);
}

fn translate_multiple(to_translate: &Vec<String>, source_language: &str, destination_language: &str, api_key: &str) -> Result<googletranslate2::Response, Error> {
	let url: String = get_request_url_for_multiple_translations(&to_translate, &source_language, &destination_language, &api_key);
	// println!("Sending request to: {}", url);
	let mut res = reqwest::get(&url)?;

	if !(res.status().is_success()) {
		eprintln!("There was a problem getting results: {}", res.status());
	}

	Ok(res.json()?)
}

fn get_request_url_for_multiple_translations(to_translate: &Vec<String>, source_language: &str, destination_language: &str, api_key: &str) -> String {
	let mut url = GOOGLE_TRANSLATE_URL.to_owned();
	url.push_str("?");
	url.push_str(&format!("&key={}", api_key));
	url.push_str(&format!("&target={}", googletranslate2::get_iso_language_code(destination_language)));
	url.push_str(&format!("&source={}", googletranslate2::get_iso_language_code(source_language)));

	for (_i, phrase) in to_translate.iter().enumerate() {
		url.push_str(&format!("&q={}", phrase.as_str()));
	}

	println!("{}", url);

	return url;
}
