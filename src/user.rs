use cursive::traits::*;
use cursive::views::{Dialog, EditView};
use cursive::Cursive;

pub const TITLE: &str = "Setup User";

pub fn setup(siv: &mut Cursive) {
    username(siv);
}

fn username(siv: &mut Cursive) {
    // Create a dialog with an edit text and a button.
    // The user can either hit the <Ok> button,
    // or press Enter on the edit text.
    let username = siv
        .user_data::<crate::InstallState>()
        .unwrap()
        .user
        .username
        .clone();

    siv.add_layer(
        Dialog::new()
            .title("Enter username")
            // Padding is (left, right, top, bottom)
            .padding_lrtb(1, 1, 1, 0)
            .content(
                EditView::new()
                    .content(username)
                    .with_name("username")
                    .fixed_width(30),
            )
            .button("Back", |s| {
                s.pop_layer();
                crate::show_main_menu(s);
            })
            .button("Ok", |s| {
                // This will run the given closure, *ONLY* if a view with the
                // correct type and the given name is found.
                let username = s
                    .call_on_name("username", |view: &mut EditView| view.get_content())
                    .unwrap();

                if username.is_empty() {
                    return;
                }

                s.with_user_data(|data: &mut crate::InstallState| {
                    data.user.username = username.to_string();
                });
                s.pop_layer();
                full_name(s);
            }),
    );
}

fn full_name(siv: &mut Cursive) {
    // Create a dialog with an edit text and a button.
    // The user can either hit the <Ok> button,
    // or press Enter on the edit text.
    let full_name = siv
        .user_data::<crate::InstallState>()
        .unwrap()
        .user
        .full_name
        .clone();

    siv.add_layer(
        Dialog::new()
            .title("Enter full_name")
            // Padding is (left, right, top, bottom)
            .padding_lrtb(1, 1, 1, 0)
            .content(
                EditView::new()
                    .content(full_name)
                    .with_name("full_name")
                    .fixed_width(30),
            )
            .button("Back", |s| {
                s.pop_layer();
                username(s);
            })
            .button("Ok", |s| {
                // This will run the given closure, *ONLY* if a view with the
                // correct type and the given name is found.
                let full_name = s
                    .call_on_name("full_name", |view: &mut EditView| view.get_content())
                    .unwrap();

                if full_name.is_empty() {
                    return;
                }

                s.with_user_data(|data: &mut crate::InstallState| {
                    data.user.full_name = full_name.to_string();
                });
                s.pop_layer();
                email(s);
            }),
    );
}

fn email(siv: &mut Cursive) {
    // Create a dialog with an edit text and a button.
    // The user can either hit the <Ok> button,
    // or press Enter on the edit text.
    let email = siv
        .user_data::<crate::InstallState>()
        .unwrap()
        .user
        .email
        .clone();

    siv.add_layer(
        Dialog::new()
            .title("Enter email")
            // Padding is (left, right, top, bottom)
            .padding_lrtb(1, 1, 1, 0)
            .content(
                EditView::new()
                    .content(email)
                    .with_name("email")
                    .fixed_width(30),
            )
            .button("Back", |s| {
                s.pop_layer();
                full_name(s);
            })
            .button("Ok", |s| {
                // This will run the given closure, *ONLY* if a view with the
                // correct type and the given name is found.
                let email = s
                    .call_on_name("email", |view: &mut EditView| view.get_content())
                    .unwrap();

                if email.is_empty() {
                    return;
                }

                s.with_user_data(|data: &mut crate::InstallState| {
                    data.user.email = email.to_string();
                });
                s.pop_layer();
                password(s);
            }),
    );
}

fn password(siv: &mut Cursive) {
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
            .title("Enter password")
            // Padding is (left, right, top, bottom)
            .padding_lrtb(1, 1, 1, 0)
            .content(
                EditView::new()
                    .content(password)
                    .secret()
                    .with_name("password")
                    .fixed_width(30),
            )
            .button("Back", |s| {
                s.pop_layer();
                email(s);
            })
            .button("Show", |s| {
                s.call_on_name("password", |view: &mut EditView| view.set_secret(false))
                    .unwrap();
            })
            .button("Ok", |s| {
                // This will run the given closure, *ONLY* if a view with the
                // correct type and the given name is found.
                let password = s
                    .call_on_name("password", |view: &mut EditView| view.get_content())
                    .unwrap();

                if password.is_empty() {
                    return;
                }

                s.with_user_data(|data: &mut crate::InstallState| {
                    data.user.password = password.to_string();
                });
                s.pop_layer();
                crate::show_main_menu(s);
            }),
    );
}
