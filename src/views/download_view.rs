use std::cell::RefCell;
use std::rc::Rc;
// use cursive::traits::*;
use cursive::Cursive;
use cursive::{traits::Resizable, views::*};
use crate::views::select_file_view::select_file;
use crate::action::net::api_get;
use serde::{Deserialize,Serialize};
// use cursive::align::HAlign;

#[derive(Deserialize,Serialize)]
pub struct StandardResult{
    success: bool,
    data: Vec::<StandardFile>
}
#[derive(Deserialize,Serialize)]
pub struct StandardFile{
    pub id: i32,
    pub name: String,
    pub description: String,
    pub path: String,
    pub uploader: String,
    pub date: String
}






pub fn download(s: &mut Cursive,session_id: Rc<RefCell<String>>)
{
    let session_string = session_id.take();
    session_id.replace(session_string.to_owned());
    let result = api_get(session_string.clone());
    let result_struct = result.json::<StandardResult>().unwrap();
    if !result_struct.success {
        return;
    }
    let files_array = result_struct.data;
    let mut files = SelectView::new();
    for (i,file) in files_array.iter().enumerate() {
        files.add_item(table(file.name.clone(),file.date.clone()),i);
    }

    // files.add_item("Ofekitah.sh",3);

    files.set_on_submit(move |s,i|{

        select_file(s,&files_array[i.to_owned()],session_string.clone());
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