use cursive::views::*;
use cursive::Cursive;

pub fn cprint(s: &mut Cursive, data: String)
{
    s.add_layer(
        Dialog::around(
            TextView::new(data)
        )
            .button("Close",|s| {
                s.pop_layer();
            })
    );
}