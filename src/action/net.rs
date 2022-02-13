use reqwest::blocking::{ClientBuilder, Response, Client,multipart};
// use dotenv::dotenv;

use serde::{Deserialize,Serialize};
#[derive(Serialize, Deserialize, Debug)]
struct UserForm{
    user: String,
    password: String
}

#[derive(Serialize,Deserialize, Debug)]
struct File{
    name: String,
    description: String,
    upload_date: String
}

#[derive(Serialize,Deserialize)]
struct NewFile{
    name: String,
    description: String
}

#[derive(Serialize)]
struct DownloadRequest {
    file_id : i32,
    session_id : String
}

pub fn api_register(name: &str, password: &str) -> Result<Response,reqwest::Error>{
    let ip = dotenv::var("IP").unwrap();
    let port = dotenv::var("PORT").unwrap();
    let data = UserForm {
        user: String::from(name),
        password: String::from(password)
    };
    let client = ClientBuilder::new().build().unwrap();
    client.post(format!("{}/api/register",ipv_format(ip,port))).form(&data).send()
        // .expect("Error sending register request")

}

pub fn api_login(name: &str, password: &str) -> Result<Response,reqwest::Error>{
    let ip = dotenv::var("IP").unwrap();
    let port = dotenv::var("PORT").unwrap();
    let data = UserForm {
        user: String::from(name),
        password: String::from(password)
    };
    let client = ClientBuilder::new().build().unwrap();
    client.post(format!("{}/api/login",ipv_format(ip,port))).form(&data).send()

}

pub fn api_get(session_id: String) -> Response{
    let ip = dotenv::var("IP").unwrap();
    let port = dotenv::var("PORT").unwrap();
    let client = ClientBuilder::new().build().unwrap();
    client.post(format!("{}/api/get",ipv_format(ip,port)))
    .body(session_id)
    .send()
        .expect("Error sending get request")
}

pub fn api_download(id: i32, session_string: String) -> Response{
    let ip = dotenv::var("IP").unwrap();
    let port = dotenv::var("PORT").unwrap();

    let form = DownloadRequest {
        file_id: id,
        session_id: session_string
    };

    let client = ClientBuilder::new().build().unwrap();
    client.post(format!("{}/api/download",ipv_format(ip,port)))
    .form(&form)
    .send()
    .expect("Error downloading a file")
}

pub fn api_upload(id_string: String, name: String, desc: String, file_path: &std::path::Path)
{
    let ip = dotenv::var("IP").unwrap();
    let port = dotenv::var("PORT").unwrap();
    let client = Client::new();
    let form = multipart::Form::new()
        .file("file",file_path).unwrap()
        .text("name",name)
        .text("description",desc)
        .text("session",id_string);

    let resp = client.post(format!("{}/api/upload",ipv_format(ip,port)))
        .multipart(form)
        .send()
        .unwrap();

    
}

fn ipv_format(ip: String,port: String) -> String
{
    let ip_type = dotenv::var("TYPE").unwrap();
    match ip_type.as_str()
    {
        "4" => {return format!("http://{}:{}",ip,port)}
        "6" => {return format!("http://[{}]:{}",ip,port)}
        _ => {return "Type not found".to_string()}
    }
}