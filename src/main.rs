#[macro_use]
extern crate lazy_static;

use actix_web::{
    http::StatusCode,
    web,
    web::{HttpResponse, Json},
    App, HttpServer, Result,
};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize)]
struct EnMorseCode {
    encoded: String,
}

#[derive(Serialize)]
struct DeMorseCode {
    decoded: String,
}

lazy_static! {
    static ref MORSE_CODE: HashMap<&'static str, &'static str> = vec![
        (".-", "A"),
        ("-...", "B"),
        ("-.-.", "C"),
        ("-..", "D"),
        (".", "E"),
        ("..-.", "F"),
        ("--.", "G"),
        ("....", "H"),
        ("..", "I"),
        (".---", "J"),
        ("-.-", "K"),
        (".-..", "L"),
        ("--", "M"),
        ("-.", "N"),
        ("---", "O"),
        (".--.", "P"),
        ("--.-", "Q"),
        (".-.", "R"),
        ("...", "S"),
        ("-", "T"),
        ("..-", "U"),
        ("...-", "V"),
        (".--", "W"),
        ("-..-", "X"),
        ("-.--", "Y"),
        ("--..", "Z"),
        (".----", "1"),
        ("..---", "2"),
        ("...--", "3"),
        ("....-", "4"),
        (".....", "5"),
        ("-....", "6"),
        ("--...", "7"),
        ("---..", "8"),
        ("----.", "9"),
        ("-----", "0"),
        ("...---...", "SOS"),
        (".-.-.-", "."),
        ("--..--", ","),
        ("..--..", "?"),
        (".----.", "'"),
        ("-.-.--", "!"),
        ("-..-.", "/"),
        ("-.--.", "("),
        ("-.--.-", ")"),
        (".-...", "&"),
        ("---...", ":"),
        ("-.-.-.", ";"),
        ("-...-", "="),
        (".-.-.", "+"),
        ("-....-", "-"),
        ("..--.-", "_"),
        (".-..-.", "\""),
        ("...-..-", "$"),
        (".--.-.", "@"),
        ("...-.-", "<End of work>"),
        ("........", "<Error>"),
        ("-.-.-", "<Starting Signal>"),
        ("...-.", "<Understood>"),
    ]
    .into_iter()
    .collect();
}

fn decode_morse(emc: EnMorseCode) -> DeMorseCode {
    DeMorseCode {
        decoded: emc
            .encoded
            .trim()
            .split("   ")
            .map(|word| {
                word.split(' ')
                    .filter_map(|code| MORSE_CODE.get(code))
                    .cloned()
                    .collect()
            })
            .collect::<Vec<String>>()
            .join(" "),
    }
}

fn post_morse(data: Json<EnMorseCode>) -> Result<Json<DeMorseCode>> {
    Ok(Json(decode_morse(data.into_inner())))
}

fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK).finish())
}

fn main() -> std::io::Result<()> {
    let limit = String::from(
        std::env::var_os("MORSE_CODE_APP_DATA_LIMIT")
            .expect("'MORSE_CODE_APP_DATA_LIMIT' environment variable not set!")
            .to_str()
            .expect("'MORSE_CODE_APP_DATA_LIMIT' is not a valid value!"),
    )
    .parse()
    .expect("'MORSE_CODE_APP_DATA_LIMIT' is not a valid unsigned integer!");

    HttpServer::new(move || {
        App::new()
            .data(web::JsonConfig::default().limit(limit))
            .service(web::resource("/").route(web::post().to(post_morse)))
            .service(web::resource("/health-check").route(web::get().to(health_check)))
    })
    .bind("0.0.0.0:80")?
    .run()
}
