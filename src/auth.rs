use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
    Client, Url,
};
use serde_json::{json, Value};
use std::{env, path::Path, fs};
use std::io::Read;
use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::Write,
};
use tiny_http::Server;



pub async fn authenticate() -> Result<(String, String), Box<dyn Error>> {

    let mut auth_code = String::from("");

    let client_id = env::var("MAL_CLI_CLIENT_ID").expect("Client ID not set.");
    let client_id = client_id.as_str();
    let code_challenge = "7tPPwQCPWku8SYxrDr1VyLBHXne7RVNmB8ndAwGvZYTCrD";

    let auth_url = Url::parse_with_params(
        "https://myanimelist.net/v1/oauth2/authorize",
        &[
            ("response_type", "code"),
            ("client_id", client_id),
            ("state", "STATE"),
            ("redirect_uri", "http://localhost:8080"),
            ("code_challenge", code_challenge),
            ("code_challenge_method", "plain"),
        ],
    )?;
    open::that(auth_url.as_str())?;

    // parse the auth token
    let server = Server::http("127.0.0.1:8080").unwrap();
    for rq in server.incoming_requests() {
        let complete_url = "http://localhost:8080".to_string() + rq.url();
        let request_url = Url::parse(complete_url.as_str())?;

        auth_code = request_url
            .query_pairs()
            .find(|(key, _value)| key == "code")
            .map(|(_, val)| val)
            .unwrap()
            .to_string();

        break;
    }

    // get access token
    let params = [
        ("client_id", client_id),
        ("client_secret", ""),
        ("code", auth_code.as_str()),
        ("code_verifier", code_challenge),
        ("redirect_uri", "http://localhost:8080"),
        ("grant_type", "authorization_code"),
    ];

    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    let response = client
        .post("https://myanimelist.net/v1/oauth2/token")
        .headers(headers)
        .form(&params)
        .send()
        .await?;

    let resp: Value = serde_json::from_str(response.text().await?.as_str()).unwrap();
    Ok((
        clean_token(resp.get("access_token").unwrap().to_string()),
        clean_token(resp.get("refresh_token").unwrap().to_string()),
    ))
}

pub async fn get_access_token() -> Result<String, Box<dyn Error>> {
    let home: String = env::var("HOME").unwrap();
    let token_location: String = format!("{home}/.config/mal-cli");

    let cache = read_token(token_location.clone());
    match cache {
        Ok(token) => Ok(clean_token(token.get("token").unwrap().to_string())),
        _ => {
            let (access_token, _) = authenticate().await.unwrap();
            save_token(&access_token, token_location);
            Ok(access_token)
        }
    }
}

pub async fn reauthenticate() -> Result<String, Box<dyn Error>> {
    let home: String = env::var("HOME").unwrap();
    let token_location: String = format!("{home}/.config/mal-cli");

    let (access_token, _) = authenticate().await.unwrap();
    save_token(&access_token, token_location);
    Ok(access_token)
}

fn clean_token(token: String) -> String {
    token
        .trim_start_matches("\"")
        .trim_end_matches("\"")
        .to_string()
}

fn save_token(token: &str, token_location: String) {
    let data = json!({
        "token": clean_token(token.to_string())
    });

    let data_string = data.to_string();
    let mut file = File::create(format!("{token_location}/token.json")).expect("Unable to create token cache.");

    // save
    file.write_all(data_string.as_bytes())
        .expect("Unable to write token cache.");
}

fn read_token(token_location: String) -> Result<Value, serde_json::Error> {

    let binding = token_location.clone();
    let path = Path::new(&binding);
    if !path.exists() {
        // Try to create the directory
        match fs::create_dir_all(path) {
            Ok(_) => (),
            Err(e) => println!("Error creating directory: {}", e),
        }
    }

    // Check if the directory exists
    if !path.exists() {
        // Try to create the directory
        match fs::create_dir_all(path) {
            Ok(_) => println!("Directory created successfully."),
            Err(e) => println!("Error creating directory: {}", e),
        }
    }
    let mut file = OpenOptions::new()
        .read(true)
        .create(true)
        .write(true)
        .open(format!("{token_location}/token.json"))
        .expect("Unable to create file..");

    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();

    let contents_str = String::from_utf8(contents).unwrap();
    serde_json::from_str(&contents_str)
}
