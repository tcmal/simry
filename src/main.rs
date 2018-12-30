#![feature(try_from)]

extern crate gio;
extern crate gtk;
extern crate glib;

mod buffer;
mod editor_window;

use std::env::args;

use gtk::Application;
use gio::ApplicationFlags;
use gio::prelude::*;
use std::rc::Rc;

use editor_window::EditorWindow;

fn main() {    
    let application = Rc::new(Application::new("com.tcmal.simry", ApplicationFlags::FLAGS_NONE)
    	.unwrap());

    let app = application.clone();
	application.connect_startup(move |_| {
        let c = EditorWindow::new(&app);

        {
	        let mut window = c.lock().unwrap();
	        window.add_empty_buffer(true);
	        window.add_empty_buffer(true);
	        window.add_empty_buffer(true);
        }

        gtk::main();
    });
	application.connect_activate(|_| {});

	application.run(&args().collect::<Vec<_>>());
}
