use gdk::{Display, RGBA};
use gtk::CssProvider;
use libadwaita::{self as adw, ApplicationWindow, prelude::AdwApplicationWindowExt};
use webkit6::{WebView, prelude::*};

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_bytes(&"* { background-color: #000; }".as_bytes().into());

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn create_webview(_window: &ApplicationWindow, uri: &str) -> WebView {
    let webview = WebView::builder().build();

    let settings = WebViewExt::settings(&webview).unwrap();
    settings.set_enable_developer_extras(true);
    settings.set_enable_page_cache(false);
    settings.set_disable_web_security(true);
    settings.set_enable_html5_local_storage(true);
    webview.set_background_color(&RGBA::new(0.0, 0.0, 0.0, 1.0));

    webview.load_uri(uri);

    webview
}

fn main() {
    let uri = std::env::args()
        .skip(1)
        .next()
        .unwrap_or("https://meexreay.github.io/poshlostios".to_string());

    let app = adw::Application::builder()
        .application_id("ru.themixray.poshlostios-webkitgtk6")
        .build();

    app.connect_activate(move |app| {
        let window = adw::ApplicationWindow::new(app);

        load_css();

        window.set_default_size(500, 500);
        window.set_fullscreened(true);

        let webview = create_webview(&window, &uri);
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
