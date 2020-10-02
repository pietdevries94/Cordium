extern crate gtk;
extern crate glib;
extern crate gio;
extern crate webkit2gtk;

use gtk::prelude::*;
use gio::prelude::*;
use glib::clone;
use gtk::{Application, Builder, ApplicationWindow, Button, Box};
use webkit2gtk::{ WebView, WebViewExt, WebContextExt, CookieManagerExt, CookiePersistentStorage };
use std::env;
use std::path::Path;

mod config;

fn build_ui(app: &gtk::Application, c: config::Config) {
    let glade_src = include_str!("main.glade");
    let builder = Builder::from_string(glade_src);

    let window: ApplicationWindow = builder.get_object("main_window").expect("Couldn't get main_window");
    window.set_application(Some(app));

    let button_box: Box = builder.get_object("button_box").expect("Couldn't get button_box");
    let webview_box: Box = builder.get_object("webview_box").expect("Couldn't get webview_box");

    let home_folder = env::var("HOME").unwrap();
    let cache_path = Path::new(&home_folder).
        join(".cache/cordium/cookies");

    for site in c.sites {
        let name = site.name;
        let url = site.url;

        let wv: WebView = WebView::new();
        wv.set_vexpand(true);

        // Set the cookies
        let ctx = wv.get_context().expect("Couldn't get webview context");
        let cm = ctx.get_cookie_manager().expect("Couldn't get cookie manager");
        cm.set_persistent_storage(cache_path.to_str().unwrap(), CookiePersistentStorage::Text);

        wv.load_uri(&url);

        let b = Button::with_label(&name);
        b.connect_clicked(clone!(@weak wv, @strong webview_box => move |_| {
            for other_wv in  webview_box.get_children().iter() {
                other_wv.set_visible(false);
            }
            wv.set_visible(true);
        }));
        
        button_box.add(&b);
        webview_box.add(&wv);
    }

    button_box.show_all();
    window.show();
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
