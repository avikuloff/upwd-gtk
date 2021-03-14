extern crate gio;
extern crate gtk;

// To import all needed traits.
use gio::prelude::*;
use gtk::prelude::*;

use gdk;
use gdk::RGBA;
use gtk::{
    Application, ApplicationWindow, Box, BoxBuilder, Button, ButtonBoxBuilder, ButtonBoxStyle,
    CheckButton, CheckButtonBuilder, Entry, EntryBuilder, Label, Orientation, PolicyType, Scale,
    ScrolledWindow, ScrolledWindowBuilder, SpinButton, StateFlags, TextBuffer, TextBufferBuilder,
    TextView, TextViewBuilder,
};
use std::env;
use std::ops::Add;
use std::rc::Rc;
use std::str::FromStr;
use upwd_lib::{calculate_entropy, generate_password, Pool};

const UPPERS: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERS: &'static str = "abcdefghijklmnopqrstuvwxyz";
const DIGITS: &'static str = "0123456789";
const SYMBOLS: &'static str = "*&^%$#@!~";

fn main() {
    let uiapp = gtk::Application::new(
        Some("com.github.avikuloff.upwd-gtk"),
        gio::ApplicationFlags::FLAGS_NONE,
    )
    .expect("Application::new failed");

    uiapp.connect_activate(|app| {
        // --------------- CREATE MAIN WINDOW --------------- //
        let win = create_application_window(app);

        // --------------- CREATE MAIN CONTAINER --------------- //
        let main_box = Box::new(Orientation::Vertical, 5);
        main_box.set_property_margin(15);

        // --------------- POOL CUSTOMIZATION --------------- //
        let pool_box = Box::new(Orientation::Vertical, 0);
        let uppers_chk_btn = create_uppers_chk_btn();
        let lowers_chk_btn = create_lowers_chk_btn();
        let digits_chk_btn = create_digits_chk_btn();
        let symbols_chk_btn = CheckButton::with_label("Special chars");
        let pool_entry = Rc::new(create_pool_entry());
        pool_box.add(&uppers_chk_btn);
        pool_box.add(&lowers_chk_btn);
        pool_box.add(&digits_chk_btn);
        pool_box.add(&symbols_chk_btn);
        pool_box.add(&*pool_entry);

        // --------------- PASSWORD LENGTH --------------- //
        let length_box = Box::new(Orientation::Horizontal, 0);
        let (length_label, length_scale) = create_length_scale();
        let length_scale = Rc::new(length_scale);
        length_box.pack_start(&length_label, false, false, 0);
        length_box.pack_end(&*length_scale, true, true, 5);

        // --------------- NUMBER OF PASSWORDS --------------- //
        let num_password_box = Box::new(Orientation::Vertical, 0);
        let (num_password_label, num_password_spin_btn) = create_num_passwords_spin_btn();
        num_password_box.add(&num_password_label);
        num_password_box.add(&num_password_spin_btn);

        // --------------- CREATE SHOW PASSWORDS WINDOW --------------- //
        let scrolled_window = create_scrolled_window();
        let passwords_box = Box::new(Orientation::Vertical, 0);
        let strong_meter_box = Rc::new(create_strong_meter_box(&length_scale, &pool_entry));
        let (passwords_text_view, passwords_text_buffer) = create_passwords_text_view();
        scrolled_window.add(&passwords_text_view);
        passwords_box.pack_start(&scrolled_window, true, true, 0);
        passwords_box.add(&*strong_meter_box);

        // --------------- GENERATE AND COPY PASSWORDS BUTTONS --------------- //
        let btn_box = ButtonBoxBuilder::new()
            .orientation(Orientation::Horizontal)
            .layout_style(ButtonBoxStyle::Expand)
            .build();
        let btn_generate = Rc::new(Button::with_label("Generate"));
        let btn_copy = Button::with_label("Copy");
        btn_box.add(&*btn_generate);
        btn_box.add(&btn_copy);

        // --------------- POPULATE MAIN CONTAINER --------------- //
        main_box.add(&pool_box);
        main_box.add(&length_box);
        main_box.add(&num_password_box);
        main_box.pack_start(&passwords_box, true, true, 0);
        main_box.pack_end(&btn_box, false, false, 0);

        // --------------- POPULATE MAIN WINDOW --------------- //
        win.add(&main_box);
        win.show_all();

        // --------------- HANDLE SIGNALS --------------- //
        let clone_entry_pool = pool_entry.clone();
        uppers_chk_btn.connect_toggled(move |chk_btn| {
            handle_chk_btn_toggled(chk_btn, &*clone_entry_pool, UPPERS)
        });
        let clone_entry_pool = pool_entry.clone();
        lowers_chk_btn.connect_toggled(move |chk_btn| {
            handle_chk_btn_toggled(chk_btn, &*clone_entry_pool, LOWERS)
        });
        let clone_entry_pool = pool_entry.clone();
        digits_chk_btn.connect_toggled(move |chk_btn| {
            handle_chk_btn_toggled(chk_btn, &*clone_entry_pool, DIGITS)
        });
        let clone_entry_pool = pool_entry.clone();
        symbols_chk_btn.connect_toggled(move |chk_btn| {
            handle_chk_btn_toggled(chk_btn, &*clone_entry_pool, SYMBOLS)
        });

        let clone_btn_generate = btn_generate.clone();
        let clone_length_scale = length_scale.clone();
        let clone_strong_meter = strong_meter_box.clone();
        pool_entry.clone().connect_changed(move |entry| {
            handle_pool_entry_changed(
                entry,
                &*clone_btn_generate,
                &*clone_length_scale,
                &*clone_strong_meter,
            )
        });

        let clone_btn_generate = btn_generate.clone();
        let clone_pool_entry = pool_entry.clone();
        length_scale.connect_value_changed(move |length| {
            handle_pool_entry_changed(
                &*clone_pool_entry,
                &*clone_btn_generate,
                &length,
                &*strong_meter_box,
            )
        });

        btn_generate.connect_clicked(move |_btn| {
            handle_generate_btn_clicked(
                &pool_entry.get_text(),
                length_scale.get_value() as usize,
                num_password_spin_btn.get_value() as u8,
                &passwords_text_buffer,
            )
        });

        btn_copy.connect_clicked(move |_btn| handle_copy_btn_clicked(&passwords_text_view));
        // --------------- END HANDLE SIGNALS --------------- //
    });
    uiapp.run(&env::args().collect::<Vec<_>>());
}

fn create_application_window(app: &Application) -> ApplicationWindow {
    gtk::ApplicationWindowBuilder::new()
        .application(app)
        .default_width(255)
        .default_height(360)
        .title("Random Password Generator")
        .build()
}

fn create_uppers_chk_btn() -> CheckButton {
    CheckButtonBuilder::new()
        .label("Uppercase letters")
        .active(true)
        .build()
}

fn create_lowers_chk_btn() -> CheckButton {
    CheckButtonBuilder::new()
        .label("Lowercase letters")
        .active(true)
        .build()
}

fn create_digits_chk_btn() -> CheckButton {
    CheckButtonBuilder::new()
        .label("Digits")
        .active(true)
        .build()
}

fn create_pool_entry() -> Entry {
    EntryBuilder::new()
        .text(&String::from(UPPERS).add(LOWERS).add(DIGITS))
        .build()
}

fn create_length_scale() -> (Label, Scale) {
    let length_scale = Scale::with_range(Orientation::Horizontal, 4.0, 64.0, 1.0);
    length_scale.set_value(12.0);

    (Label::new(Some("Length")), length_scale)
}

fn create_num_passwords_spin_btn() -> (Label, SpinButton) {
    let num_password_spin_btn = SpinButton::with_range(1.0, 100.0, 1.0);
    num_password_spin_btn.set_value(1.0);

    (
        Label::new(Some("Number of passwords")),
        num_password_spin_btn,
    )
}

fn create_passwords_text_view() -> (TextView, TextBuffer) {
    let buffer = TextBufferBuilder::new().text("").build();
    let view = TextViewBuilder::new()
        .buffer(&buffer)
        .border_width(3)
        .build();

    (view, buffer)
}

fn create_scrolled_window() -> ScrolledWindow {
    ScrolledWindowBuilder::new()
        .min_content_height(65)
        .vscrollbar_policy(PolicyType::Always)
        .build()
}

fn create_strong_meter_box(length_scale: &Scale, pool_entry: &Entry) -> Box {
    let strong_meter_box = BoxBuilder::new().height_request(2).build();

    strong_meter_set_color(
        &strong_meter_box,
        length_scale.get_value() as usize,
        Pool::from_str(&pool_entry.get_text()).unwrap(),
    );

    strong_meter_box
}

// В зависимости от состояния `chk_btn` добавляет или удаляет символы `string` из `entry`
fn handle_chk_btn_toggled(chk_btn: &CheckButton, entry: &Entry, string: &str) {
    let mut pool = Pool::from_str(&entry.get_text()).unwrap();

    if chk_btn.get_active() {
        pool.extend_from_string(string);
    } else {
        pool.remove_all(string);
    }

    entry.set_text(&pool.to_string());
}

// Если `entry` не содержит ни одного символа, то кнопка `btn_generate` блокируется
fn handle_pool_entry_changed(
    entry: &Entry,
    btn_generate: &Button,
    length: &Scale,
    strong_meter: &Box,
) {
    let pool = Pool::from_str(&entry.get_text()).unwrap();

    if pool.len() == 0 {
        btn_generate.set_sensitive(false);
    } else {
        btn_generate.set_sensitive(true);
    }

    strong_meter_set_color(strong_meter, length.get_value() as usize, pool);
}

// Создает `num_passwords` паролей длиной `length` символов, используя символы определенные в `pool`.
// Перезаписывает `buffer`
fn handle_generate_btn_clicked(pool: &str, length: usize, num_passwords: u8, buffer: &TextBuffer) {
    let pool = Pool::from_str(pool).unwrap();

    for i in 0..num_passwords {
        let password = generate_password(&pool, length);
        if i == 0 {
            buffer.set_text(&password);
        } else {
            let iter = &mut buffer.get_iter_at_line(i as i32);
            buffer.insert(iter, "\n");
            buffer.insert(iter, &password);
        }
    }
}

// Копирует пароли из `text_view` в буфер обмена
fn handle_copy_btn_clicked(text_view: &TextView) {
    let clipboard = text_view.get_clipboard(&gdk::SELECTION_CLIPBOARD);
    let buffer = text_view.get_buffer().unwrap();
    let text = buffer
        .get_text(&buffer.get_start_iter(), &buffer.get_end_iter(), false)
        .unwrap();

    clipboard.set_text(&text);
}

fn strong_meter_set_color(strong_meter: &Box, length: usize, pool: Pool) {
    let pool_size = pool.len();

    let color = if pool_size == 0 {
        RGBA::white()
    } else {
        match calculate_entropy(length, pool_size) as u16 {
            1..=52 => RGBA::red(),
            53..=70 => RGBA::from_str("#FFFF00").unwrap(),
            val if val > 70 => RGBA::green(),
            _ => RGBA::white(),
        }
    };

    strong_meter.override_background_color(StateFlags::NORMAL, Some(&color));
}
