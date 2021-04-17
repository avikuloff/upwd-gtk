extern crate gio;
extern crate gtk;

use std::env;
use std::rc::Rc;
use std::str::FromStr;

use gdk::{RGBA, SELECTION_CLIPBOARD};
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Box, BoxBuilder, Button, ButtonBoxBuilder, ButtonBoxStyle,
    CheckButton, Entry, EntryBuilder, HeaderBarBuilder, Label, Orientation, PolicyType, Popover,
    PopoverBuilder, Scale, ScrolledWindow, ScrolledWindowBuilder, SpinButton, StateFlags,
    TextBuffer, TextBufferBuilder, TextView, TextViewBuilder,
};
use upwd_lib::{calculate_entropy, generate_password, Pool};

// To import all needed traits.
use crate::config::{Config, ConfigBuilder};

mod config;

fn main() {
    let uiapp = gtk::Application::new(
        Some("com.github.avikuloff.upwd-gtk"),
        gio::ApplicationFlags::FLAGS_NONE,
    )
    .expect("Application::new failed");

    uiapp.connect_activate(|app| {
        let cfg: Rc<Config> = Rc::new(Config::load());
        // --------------- CREATE MAIN WINDOW --------------- //
        let win = create_application_window(app);

        // --------------- CREATE HEADER BAR --------------- //
        let header = HeaderBarBuilder::new()
            .title("Random Password Generator")
            .show_close_button(true)
            .has_subtitle(false)
            .build();
        let save_btn = Button::with_label("Save");
        save_btn.set_tooltip_text(Some("Save current configuration"));
        header.pack_start(&save_btn);

        // --------------- CREATE MAIN CONTAINER --------------- //
        let main_box = Box::new(Orientation::Vertical, 5);
        main_box.set_property_margin(15);

        // --------------- POOL CUSTOMIZATION --------------- //
        let pool_box = Box::new(Orientation::Vertical, 0);
        let uppers_chk_btn = CheckButton::with_label("Uppercase letters");
        let lowers_chk_btn = CheckButton::with_label("Lowercase letters");
        let digits_chk_btn = CheckButton::with_label("Digits");
        let symbols_chk_btn = CheckButton::with_label("Special chars");
        uppers_chk_btn.set_active(cfg.use_uppers());
        lowers_chk_btn.set_active(cfg.use_lowers());
        digits_chk_btn.set_active(cfg.use_digits());
        symbols_chk_btn.set_active(cfg.use_symbols());

        let pool_entry = Rc::new(EntryBuilder::new().text(cfg.pool()).build());
        pool_box.add(&uppers_chk_btn);
        pool_box.add(&lowers_chk_btn);
        pool_box.add(&digits_chk_btn);
        pool_box.add(&symbols_chk_btn);
        pool_box.add(&*pool_entry);

        // --------------- PASSWORD LENGTH --------------- //
        let length_box = Box::new(Orientation::Horizontal, 0);
        let (length_label, length_scale) = create_length_scale();
        length_scale.set_value(cfg.length() as f64);
        let length_scale = Rc::new(length_scale);
        length_box.pack_start(&length_label, false, false, 0);
        length_box.pack_end(&*length_scale, true, true, 5);

        // --------------- NUMBER OF PASSWORDS --------------- //
        let num_password_box = Box::new(Orientation::Vertical, 0);
        let (num_password_label, num_password_spin_btn) = create_num_passwords_spin_btn();
        num_password_spin_btn.set_value(cfg.count() as f64);
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
        btn_copy.set_tooltip_text(Some("Copy to clipboard"));
        let copy_popover = create_copy_popover(&btn_copy);
        btn_box.add(&*btn_generate);
        btn_box.add(&btn_copy);

        // --------------- POPULATE MAIN CONTAINER --------------- //
        main_box.add(&pool_box);
        main_box.add(&length_box);
        main_box.add(&num_password_box);
        main_box.pack_start(&passwords_box, true, true, 0);
        main_box.pack_end(&btn_box, false, false, 0);

        // --------------- POPULATE MAIN WINDOW --------------- //
        win.set_titlebar(Some(&header));
        win.add(&main_box);
        win.show_all();

        // --------------- HANDLE SIGNALS --------------- //
        {
            let pool_entry = pool_entry.clone();
            let cfg = cfg.clone();
            uppers_chk_btn.connect_toggled(move |chk_btn| {
                handle_chk_btn_toggled(chk_btn, &*pool_entry, cfg.uppers());
            });
        }
        {
            let pool_entry = pool_entry.clone();
            let cfg = cfg.clone();
            lowers_chk_btn.connect_toggled(move |chk_btn| {
                handle_chk_btn_toggled(chk_btn, &*pool_entry, cfg.lowers())
            });
        }
        {
            let pool_entry = pool_entry.clone();
            let cfg = cfg.clone();
            digits_chk_btn.connect_toggled(move |chk_btn| {
                handle_chk_btn_toggled(chk_btn, &*pool_entry, cfg.digits())
            });
        }
        {
            let pool_entry = pool_entry.clone();
            let cfg = cfg.clone();
            symbols_chk_btn.connect_toggled(move |chk_btn| {
                handle_chk_btn_toggled(chk_btn, &*pool_entry, cfg.symbols())
            });
        }

        {
            let btn_generate = btn_generate.clone();
            let length_scale = length_scale.clone();
            let strong_meter_box = strong_meter_box.clone();
            pool_entry.clone().connect_changed(move |entry| {
                handle_pool_entry_changed(entry, &*btn_generate);

                let pool = Pool::from_str(&entry.get_text()).unwrap();
                strong_meter_set_color(&*strong_meter_box, length_scale.get_value() as usize, pool);
            });
        }

        {
            let pool_entry = pool_entry.clone();
            let strong_meter_box = strong_meter_box.clone();
            length_scale.connect_value_changed(move |length| {
                let pool = Pool::from_str(&*pool_entry.get_text()).unwrap();
                strong_meter_set_color(&*strong_meter_box, length.get_value() as usize, pool);
            });
        }
        {
            let pool_entry = pool_entry.clone();
            let length_scale = length_scale.clone();
            let num_password_spin_btn = num_password_spin_btn.clone();
            btn_generate.connect_clicked(move |_btn| {
                handle_generate_btn_clicked(
                    &pool_entry.get_text(),
                    length_scale.get_value() as usize,
                    num_password_spin_btn.get_value() as u8,
                    &passwords_text_buffer,
                )
            });
        }

        btn_copy.connect_clicked(move |_btn| {
            handle_copy_btn_clicked(&passwords_text_view);
            copy_popover.show_all();
        });

        {
            let cfg = cfg.clone();
            save_btn.connect_clicked(move |_btn| {
                let cfg = ConfigBuilder::new()
                    .uppers(cfg.uppers().to_owned())
                    .lowers(cfg.lowers().to_owned())
                    .digits(cfg.digits().to_owned())
                    .symbols(cfg.symbols().to_owned())
                    .use_uppers(uppers_chk_btn.get_active())
                    .use_lowers(lowers_chk_btn.get_active())
                    .use_digits(digits_chk_btn.get_active())
                    .use_symbols(symbols_chk_btn.get_active())
                    .pool(pool_entry.get_text().to_owned())
                    .length(length_scale.get_value() as u8)
                    .count(num_password_spin_btn.get_value() as u32)
                    .build();

                cfg.save()
            });
        }
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

fn create_length_scale() -> (Label, Scale) {
    let length_scale = Scale::with_range(Orientation::Horizontal, 4.0, 64.0, 1.0);
    length_scale.set_value(12.0);

    (Label::new(Some("Length")), length_scale)
}

fn create_num_passwords_spin_btn() -> (Label, SpinButton) {
    let num_password_spin_btn = SpinButton::with_range(1.0, 1000.0, 1.0);
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

fn create_copy_popover(copy_btn: &Button) -> Popover {
    PopoverBuilder::new()
        .relative_to(copy_btn)
        .child(&Label::new(Some("Copied!")))
        .modal(true)
        .border_width(6)
        .build()
}

// В зависимости от состояния `chk_btn` добавляет или удаляет символы `string` из `entry`
fn handle_chk_btn_toggled(chk_btn: &CheckButton, entry: &Entry, string: &str) {
    let mut pool = Pool::from_str(&entry.get_text()).unwrap();

    if chk_btn.get_active() {
        pool.extend_from_string(string);
    } else {
        string.chars().for_each(|ch| {
            pool.shift_remove(&ch);
        });
    }

    entry.set_text(&pool.to_string());
}

// Если `entry` не содержит ни одного символа, то кнопка `btn_generate` блокируется
fn handle_pool_entry_changed(entry: &Entry, btn_generate: &Button) {
    let pool = Pool::from_str(&entry.get_text()).unwrap();

    if pool.len() == 0 {
        btn_generate.set_sensitive(false);
    } else {
        btn_generate.set_sensitive(true);
    }
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
    let clipboard = text_view.get_clipboard(&SELECTION_CLIPBOARD);
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
