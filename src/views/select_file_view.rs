use cursive::traits::*;
use cursive::{views::*, traits::Resizable};
use cursive::Cursive;
use cursive::theme::{ColorStyle,Color,BaseColor};
use crate::views::download_view::StandardFile;
use crate::action::net::api_download;
use crate::views::print::cprint;

pub fn select_file(s: &mut Cursive, file_info: &StandardFile, session_string: String)
{
    let name = &file_info.name;
    let desc = &file_info.description;
    let date = &file_info.date;
    let id = file_info.id.clone();
    let name_view = TextView::new(name);
    let desc_view = TextView::new(desc);
    let date_view = TextView::new(format!("Upload date: {}",date))
        .style(ColorStyle::new(Color::Dark(BaseColor::Blue),ColorStyle::secondary().back));
    s.add_layer(
        Dialog::around(
            LinearLayout::vertical()
            .child(TextView::new("NAME:"))
            .child(name_view)
            .child(DummyView)
            .child(TextView::new("DESCRIPTION:").scrollable())
            .child(desc_view)
            .child(DummyView.fixed_height(10))
            .child(date_view)
        )
        .button("Close", |s| {s.pop_layer();})
        .button("Download",move |s| {
            ok(s,id,session_string.clone());
            s.pop_layer();
        })
        .title("File information")
        .fixed_width(40)
    )
}

fn ok(s: &mut Cursive,id: i32, session_string: String )
{
    let result = api_download(id,session_string.clone());
    std::fs::write("./output", &result.bytes().unwrap())
        .expect("Failed copying data to a file");
    s.pop_layer();
    cprint(s,"Downloaded the file as output".to_string());
}