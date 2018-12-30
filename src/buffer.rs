use std::path::PathBuf;
use std::fs::File;
use std::sync::RwLock;
use std::convert::TryInto;
use std::io::Read;

use gtk::{TextBuffer, TextBufferExt};

#[derive(Debug)]
pub struct Buffer {
	/// The file this buffer is for, if applicable.
	file: Option<File>,

	/// The path of the file this is for, if applicable.
	path: Option<PathBuf>,

	/// The contents of this buffer.
	contents: RwLock<String>,

	/// The GTK buffer
	pub ui_buffer: TextBuffer
}

impl Buffer {
	/// Try to make a new buffer from a path.
	/// The file at that path must be UTF-8. (TODO)
	/// Returns the new buffer or nothing if there was an error reading.
	pub fn from_path<'a>(path: PathBuf) -> Result<Buffer, std::io::Error> {
		// Open the file
		let mut file = File::open(path.as_path())?;

		// Get the filename
		let meta = file.metadata()?;

		// Read the file
		let size = meta.len();

		let mut buf: Vec<u8> = Vec::with_capacity(size.try_into().unwrap());
		file.read_to_end(&mut buf)?;

		let buf = RwLock::new(String::from_utf8(buf).unwrap());

		// Make the UI counterpart
		let ui_buffer = TextBuffer::new(None);
		ui_buffer.set_text(buf.read().unwrap().as_ref());

		Ok(Buffer { file: Some(file), contents: buf, ui_buffer, path: Some(path) })
	}

	/// Create a new empty buffer
	pub fn empty() -> Buffer {
		let buf = RwLock::new(String::new());

		let ui_buffer = TextBuffer::new(None);
		ui_buffer.set_text(buf.read().unwrap().as_ref());

		Buffer { file: None, contents: buf, ui_buffer, path: None }
	}

	/// Get the title of this buffer
	pub fn button_name(&self) -> &str {
		if let Some(ref path) = self.path {
			path.to_str().unwrap()
		} else {
			"untitled"
		}
	}
}