/* Example response:
	"data": {
		"translations": [
			{
				"translatedText": "Hola Mundo!"
			}
		]
	}
}
*/
#[derive(serde::Deserialize, Debug)]
pub struct Response {
	pub data: Data,
}

#[derive(serde::Deserialize, Debug)]
pub struct Data {
	pub translations: Vec<Translation>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Translation {
	#[serde(rename="translatedText")]
	pub translated_text: String,
}

pub fn get_iso_language_code(lang: &str) -> String {
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