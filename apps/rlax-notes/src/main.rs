use gtk::glib;
use gtk::prelude::*;
use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Default)]
struct Document {
    path: Option<PathBuf>,
    dirty: bool,
}

fn main() -> glib::ExitCode {
    let app = gtk::Application::builder()
        .application_id("id.rlax.notes")
        .build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &gtk::Application) {
    install_theme();
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Rlax Notes — Untitled")
        .default_width(760)
        .default_height(540)
        .build();

    let document = Rc::new(RefCell::new(Document::default()));
    let root = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let toolbar = gtk::Box::new(gtk::Orientation::Horizontal, 6);
    toolbar.add_css_class("toolbar");
    toolbar.set_margin_top(8);
    toolbar.set_margin_bottom(8);
    toolbar.set_margin_start(8);
    toolbar.set_margin_end(8);

    let new_button = gtk::Button::with_label("New");
    let open_button = gtk::Button::with_label("Open");
    let save_button = gtk::Button::with_label("Save");
    let save_as_button = gtk::Button::with_label("Save as");
    save_button.add_css_class("suggested-action");
    for button in [&new_button, &open_button, &save_button, &save_as_button] {
        button.add_css_class("tool-button");
        toolbar.append(button);
    }
    let brand = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    brand.set_margin_start(16);
    brand.set_margin_end(16);
    brand.set_margin_top(12);
    brand.set_margin_bottom(12);
    brand.add_css_class("brand-header");
    let icon = gtk::Image::from_icon_name("rlax-notes");
    icon.set_pixel_size(54);
    brand.append(&icon);
    let brand_text = gtk::Box::new(gtk::Orientation::Vertical, 3);
    let brand_title = gtk::Label::new(Some("RLAX NOTES"));
    brand_title.add_css_class("brand-title");
    brand_title.set_xalign(0.0);
    let brand_subtitle = gtk::Label::new(Some("MIGHTY ACTION X  •  TEXT EDITOR"));
    brand_subtitle.add_css_class("brand-subtitle");
    brand_subtitle.set_xalign(0.0);
    brand_text.append(&brand_title);
    brand_text.append(&brand_subtitle);
    brand.append(&brand_text);
    root.append(&brand);
    root.append(&toolbar);

    let text_view = gtk::TextView::new();
    text_view.set_wrap_mode(gtk::WrapMode::WordChar);
    text_view.set_monospace(true);
    text_view.add_css_class("note-editor");
    let buffer = text_view.buffer();
    let scroller = gtk::ScrolledWindow::builder()
        .hexpand(true)
        .vexpand(true)
        .child(&text_view)
        .build();
    root.append(&scroller);
    window.set_child(Some(&root));

    let update_title: Rc<dyn Fn()> = Rc::new({
        let window = window.clone();
        let document = document.clone();
        move || {
            let doc = document.borrow();
            let name = doc
                .path
                .as_ref()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .unwrap_or("Untitled");
            let marker = if doc.dirty { " •" } else { "" };
            window.set_title(Some(&format!("Rlax Notes — {name}{marker}")));
        }
    });

    buffer.connect_changed({
        let document = document.clone();
        let update_title = update_title.clone();
        move |_| {
            document.borrow_mut().dirty = true;
            update_title();
        }
    });

    new_button.connect_clicked({
        let buffer = buffer.clone();
        let document = document.clone();
        let update_title = update_title.clone();
        move |_| {
            buffer.set_text("");
            *document.borrow_mut() = Document::default();
            update_title();
        }
    });

    open_button.connect_clicked({
        let window = window.clone();
        let buffer = buffer.clone();
        let document = document.clone();
        let update_title = update_title.clone();
        move |_| {
            let dialog = gtk::FileChooserNative::new(
                Some("Open text file"),
                Some(&window),
                gtk::FileChooserAction::Open,
                Some("Open"),
                Some("Cancel"),
            );
            add_text_filters(&dialog);
            dialog.connect_response({
                let window = window.clone();
                let buffer = buffer.clone();
                let document = document.clone();
                let update_title = update_title.clone();
                move |dialog, response| {
                    if response == gtk::ResponseType::Accept {
                        if let Some(path) = dialog.file().and_then(|f| f.path()) {
                            match fs::read_to_string(&path) {
                                Ok(text) => {
                                    buffer.set_text(&text);
                                    *document.borrow_mut() = Document {
                                        path: Some(path),
                                        dirty: false,
                                    };
                                    update_title();
                                }
                                Err(err) => {
                                    show_error(&window, &format!("Could not open file: {err}"))
                                }
                            }
                        }
                    }
                    dialog.destroy();
                }
            });
            dialog.show();
        }
    });

    save_button.connect_clicked({
        let window = window.clone();
        let buffer = buffer.clone();
        let document = document.clone();
        let update_title = update_title.clone();
        move |_| save_current(&window, &buffer, &document, &update_title, false)
    });
    save_as_button.connect_clicked({
        let window = window.clone();
        let buffer = buffer.clone();
        let document = document.clone();
        let update_title = update_title.clone();
        move |_| save_current(&window, &buffer, &document, &update_title, true)
    });

    let key_controller = gtk::EventControllerKey::new();
    key_controller.connect_key_pressed({
        let window = window.clone();
        let buffer = buffer.clone();
        let document = document.clone();
        let update_title = update_title.clone();
        move |_, key, _, modifiers| {
            let ctrl = modifiers.contains(gtk::gdk::ModifierType::CONTROL_MASK);
            if ctrl && key == gtk::gdk::Key::s {
                save_current(&window, &buffer, &document, &update_title, false);
                glib::Propagation::Stop
            } else {
                glib::Propagation::Proceed
            }
        }
    });
    window.add_controller(key_controller);
    window.present();
}

fn install_theme() {
    let provider = gtk::CssProvider::new();
    provider.load_from_data(
        "
        window { background: #080012; color: #e8e8ef; }
        .brand-header { background: linear-gradient(90deg, #1A002A, #080012); border-bottom: 1px solid rgba(255,20,147,0.45); }
        .brand-title { color: #FF1493; font-size: 18px; font-weight: 900; letter-spacing: 2px; }
        .brand-subtitle { color: #8a8ab0; font-size: 9px; letter-spacing: 1.5px; }
        .toolbar { background: #0d0718; border-bottom: 1px solid rgba(0,255,255,0.18); }
        button.tool-button { color: #e8e8ef; background: #1A002A; border: 1px solid rgba(255,20,147,0.38); border-radius: 7px; padding: 7px 14px; }
        button.tool-button:hover { background: #2a0044; border-color: #00FFFF; box-shadow: 0 0 10px rgba(0,255,255,0.22); }
        button.tool-button.suggested-action { color: #080012; background: #FF1493; border-color: #FF1493; font-weight: 800; }
        button.tool-button.suggested-action:hover { background: #ff55ad; }
        textview.note-editor, textview.note-editor text { color: #e8e8ef; background: #080012; caret-color: #00FFFF; selection-background-color: rgba(255,20,147,0.55); }
        textview.note-editor { padding: 20px; font-family: 'JetBrains Mono', monospace; font-size: 13px; }
        scrollbar slider { background: rgba(255,20,147,0.55); border-radius: 5px; }
        filechooser { background: #080012; color: #e8e8ef; }
        ",
    );
    if let Some(display) = gtk::gdk::Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

fn add_text_filters(dialog: &gtk::FileChooserNative) {
    let filter = gtk::FileFilter::new();
    filter.set_name(Some("Text and Markdown"));
    filter.add_mime_type("text/plain");
    filter.add_pattern("*.txt");
    filter.add_pattern("*.md");
    dialog.add_filter(&filter);
    let all = gtk::FileFilter::new();
    all.set_name(Some("All files"));
    all.add_pattern("*");
    dialog.add_filter(&all);
}

fn save_current(
    window: &gtk::ApplicationWindow,
    buffer: &gtk::TextBuffer,
    document: &Rc<RefCell<Document>>,
    update_title: &Rc<dyn Fn()>,
    force_dialog: bool,
) {
    if !force_dialog {
        if let Some(path) = document.borrow().path.clone() {
            write_document(window, buffer, document, update_title, path);
            return;
        }
    }
    let dialog = gtk::FileChooserNative::new(
        Some("Save note"),
        Some(window),
        gtk::FileChooserAction::Save,
        Some("Save"),
        Some("Cancel"),
    );
    dialog.set_current_name("untitled.txt");
    add_text_filters(&dialog);
    dialog.connect_response({
        let window = window.clone();
        let buffer = buffer.clone();
        let document = document.clone();
        let update_title = update_title.clone();
        move |dialog, response| {
            if response == gtk::ResponseType::Accept {
                if let Some(path) = dialog.file().and_then(|f| f.path()) {
                    write_document(&window, &buffer, &document, &update_title, path);
                }
            }
            dialog.destroy();
        }
    });
    dialog.show();
}

fn write_document(
    window: &gtk::ApplicationWindow,
    buffer: &gtk::TextBuffer,
    document: &Rc<RefCell<Document>>,
    update_title: &Rc<dyn Fn()>,
    path: PathBuf,
) {
    let text = buffer.text(&buffer.start_iter(), &buffer.end_iter(), true);
    match fs::write(&path, text.as_str()) {
        Ok(()) => {
            *document.borrow_mut() = Document {
                path: Some(path),
                dirty: false,
            };
            update_title();
        }
        Err(err) => show_error(window, &format!("Could not save file: {err}")),
    }
}

fn show_error(parent: &gtk::ApplicationWindow, message: &str) {
    let dialog = gtk::MessageDialog::builder()
        .transient_for(parent)
        .modal(true)
        .message_type(gtk::MessageType::Error)
        .buttons(gtk::ButtonsType::Close)
        .text(message)
        .build();
    dialog.connect_response(|dialog, _| dialog.close());
    dialog.present();
}
