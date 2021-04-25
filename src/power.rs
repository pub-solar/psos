use cursive::traits::*;
use cursive::views::{Dialog, TextView};
use cursive::Cursive;
use std::process::Command;

pub const TITLE: &str = "Reboot";

pub fn reboot(siv: &mut Cursive) {
    // Create a dialog with an edit text and a button.
    // The user can either hit the <Ok> button,
    // or press Enter on the edit text.
    let password = siv
        .user_data::<crate::InstallState>()
        .unwrap()
        .user
        .password
        .clone();

    siv.add_layer(
        Dialog::new()
            .title("Reboot")
            // Padding is (left, right, top, bottom)
            .padding_lrtb(1, 1, 1, 0)
            .content(TextView::new("Are you sure you want to reboot?").fixed_width(30))
            .button("Cancel", |s| {
                s.pop_layer();
                crate::show_main_menu(s);
            })
            .button("Reboot now", |s| {
                Command::new("reboot")
                    .arg("now")
                    .status()
                    .expect("process failed to execute");
            }),
    );
}
