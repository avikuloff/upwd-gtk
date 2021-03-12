extern crate gio;
extern crate gtk;

// To import all needed traits.
use gdk;
use gio::prelude::*;
use gtk::prelude::*;

use gtk::{Box, Button, ButtonBox, ButtonBoxStyle, CheckButton, Label, Orientation, PolicyType, Scale, ScrolledWindow, TextView, NONE_ADJUSTMENT, Entry, SpinButton};
use std::env;
use upwd_lib::{generate_password, Pool};
use std::str::FromStr;
use std::rc::Rc;

const UPPERCASE: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE: &'static str = "abcdefghijklmnopqrstuvwxyz";
const DIGITS: &'static str = "0123456789";
const SYMBOLS: &'static str = "*&^%$#@!~";

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
        let chbox_uppers = CheckButton::with_label("Uppercase letters");
        let chbox_lowers = CheckButton::with_label("Lowercase letters");
        let chbox_digits = CheckButton::with_label("Digits");
        let chbox_symbols = CheckButton::with_label("Special chars");
        let entry_pool = Rc::new(Entry::new());
        chbox_uppers.set_active(true);
        chbox_lowers.set_active(true);
        chbox_digits.set_active(true);
        entry_pool.set_text(&format!("{}{}{}", UPPERCASE, LOWERCASE, DIGITS));
        ch_box.add(&chbox_uppers);
        ch_box.add(&chbox_lowers);
        ch_box.add(&chbox_digits);
        ch_box.add(&chbox_symbols);
        ch_box.add(&*entry_pool);
        main_box.add(&ch_box);

        let length_box = Box::new(Orientation::Horizontal, 0);
        let length_label = Label::new(Some("Length"));
        let scale_length = Scale::with_range(Orientation::Horizontal, 4.0, 64.0, 1.0);
        scale_length.set_value(12.0);
        length_box.pack_start(&length_label, false, false, 0);
        length_box.pack_end(&scale_length, true, true, 5);
        main_box.add(&length_box);

        let num_password_box = Box::new(Orientation::Vertical, 0);
        num_password_box.set_margin_top(10);
        num_password_box.set_margin_bottom(10);
        let num_password_label = Label::new(Some("Number of passwords"));
        let num_password_spin_btn = SpinButton::with_range(1_f64, 100_f64, 1_f64);
        num_password_spin_btn.set_value(1.0);
        num_password_box.add(&num_password_label);
        num_password_box.add(&num_password_spin_btn);
        main_box.add(&num_password_box);

        let bbox = ButtonBox::new(Orientation::Horizontal);
        bbox.set_layout(ButtonBoxStyle::Expand);
        let btn_generate = Rc::new(Button::with_label("Generate"));
        let btn_copy = Button::with_label("Copy");
        bbox.add(&*btn_generate);
        bbox.add(&btn_copy);
        let text = TextView::new();
        let buf = text.get_buffer().unwrap();
        buf.set_text("");

        let sw = ScrolledWindow::new(NONE_ADJUSTMENT, NONE_ADJUSTMENT);
        sw.set_min_content_height(80);
        sw.set_policy(PolicyType::Automatic, PolicyType::Automatic);
        sw.add(&text);

        let text_box = Box::new(Orientation::Vertical, 5);
        text_box.pack_start(&sw, false, false, 0);
        text_box.pack_start(&bbox, false, false, 0);
        main_box.add(&text_box);

        win.add(&main_box);
        win.show_all();

        let clone_entry_pool = entry_pool.clone();
        chbox_uppers.connect_toggled(move |v| {
            let mut pool = Pool::from_str(&clone_entry_pool.get_text()).unwrap();

            if v.get_active() {
                pool.extend_from_string(UPPERCASE);
            } else {
                pool.remove_all(UPPERCASE);
            }

            &clone_entry_pool.set_text(&pool.to_string());
        });
        let clone_entry_pool = entry_pool.clone();
        chbox_lowers.connect_toggled(move |v| {
            let mut pool = Pool::from_str(&clone_entry_pool.get_text()).unwrap();

            if v.get_active() {
                pool.extend_from_string(LOWERCASE);
            } else {
                pool.remove_all(LOWERCASE);
            }

            &clone_entry_pool.set_text(&pool.to_string());
        });
        let clone_entry_pool = entry_pool.clone();
        chbox_digits.connect_toggled(move |v| {
            let mut pool = Pool::from_str(&clone_entry_pool.get_text()).unwrap();

            if v.get_active() {
                pool.extend_from_string(DIGITS);
            } else {
                pool.remove_all(DIGITS);
            }

            &clone_entry_pool.set_text(&pool.to_string());
        });
        let clone_entry_pool = entry_pool.clone();
        chbox_symbols.connect_toggled(move |v| {
            let mut pool = Pool::from_str(&clone_entry_pool.get_text()).unwrap();

            if v.get_active() {
                pool.extend_from_string(SYMBOLS);
            } else {
                pool.remove_all(SYMBOLS);
            }

            &clone_entry_pool.set_text(&pool.to_string());
        });

        let clone_entry_pool = entry_pool.clone();
        let clone_btn_generate = btn_generate.clone();
        clone_entry_pool.connect_changed(move |v| {
            if v.get_text_length() == 0 {
                clone_btn_generate.set_sensitive(false);
                return;
            } else {
                clone_btn_generate.set_sensitive(true);
            }
        });

        btn_generate.connect_clicked(move |_| {
            let len = scale_length.get_value() as usize;
            let num_passwords = num_password_spin_btn.get_value() as u8;
            let pool = Pool::from_str(&entry_pool.get_text()).unwrap();

            for i in 0..num_passwords {
                let mut password = generate_password(&pool, len);
                if i == 0 {
                    buf.set_text(&password);
                } else {
                    password.insert(0, '\n');
                    buf.insert(&mut buf.get_iter_at_line(i as i32), &password)
                }
            }
        });

        // Todo add tooltip
        btn_copy.connect_clicked(move |_btn| {
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
