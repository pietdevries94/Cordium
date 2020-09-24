extern crate gtk;
extern crate glib;
extern crate gio;
extern crate webkit2gtk;

use gtk::prelude::*;
use gio::prelude::*;
use glib::clone;
use gtk::Orientation::Vertical;
use gtk::{Application, ApplicationWindow, Button};
use webkit2gtk::{ WebView, WebViewExt };

mod config;

fn build_ui(app: &gtk::Application, c: config::Config) {
    let window = ApplicationWindow::new(app);
    window.set_title("First GTK+ Program");

    let vbox = gtk::Box::new(Vertical, 0);

    let wv: WebView = WebView::new();
    wv.set_size_request(200, 200);

    for site in c.sites {
        let name = site.name;
        let url = site.url;

        let b = Button::with_label(&name);
        b.connect_clicked(clone!(@weak wv, @strong url => move |_| {
            wv.load_uri(&url);
        }));
        vbox.add(&b);
    }

    vbox.add(&wv);

    window.add(&vbox);
    window.show_all();
}

fn main() {

    let application = Application::new(
        Some("com.github.pietdevries94.cordium"),
        Default::default(),
    ).expect("failed to initialize Cordium");

    application.connect_activate(|app| {
        let c = config::read_config();
        build_ui(app, c);
    });

    application.run(&[]);
}
