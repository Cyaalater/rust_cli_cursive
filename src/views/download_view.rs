// use cursive::traits::*;
use cursive::Cursive;
use cursive::{traits::Resizable, views::*};
use crate::views::select_file_view::select_file;
// use cursive::align::HAlign;

pub struct StandardFile{
    pub name: String,
    pub description: String,
    pub date: String
}
impl StandardFile{
    fn new(name : String, desc : String, date : String) -> StandardFile
    {
        StandardFile {
            name,
            description: desc,
            date
        }
    }
}


pub fn download(s: &mut Cursive)
{
    let files_array = vec![
        StandardFile::new("Dayan.exe".to_string(), "Going to die lol".to_string(),"2021-1-1".to_string()),
        StandardFile::new("Amara.db".to_string(), "Nice lmao".to_string(), "2021-1-2".to_string())
    ];
    let mut files = SelectView::new();
    for (i,file) in files_array.iter().enumerate() {
        files.add_item(table(file.name.clone(),file.date.clone()),i);
    }

    // files.add_item("Ofekitah.sh",3);

    files.set_on_submit(move |s,i|{

        select_file(s,&files_array[i.to_owned()]);
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