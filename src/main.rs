extern crate gio;
extern crate gtk;

// To import all needed traits.
use gdk;
use gio::prelude::*;
use gtk::prelude::*;

use gtk::{Box, Button, ButtonBox, ButtonBoxStyle, CheckButton, Label, Orientation, PolicyType, Scale, ScrolledWindow, TextView, NONE_ADJUSTMENT};
use std::env;
use upwd_lib::{generate_password, Pool};

fn main() {
    // Todo change app id
    let uiapp = gtk::Application::new(
        Some("com.github.avikuloff.upwd-gtk"),
        gio::ApplicationFlags::FLAGS_NONE,
    )
    .expect("Application::new failed");

    uiapp.connect_activate(|app| {
        let win = gtk::ApplicationWindow::new(app);
        win.set_default_size(240, 320);
        win.set_title("Random Password Generator");

        let main_box = Box::new(Orientation::Vertical, 0);
        main_box.set_property_margin(15);

        let ch_box = Box::new(Orientation::Vertical, 0);
        let chbox_uppers = CheckButton::with_label("Uppercase");
        let chbox_lowers = CheckButton::with_label("Lowercase");
        let chbox_digits = CheckButton::with_label("Digits");
        let chbox_symbols = CheckButton::with_label("Special chars");
        chbox_uppers.set_active(true);
        chbox_lowers.set_active(true);
        chbox_digits.set_active(true);
        ch_box.pack_start(&chbox_uppers, false, false, 0);
        ch_box.pack_start(&chbox_lowers, false, false, 0);
        ch_box.pack_start(&chbox_digits, false, false, 0);
        ch_box.pack_start(&chbox_symbols, false, false, 0);
        main_box.add(&ch_box);

        let length_box = Box::new(Orientation::Horizontal, 0);
        let length_label = Label::new(Some("Length"));
        let scale_length = Scale::with_range(Orientation::Horizontal, 4.0, 64.0, 1.0);
        scale_length.set_value(12.0);
        length_box.pack_start(&length_label, false, false, 0);
        length_box.pack_end(&scale_length, true, true, 5);
        main_box.add(&length_box);

        // Todo add number of passwords field

        let bbox = ButtonBox::new(Orientation::Horizontal);
        bbox.set_layout(ButtonBoxStyle::Expand);
        let btn_generate = Button::with_label("Generate");
        let btn_copy = Button::with_label("Copy");
        bbox.add(&btn_generate);
        bbox.add(&btn_copy);
        let text = TextView::new();
        let buf = text.get_buffer().unwrap();
        buf.set_text("");

        // Todo needs resizing window
        let sw = ScrolledWindow::new(NONE_ADJUSTMENT, NONE_ADJUSTMENT);
        sw.set_policy(PolicyType::Automatic, PolicyType::Automatic);
        sw.add(&text);

        let text_box = Box::new(Orientation::Vertical, 5);
        text_box.pack_start(&sw, false, false, 0);
        text_box.pack_start(&bbox, false, false, 0);
        main_box.add(&text_box);

        win.add(&main_box);
        win.show_all();
/*
        chbox_uppers.connect_toggled(move |v: &CheckButton| {
            let mut pool = Pool::from_str(&entry_buffer.get_text()).unwrap();

            if v.get_active() {
                pool.extend_from_string("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
            } else {
                pool.remove_all("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
            }

            &entry_buffer.set_text(&pool.to_string());
        });
*/
        btn_generate.connect_clicked(move |v| {
            let mut pool = Pool::new();

            if chbox_uppers.get_active() {
                pool.extend_from_string("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
            }
            if chbox_lowers.get_active() {
                pool.extend_from_string("abcdefghijklmnopqrstuvwxyz");
            }
            if chbox_digits.get_active() {
                pool.extend_from_string("0123456789");
            }
            if chbox_symbols.get_active() {
                pool.extend_from_string("*&^%$#@!~");
            }

            if pool.is_empty() {
                v.set_sensitive(false);
                return;
            } else {
                v.set_sensitive(true);
            }

            buf.set_text(&*generate_password(
                &pool,
                scale_length.get_value() as usize,
            ));
        });
        btn_copy.connect_clicked(move |_| {
            let clipboard = &text.get_clipboard(&gdk::SELECTION_CLIPBOARD);
            let buffer = &text.get_buffer().unwrap();
            let text = buffer
                .get_text(&buffer.get_start_iter(), &buffer.get_end_iter(), false)
                .unwrap();
            clipboard.set_text(&text);
        });
    });
    uiapp.run(&env::args().collect::<Vec<_>>());
}
