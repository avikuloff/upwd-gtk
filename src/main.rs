extern crate gio;
extern crate gtk;

use std::env;
use std::rc::Rc;
use std::str::FromStr;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::{
    Adjustment, Align, ApplicationWindow, Box, Builder, Button, CheckButton, Clipboard, Entry,
    FlowBox, InfoBar, Label, LevelBar, MessageType, Scale, SpinButton, TextBuffer,
};
use upwd_lib::{calculate_entropy, generate_password, Pool};

use crate::config::{Config, ConfigBuilder};

mod config;

fn main() {
    let uiapp = gtk::Application::new(
        Some("com.github.avikuloff.upwd-gtk"),
        gio::ApplicationFlags::FLAGS_NONE,
    )
    .expect("Application::new failed");

    uiapp.connect_activate(|app| {
        let builder = Builder::from_file("/home/andrew/IdeaProjects/upwd-gtk/main.ui");
        let win: ApplicationWindow = builder.get_object("window").unwrap();

        let btn_save: Rc<Button> = Rc::new(builder.get_object("save").unwrap());

        let info_box: Rc<Box> = Rc::new(builder.get_object("info-box").unwrap());

        let cfg = {
            let info_box = info_box.clone();
            let cfg = Config::load().unwrap_or_else(move |e| {
                let info_bar = create_info_bar(&format!("Загружена конфигурация по умолчанию.\n{}", e), MessageType::Warning);
                info_box.add(&info_bar);
                info_bar.show_all();
                timeout_add_seconds(10, move || unsafe {
                    info_bar.destroy();
                    Continue(false)
                });
                Config::default()
            });

            Rc::new(cfg)
        };


        let pool_options_box: FlowBox = builder.get_object("pool-options").unwrap();
        let pool_entry: Rc<Entry> = Rc::new(builder.get_object("pool").unwrap());
        pool_entry.set_text(cfg.pool());
        let pool_options = cfg.pool_options().clone();
        for item in pool_options {
            let chk_btn = CheckButton::with_label(item.name());
            chk_btn.set_active(item.checked());
            chk_btn.show();
            pool_options_box.add(&chk_btn);

            chk_btn.connect_toggled(pool_option_toggled(
                &chk_btn,
                pool_entry.clone(),
                Rc::new(item.value().to_owned()),
            ));
        }

        let length: Rc<Scale> = Rc::new(builder.get_object("length").unwrap());
        length.set_adjustment(&Adjustment::new(
            cfg.length() as f64,
            4.0,
            cfg.max_length() as f64,
            1.0,
            0.0,
            0.0,
        ));

        let count: Rc<SpinButton> = Rc::new(builder.get_object("count").unwrap());
        count.set_adjustment(&Adjustment::new(
            cfg.count() as f64,
            1.0,
            cfg.max_count() as f64,
            1.0,
            0.0,
            0.0,
        ));

        let text_buffer: Rc<TextBuffer> = Rc::new(builder.get_object("text-buffer").unwrap());

        let strong_meter: Rc<LevelBar> = Rc::new(builder.get_object("strong-meter").unwrap());
        {
            let pool = Pool::from_str(&pool_entry.get_text()).unwrap();
            let entropy = calculate_entropy(length.get_value() as usize, pool.len());
            strong_meter.set_value(entropy / 20.0);
        }

        let btn_generate: Rc<Button> = Rc::new(builder.get_object("generate").unwrap());
        let btn_copy: Rc<Button> = Rc::new(builder.get_object("copy").unwrap());

        app.add_window(&win);

        pool_entry.connect_changed(pool_changed(
            &pool_entry,
            length.clone(),
            strong_meter.clone(),
            btn_generate.clone(),
        ));
        length.connect_value_changed(length_changed(
            &length,
            pool_entry.clone(),
            strong_meter.clone(),
        ));
        btn_generate.connect_clicked(btn_generate_clicked(
            &btn_generate,
            pool_entry.clone(),
            length.clone(),
            count.clone(),
            text_buffer.clone(),
        ));
        btn_copy.connect_clicked(btn_copy_clicked(
            &btn_copy,
            info_box.clone(),
            text_buffer.clone(),
        ));
        btn_save.connect_clicked(btn_save_clicked(
            &btn_save,
            cfg.clone(),
            info_box.clone(),
            pool_entry.clone(),
            length.clone(),
            count.clone(),
        ));
    });

    uiapp.run(&env::args().collect::<Vec<_>>());
}

fn pool_option_toggled(
    _chk_btn: &CheckButton,
    entry: Rc<Entry>,
    char_set: Rc<String>,
) -> impl Fn(&CheckButton) {
    move |chk_btn| {
        let mut pool = Pool::from_str(&entry.get_text()).unwrap();

        if chk_btn.get_active() {
            pool.extend_from_string(&*char_set);
        } else {
            char_set.chars().for_each(|ch| {
                pool.shift_remove(&ch);
            });
        }

        entry.set_text(&pool.to_string());
    }
}

fn pool_changed(
    _entry: &Entry,
    length_scale: Rc<Scale>,
    strong_meter: Rc<LevelBar>,
    btn_generate: Rc<Button>,
) -> impl Fn(&Entry) {
    move |entry| {
        let pool = Pool::from_str(&entry.get_text()).unwrap();
        let pool_len = pool.len();
        let entropy = calculate_entropy(length_scale.get_value() as usize, pool_len);

        strong_meter.set_value(entropy / 20.0);

        if pool_len == 0 {
            btn_generate.set_sensitive(false);
        } else if !btn_generate.get_sensitive() {
            btn_generate.set_sensitive(true);
        }
    }
}

fn length_changed(_lenght: &Scale, pool: Rc<Entry>, strong_meter: Rc<LevelBar>) -> impl Fn(&Scale) {
    move |length| {
        let pool = Pool::from_str(&pool.get_text()).unwrap();
        let pool_len = pool.len();
        let entropy = calculate_entropy(length.get_value() as usize, pool_len);

        strong_meter.set_value(entropy / 20.0);
    }
}

fn btn_generate_clicked(
    _btn: &Button,
    pool: Rc<Entry>,
    length: Rc<Scale>,
    count: Rc<SpinButton>,
    buffer: Rc<TextBuffer>,
) -> impl Fn(&Button) {
    move |_btn| {
        buffer.set_text("");
        let pool = Pool::from_str(&pool.get_text()).unwrap();
        let iter = &mut buffer.get_end_iter();

        for _ in 0..(count.get_value() as i32) {
            let password = generate_password(&pool, length.get_value() as usize);
            buffer.insert(iter, &(password + "\n"));
        }

        buffer.backspace(iter, false, true);
    }
}

fn btn_copy_clicked(_btn: &Button, info_box: Rc<Box>, buffer: Rc<TextBuffer>) -> impl Fn(&Button) {
    move |_btn| {
        let info_bar = match copy_passwords_to_clipboard(buffer.clone()) {
            Ok(_) => create_info_bar("Скопировано в буфер обмена", MessageType::Info),
            Err(e) => create_info_bar(
                &format!(
                    "Не удалось скопировать содержимое текстового поля в буфер обмена. Ошибка: {}",
                    e
                ),
                MessageType::Error,
            ),
        };
        info_box.add(&info_bar);
        info_bar.show_all();
        timeout_add_seconds(5, move || unsafe {
            info_bar.destroy();
            Continue(false)
        });
    }
}

fn btn_save_clicked(
    _btn: &Button,
    cfg: Rc<Config>,
    info_box: Rc<Box>,
    pool: Rc<Entry>,
    length: Rc<Scale>,
    count: Rc<SpinButton>,
) -> impl Fn(&Button) {
    move |_btn| {
        let cfg = ConfigBuilder::new()
            .pool_options(cfg.pool_options().to_owned())
            .pool(pool.get_text().to_owned())
            .length(length.get_value() as u8)
            .count(count.get_value() as u32)
            .max_length(cfg.max_length())
            .max_count(cfg.max_count())
            .build();

        let info_bar = match cfg.save() {
            Ok(_) => create_info_bar("Saved.", MessageType::Info),
            Err(e) => create_info_bar(&e.to_string(), MessageType::Error),
        };
        info_box.add(&info_bar);
        info_bar.show_all();
        timeout_add_seconds(5, move || unsafe {
            info_bar.destroy();
            Continue(false)
        });
    }
}

fn create_info_bar(message: &str, message_type: MessageType) -> InfoBar {
    let info_bar = InfoBar::new();
    let label = Label::new(Some(message));
    label.set_selectable(true);
    label.set_line_wrap(true);
    info_bar.set_message_type(message_type);
    info_bar.set_valign(Align::Start);
    info_bar.set_show_close_button(true);
    info_bar.get_content_area().add(&label);
    info_bar
}

// Копирует содержимое текстового буфера в буфер обмена.
fn copy_passwords_to_clipboard(buffer: Rc<TextBuffer>) -> Result<(), String> {
    let clipboard = &Clipboard::get(&gdk::ATOM_NONE);
    let (start, end) = buffer.get_bounds();
    let text = buffer.get_text(&start, &end, false);

    match text {
        Some(val) => clipboard.set_text(&val),
        None => return Err("Не удалось получить текст из буфера.".to_owned()),
    };

    buffer.copy_clipboard(clipboard);
    Ok(())
}
