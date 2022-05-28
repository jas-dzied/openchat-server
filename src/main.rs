#[macro_use] extern crate rocket;
use std::collections::HashMap;
use rocket::serde::json::Json;

#[post("/add_server/<name>/<url>")]
fn add_server(name: &str, url: &str) -> &'static str {

    let server_text = std::fs::read_to_string("./servers.json").unwrap();
    let mut server_data = serde_json::from_str::<HashMap<String, String>>(&server_text).unwrap();
    server_data.insert(
        String::from_utf8(base64::decode_config(name, base64::URL_SAFE).unwrap()).unwrap(),
        String::from_utf8(base64::decode_config(url, base64::URL_SAFE).unwrap()).unwrap()
    );
    std::fs::write(
        "./servers.json",
        serde_json::to_string_pretty(&server_data).unwrap()
    ).unwrap();
    "Server added"

}

#[delete("/remove_server/<name>")]
fn remove_server(name: &str) -> &'static str {

    let server_text = std::fs::read_to_string("./servers.json").unwrap();
    let mut server_data = serde_json::from_str::<HashMap<String, String>>(&server_text).unwrap();
    server_data.remove(name).unwrap();
    std::fs::write(
        "./servers.json",
        serde_json::to_string_pretty(&server_data).unwrap()
    ).unwrap();
    "Server removed"

}

#[get("/get_servers")]
fn get_servers() -> Json<HashMap<String, String>> {

    let server_text = std::fs::read_to_string("./servers.json").unwrap();
    let server_data = serde_json::from_str::<HashMap<String, String>>(&server_text).unwrap();
    Json(server_data)

}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![add_server, remove_server, get_servers])
}
