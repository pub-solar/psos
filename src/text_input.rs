use cursive::traits::*;
use cursive::views::{Dialog, EditView};
use cursive::Cursive;

pub const TITLE: &str = "Setup hostname";

pub fn create<F>(
    siv: &mut Cursive,
    title: &str,
    content: &str,
    on_back_button: F,
    on_submit: F,
    view_name: String,
) where
    F: FnMut(&mut Cursive, &str) -> (),
{
    if view_name.is_empty() {
        view_name = "text_input_view".to_string();
    }

    Dialog::new()
        .title(title)
        // Padding is (left, right, top, bottom)
        .padding_lrtb(1, 1, 1, 0)
        .content(
            EditView::new()
                .content(content)
                .on_submit(|s, _| {
                    check_valid(s, view_name.to_string(), on_submit);
                })
                .with_name(view_name)
                .fixed_width(30),
        )
        .button("Back", |s| {
            s.pop_layer();
            crate::show_main_menu(s);
        })
        .button("Ok", |s| {
            // This will run the given closure, *ONLY* if a view with the
            // correct type and the given name is found.

            check_valid(s, view_name.to_string(), on_submit);
        });
}

fn check_valid<F>(s: &mut Cursive, view_name: String, cb: F)
where
    F: FnMut(&mut Cursive, &str) -> (),
{
    let output = s
        .call_on_name(view_name.as_str(), |view: &mut EditView| view.get_content())
        .unwrap();

    if output.is_empty() {
        return;
    }

    cb(s, &output);
}
