use cursive::traits::*;
use cursive::Cursive;
use cursive::{traits::Resizable, views::*};
use crate::views::splash;


pub fn setup() -> Dialog {
    Dialog::around(
        LinearLayout::vertical()
            .child(TextView::new("Enter server's location"))
            .child(
                LinearLayout::horizontal()
                    .child(EditView::new().with_name("IP").fixed_width(15))
                    .child(TextView::new("IP")),
            )
            .child(
                LinearLayout::horizontal()
                    .child(EditView::new().with_name("PORT").fixed_width(10))
                    .child(TextView::new("PORT")),
            )
            .child(Button::new("Apply", |s| {
                let ip = s
                    .call_on_name("IP", |view: &mut EditView| view.get_content())
                    .unwrap();
                let port = s
                    .call_on_name("PORT", |view: &mut EditView| view.get_content())
                    .unwrap();
                ok(s, &ip, &port);
            }))
    )
}

fn ok(s: &mut Cursive, ip: &str, port: &str) {
    std::fs::write("./.env", format!("IP={}\nPORT={}", ip, port));
    s.pop_layer();
    s.add_layer(splash::build());
}
