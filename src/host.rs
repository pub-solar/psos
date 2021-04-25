use cursive::traits::*;
use cursive::views::{Dialog, EditView};
use cursive::Cursive;

pub const TITLE: &str = "Setup hostname";

pub fn setup(siv: &mut Cursive) {
    hostname(siv);
}

fn hostname(siv: &mut Cursive) {
    // Create a dialog with an edit text and a button.
    // The user can either hit the <Ok> button,
    // or press Enter on the edit text.
    let hostname = siv
        .user_data::<crate::InstallState>()
        .unwrap()
        .hostname
        .clone();

    siv.add_layer(
        Dialog::new()
            .title("Enter hostname")
            // Padding is (left, right, top, bottom)
            .padding_lrtb(1, 1, 1, 0)
            .content(
                EditView::new()
                    .content(hostname)
                    .with_name("hostname")
                    .fixed_width(30),
            )
            .button("Back", |s| {
                s.pop_layer();
                crate::show_main_menu(s);
            })
            .button("Ok", |s| {
                // This will run the given closure, *ONLY* if a view with the
                // correct type and the given name is found.
                let hostname = s
                    .call_on_name("hostname", |view: &mut EditView| view.get_content())
                    .unwrap();

                if hostname.is_empty() {
                    return;
                }

                s.with_user_data(|data: &mut crate::InstallState| {
                    data.hostname = hostname.to_string();
                });
                s.pop_layer();
                crate::show_main_menu(s);
            }),
    );
}
