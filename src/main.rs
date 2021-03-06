extern crate gtk;
extern crate glib;
extern crate gio;
extern crate webkit2gtk;

use gtk::prelude::*;
use gio::prelude::*;
use glib::clone;
use gtk::{Application, Builder, ApplicationWindow, Button, Box};
use webkit2gtk::{ WebView, WebViewExt, WebContextExt, CookieManagerExt, CookiePersistentStorage, SecurityOrigin, NotificationExt };
use std::env;
use std::path::Path;
use gio::{FileIcon, File};

mod config;

fn build_ui(app: &gtk::Application, c: config::Config) {
    let glade_src = include_str!("main.glade");
    let builder = Builder::from_string(glade_src);

    let window: ApplicationWindow = builder.get_object("main_window").expect("Couldn't get main_window");
    window.set_application(Some(app));

    let button_box: Box = builder.get_object("button_box").expect("Couldn't get button_box");
    let webview_box: Box = builder.get_object("webview_box").expect("Couldn't get webview_box");

    let home_folder = env::var("HOME").unwrap();
    let cookie_path = Path::new(&home_folder).
        join(".cache/cordium/cookies");

    let default_dark_mode = c.dark_mode.unwrap_or(false);
    let dark_mode_script = "
        (async () => {
            await import('https://unpkg.com/darkreader/darkreader.js');
            DarkReader.enable();
        })();
    ";

    for site in c.sites {
        let name = site.name;
        let url = site.url;

        let wv = WebView::new();
        wv.set_vexpand(true);

        // Set the cookies
        let ctx = wv.get_context().expect("Couldn't get webview context");
        let cm = ctx.get_cookie_manager().expect("Couldn't get cookie manager");
        cm.set_persistent_storage(cookie_path.to_str().unwrap(), CookiePersistentStorage::Text);

        // Notifications
        let allowed = &[&SecurityOrigin::new_for_uri(url.as_str())]; 
        let disallowed = &[];
        ctx.initialize_notification_permissions(allowed, disallowed);

        // Edit notifications to show where they are comming from
        let mut base_title = name.to_string();
        base_title.push_str("\n");
        let base_app = window.get_application().expect("Somehow there is no application");
        let icon_path = site.icon_path.unwrap_or_default();
        wv.connect_show_notification(move |_, original| {
            let mut title = base_title.to_string();
            title.push_str(&original.get_title().expect("").to_string());
            let n = gio::Notification::new(&title);

            let gs = original.get_body();
            if gs.is_some() {
                n.set_body(Some(& gs.expect("").to_string()));
            }

            let f = File::new_for_path(&icon_path);
            let fi = FileIcon::new(&f);
            n.set_icon(&fi);

            base_app.send_notification(None, &n);
            
            true
        });

        // Set darkmode
        if site.dark_mode.unwrap_or(default_dark_mode) {
            wv.connect_load_changed(move |s, _| {
                let cancellable = gio::Cancellable::new();
                s.run_javascript(dark_mode_script, Some(&cancellable), move |_| {});
            });
        }
        // Always set background black when using global dark, even when dark mode of specific app is disabled to prevent white flashes
        if default_dark_mode || site.dark_mode.unwrap_or(false) {
            wv.set_background_color(&gdk::RGBA::black());
        }
        wv.load_uri(&url);

        let b = Button::with_label(&name);
        b.connect_clicked(clone!(@weak wv, @strong webview_box, @strong url => move |_| {
            // Go back to original url by clicking on active tab
            if wv.get_visible() == true {
                wv.load_uri(&url);
                return;
            };
            for other_wv in webview_box.get_children().iter() {
                other_wv.set_visible(false);
            }
            wv.set_visible(true);
        }));
        
        button_box.add(&b);
        webview_box.add(&wv);
    }

    button_box.show_all();
    webview_box.get_children().first().expect("There are no webviews").set_visible(true);
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
