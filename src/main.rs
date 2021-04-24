use cursive::align::HAlign;
use cursive::event::EventResult;
use cursive::traits::*;
use cursive::view::Margins;
use cursive::views::{Dialog, OnEventView, SelectView, TextView};
use cursive::Cursive;
use std::env;

mod psos {
    pub mod types;
    pub mod user;
}

// We'll use a SelectView here.
//
// A SelectView is a scrollable list of items, from which the user can select
// one.

fn main() {
    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());
    siv.add_layer(
        Dialog::new()
            .title("PubSolarOS")
            .padding_lrtb(1, 1, 1, 1)
            .content(TextView::new("loading")),
    );
    siv.run();

    let args: Vec<String> = env::args().collect();
    if args[0] == "--skip-guided" {
        show_main_menu(&mut siv);
    } else {
        run_guided_install(&mut siv);
    }
}

fn run_guided_install(siv: &mut Cursive) {}

fn show_main_menu(siv: &mut Cursive) {
    let mut select = SelectView::new()
        // Center the text horizontally
        .h_align(HAlign::Center)
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
    let content = [crate::psos::user::TITLE, ""].join("\n");
    select.add_all_str(content.lines());

    // Sets the callback for when "Enter" is pressed.
    select.set_on_submit(|siv: &mut Cursive, title: &str| execute_selection(&mut siv, &mut title));

    // Let's override the `j` and `k` keys for navigation
    let select = OnEventView::new(select)
        .on_pre_event_inner('k', |s, _| {
            let cb = s.select_up(1);
            Some(EventResult::Consumed(Some(cb)))
        })
        .on_pre_event_inner('j', |s, _| {
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
    if title == "" {
        return;
    }

    siv.pop_layer();

    match title {
        crate::psos::user::TITLE => {
            crate::psos::user::setup(&mut siv);
        }
    }
}
