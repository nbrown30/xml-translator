extern crate quick_xml;
extern crate regex;

use quick_xml::Reader;
use quick_xml::events::{Event, BytesEnd, BytesStart, BytesText};
use quick_xml::Writer;
use std::io::Cursor;
// use std::iter;
use std::fs::File;
use std::io::prelude::*;
use std::borrow::Cow;
use std::str::from_utf8;
use regex::Regex;

fn main() -> std::io::Result<()> {

	let mut reader = Reader::from_file("en.xml").expect("Could not open file to be read");
	let mut writer = Writer::new(Cursor::new(Vec::new()));
	let mut buf = Vec::new();
	let merge_field_pattern = Regex::new(r"\{!.+}").unwrap();

	loop {
		match reader.read_event(&mut buf) {
			Ok(Event::Start(ref e)) if e.name() == b"field" => {

				// crates a new element ... alternatively we could reuse `e` by calling
				// `e.into_owned()`
				let mut elem = BytesStart::owned(b"field".to_vec(), "field".len());

				elem.extend_attributes(e.attributes().map(|attr| {
						let mut attr = attr.unwrap();

						if attr.key == b"language" {
							attr.value = Cow::Borrowed(b"es");
						}

						return attr;
					})
				);

				// writes the event to the writer
				assert!(writer.write_event(Event::Start(elem)).is_ok());
			},
			Ok(Event::CData(ref e)) => if e.len() != 0 {
				let cdata_inner_text = from_utf8(e).unwrap();

				if !merge_field_pattern.is_match(cdata_inner_text) {
					println!("CData({:?})", cdata_inner_text);
					let elem = BytesText::from_plain(b"Replaced");

					assert!(writer.write_event(Event::CData(elem)).is_ok());
				}
			},
			Ok(Event::End(ref e)) if e.name() == b"field" => {
				assert!(writer.write_event(Event::End(BytesEnd::borrowed(b"field"))).is_ok());
			},
			Ok(Event::Eof) => break,
			Ok(e) => assert!(writer.write_event(e).is_ok()),
			Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
		}
		buf.clear();
	}

	let mut file = File::create("output.xml")?;
	file.write_all(writer.into_inner().get_ref())?;

	Ok(())
}

