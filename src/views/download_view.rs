use cursive::traits::*;
use cursive::Cursive;
use cursive::{traits::Resizable, views::*};
// use cursive::align::HAlign;

pub fn download(s: &mut Cursive)
{
    let mut files = SelectView::new();
    files.add_item(table("Dayan.exe".to_string(),"Going to die lol".to_string()),1);
    files.add_item(table("Amara.db".to_string(),"Nice lmao".to_string()),2);
    // files.add_item("Ofekitah.sh",3);

    files.set_on_submit(|s, file|{
        s.pop_layer()
    });

    s.add_layer(
        Dialog::around(
            files
        )
        .button("Close", |s|{
            s.pop_layer();
        })
        .fixed_width(40)
    )
}

fn table(name: String, desc: String) -> String
{
    return format!("{} ! {}",fillstr(name,12),desc)
}

fn fillstr(data: String,len: usize) -> String
{
    let mut result : String = data;
    while result.chars().count() <= len
    {
        result.push_str(" ");
    }
    return result;
}