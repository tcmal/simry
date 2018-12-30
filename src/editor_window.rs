use gtk::Application;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use gtk::{Builder, CssProvider, Button, Box, ButtonBox, TextView, ApplicationWindow};
use gtk::prelude::*;
use gio::prelude::*;

use buffer::Buffer;

const CLASS_TAB_ACTIVE: &str = "active";
const CLASS_TAB: &str = "tab";

/// Owns all the data & UI state for an editor window.
#[derive(Debug)]
pub struct EditorWindow {
	/// The buffers from this editor window
	buffers: Vec<Buffer>,

	/// The open buffer
	selected_buffer: usize,

	/// The tabs for each buffer. Indexes will match with buffers array.
	buffer_buttons: Vec<BufferTab>,

	/// The GTK Window
	window: ApplicationWindow,

	/// The GTK Layout
	layout: Box,

	/// The GTK Text View
	view: TextView,

	/// The container for all buffer tabs
	tabbar: ButtonBox,

	/// A reference to this object, for UI events to clone.
	self_ref: Option<Arc<Mutex<EditorWindow>>>
}

impl EditorWindow {
	/// Make a new editor window.
	/// Ownership is always shared.
	pub fn new(application: &Application) -> Arc<Mutex<EditorWindow>> {
		// Construct the UI
		let glade = include_str!("ui/editor_window.glade");
		let builder = Builder::new();
		builder.add_from_string(glade).unwrap();

		let window: ApplicationWindow = builder.get_object("main-window").unwrap();
		let layout: Box = builder.get_object("box").unwrap();
		let view: TextView = builder.get_object("editor-view").unwrap();
		let tabbar: ButtonBox = builder.get_object("tab-bar").unwrap();

		application.add_window(&window);
		window.connect_delete_event(|_, _| {
			// Stop the main loop.
			gtk::main_quit();
			// Let the default handler destroy the window.
			Inhibit(false)
		});

		// let action = gio::SimpleAction::new("test", None);
		// action.connect_activate(|_, _| {
		// 	println!("test action");
		// });

		// window.add_action(&action);

		window.show_all();

		// Include the CSS
		let css_provider = CssProvider::new();
		css_provider.connect_parsing_error(|_, _, _| {
			panic!("Error parsing Editor Window CSS!");
		});
		css_provider.load_from_data(include_bytes!("ui/editor_window.css")).unwrap();
		gtk::StyleContext::add_provider_for_screen(&window.get_screen().unwrap(), &css_provider, gtk::STYLE_PROVIDER_PRIORITY_USER);

		
		// Make the instance.
		let editor_window = EditorWindow {
			buffers: Vec::new(),
			selected_buffer: 0,
			buffer_buttons: Vec::new(), layout, tabbar, view, window,
			self_ref: None
		};

		// Then give it a reference to itself.
		let arc = Arc::new(Mutex::new(editor_window));
		arc.lock().unwrap().self_ref = Some(arc.clone());

		arc
	}

	/// Add an empty buffer to the window.
	pub fn add_empty_buffer(&mut self, select: bool) {
		self.add_buffer(Buffer::empty(), select);
	}

	/// Open the given file in the window.
	pub fn open_file(&mut self, path: PathBuf, select: bool) -> Result<usize, std::io::Error> {
		let buf = Buffer::from_path(path)?;

		Ok(self.add_buffer(buf, select))
	}

	/// Select a buffer, refreshing the UI.
	pub fn select_buffer(&mut self, buf_index: usize) {
		if buf_index != self.selected_buffer {
			self.buffer_buttons[self.selected_buffer].set_active(false);
		}
		
		self.buffer_buttons[buf_index].set_active(true);
		
		self.selected_buffer = buf_index;

		self.view.set_buffer(Some(&self.buffers[self.selected_buffer].ui_buffer));
	}

	/// Adds a buffer, selecting it if necessary. Returns it's index.
	fn add_buffer(&mut self, buf: Buffer, select: bool) -> usize { 
		self.buffers.push(buf);

		let index = self.buffers.len() - 1;
		self.make_buffer_tab(index);

		if select {
			self.select_buffer(index);
		}

		index
	}

	/// Adds the tab for a buffer
	fn make_buffer_tab(&mut self, buf_index: usize) -> () {
		let buf = &self.buffers[buf_index];
		self.buffer_buttons.push(BufferTab::new(buf, buf_index, self.self_ref.clone().unwrap()));

		self.tabbar.pack_end(&self.buffer_buttons[buf_index].btn, false, true, 0);
		&self.buffer_buttons[buf_index].show();
	}
}

/// Thin wrapper for a tab.
#[derive(Debug)]
struct BufferTab {
	/// The button this wraps.
	pub btn: Button
}

impl BufferTab {
	/// Make a new buffertab. Internally, this makes the button and connects events.
	fn new(buf: &Buffer, buf_index: usize, op: Arc<Mutex<EditorWindow>>) -> BufferTab {
		let btn = Button::new_with_label(buf.button_name());

		btn.get_style_context().unwrap().add_class(CLASS_TAB);

		btn.connect_clicked(move |_| {
			op.lock().unwrap().select_buffer(buf_index);
		});

		BufferTab { btn}
	}

	/// Convenience function, show the button this wraps.
	fn show(&self) {
		self.btn.show()
	}

	/// Convenience function, update CSS class of button.
	fn set_active(&mut self, active: bool) {
		if active {
			self.btn.get_style_context().unwrap().add_class(CLASS_TAB_ACTIVE);
		} else {
			self.btn.get_style_context().unwrap().remove_class(CLASS_TAB_ACTIVE);
		}
		self.btn.show();
	}
}