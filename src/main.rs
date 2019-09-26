extern crate quick_xml;

use quick_xml::Reader;
use quick_xml::events::{Event, BytesEnd, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;
// use std::iter;
use std::fs::File;
use std::io::prelude::*;
use std::borrow::Cow;

fn main() -> std::io::Result<()> {

	let mut reader = Reader::from_file("en.xml").expect("Could not open file to be read");
	let mut writer = Writer::new(Cursor::new(Vec::new()));
	let mut buf = Vec::new();
	let mut count = 0;

	loop {
		match reader.read_event(&mut buf) {
			Ok(Event::Start(ref e)) if e.name() == b"field" => {

				// crates a new element ... alternatively we could reuse `e` by calling
				// `e.into_owned()`
				let mut elem = BytesStart::owned(b"field".to_vec(), "field".len());

				// collect existing attributes
				elem.extend_attributes(e.attributes().map(|attr| {
					let mut attr = attr.unwrap();

					if attr.key == b"language" {
						attr.value = Cow::Borrowed(b"es");
					}

					attr
				}));

				// writes the event to the writer
				assert!(writer.write_event(Event::Start(elem)).is_ok());
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

