use cursive::align::HAlign;
use cursive::event::EventResult;
use cursive::traits::*;
use cursive::view::Margins;
use cursive::views::{Dialog, OnEventView, SelectView};
use cursive::Cursive;
use std::env;

pub mod host;
pub mod power;
pub mod text_input;
pub mod user;

pub type Callback = fn();

pub struct User {
    pub username: String,
    pub password: String,
    pub full_name: String,
    pub email: String,
    pub gpg_key_id: String,
}

pub struct InstallState {
    pub timezone: String,
    pub keyboard_layout: String,
    pub hostname: String,
    pub user: User,
}

fn main() {
    let state = crate::InstallState {
        timezone: "".to_string(),
        keyboard_layout: "".to_string(),
        hostname: "".to_string(),
        user: crate::User {
            username: "".to_string(),
            full_name: "".to_string(),
            password: "".to_string(),
            email: "".to_string(),
            gpg_key_id: "".to_string(),
        },
    };

    let mut siv = cursive::default();
    siv.set_user_data(state);
    siv.add_global_callback('q', |s| s.quit());
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "--skip-guided" {
        show_main_menu(&mut siv);
    } else {
        run_guided_install(&mut siv);
    }
    siv.run();
}

fn run_guided_install(siv: &mut Cursive) {}

pub fn show_main_menu(siv: &mut Cursive) {
    let mut select = SelectView::new()
        // Center the text horizontally
        .h_align(HAlign::Left)
        // Use keyboard to jump to the pressed letters
        .autojump();

    // Read the list of cities from separate file, and fill the view with it.
    // (We include the file at compile-time to avoid runtime read errors.)
    // TODO: add --skip-forced in case you already have the device in the repo,
    // And only need to including it in the configuration
    // TODO: device configuration file needs to be written at the latest before nixos-install
    // TODO: Check if /etc/nixos is setup and valid
    // TODO: Check if /nix/store is setup and valid
    // TODO: All optional steps are only optional if you set --skip-forced?
    let content = [
        crate::user::TITLE,
        crate::host::TITLE,
        "",
        crate::power::TITLE,
    ]
    .join("\n");
    select.add_all_str(content.lines());

    // Sets the callback for when "Enter" is pressed.
    select.set_on_submit(|siv: &mut Cursive, title: &str| execute_selection(siv, title));

    // Let's override the `j` and `k` keys for navigation
    let select = OnEventView::new(select)
        .on_pre_event_inner('i', |s, _| {
            let cb = s.select_up(1);
            Some(EventResult::Consumed(Some(cb)))
        })
        .on_pre_event_inner('k', |s, _| {
            let cb = s.select_down(1);
            Some(EventResult::Consumed(Some(cb)))
        });
    // Let's add a ResizedView to keep the list at a reasonable size
    // (it can scroll anyway).
    siv.add_layer(
        Dialog::around(select.scrollable().fixed_size((35, 14)))
            .title("PubSolar OS")
            .padding(Margins {
                bottom: 0,
                top: 1,
                left: 2,
                right: 2,
            })
            .button("Quit", |s| s.quit()),
    );
}

// Let's put the callback in a separate function to keep it clean,
// but it's not required.
fn execute_selection(siv: &mut Cursive, title: &str) {
    siv.pop_layer();

    match title {
        crate::user::TITLE => {
            crate::user::setup(siv);
        }
        crate::power::TITLE => {
            crate::power::reboot(siv);
        }
        crate::host::TITLE => {
            crate::host::setup(siv);
        }
        _ => {}
    }
}
