use std::{
    fs,
    path::{Path, PathBuf},
};

use gdk::{Display, RGBA, builders::RGBABuilder};
use gio::{Cancellable, ffi::GListStore};
use glib::{Bytes, GStringPtr, enums::EnumValuesStorage};
use gtk::{CssProvider, FileDialog, StringObject};
use libadwaita::{self as adw, ApplicationWindow, prelude::AdwApplicationWindowExt};
use webkit6::{
    FileChooserRequest, LoadEvent, NetworkProxySettings, NetworkSession, WebView, prelude::*,
};

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_bytes(&"* { background-color: #000; }".as_bytes().into());

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn create_webview(window: &ApplicationWindow) -> WebView {
    let webview = WebView::builder().build();

    let settings = WebViewExt::settings(&webview).unwrap();
    settings.set_enable_developer_extras(true);
    settings.set_enable_page_cache(false);
    settings.set_disable_web_security(true);
    webview.set_background_color(&RGBA::new(0.0, 0.0, 0.0, 1.0));

    webview.load_uri("https://meexreay.github.io/poshlostios");

    webview
}

fn main() {
    let app = adw::Application::builder()
        .application_id("ru.themixray.poshlostios-webkitgtk6")
        .build();

    app.connect_activate(move |app| {
        let window = adw::ApplicationWindow::new(app);

        load_css();

        window.set_default_size(500, 500);
        window.set_fullscreened(true);

        let webview = create_webview(&window);
        window.set_content(Some(&webview));

        let ctrl_shift_i = gtk::Shortcut::builder()
            .trigger(&gtk::ShortcutTrigger::parse_string("<Control><Shift>i").unwrap())
            .action(&gtk::CallbackAction::new({
                let webview = webview.clone();
                move |_, _| -> glib::Propagation {
                    let inspector = webview.inspector().unwrap();
                    inspector.show();

                    glib::Propagation::Stop
                }
            }))
            .build();

        let controller = gtk::ShortcutController::new();
        controller.add_shortcut(ctrl_shift_i);
        webview.add_controller(controller);

        window.present();
    });

    app.run_with_args::<&str>(&[]);
}
