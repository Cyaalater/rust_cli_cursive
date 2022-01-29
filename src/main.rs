use cursive::views::{Dialog, TextView};
mod views;
fn main() {
    let mut siv = cursive::default();
    siv.add_layer(
        views::splash::build()
    );
    siv.run();
}
