use std::{fs, path::Path};
use jzon::{JsonValue, parse};

pub struct Options {
    pub four_k_mode: bool,
    pub borderless: bool,
    pub vsync: bool,
    pub fullscreen: bool,
}

const OPTIONS_JSON_PATH: &str = "options.json";

pub fn get_options() -> Options {

    if !Path::new(OPTIONS_JSON_PATH).exists() {
        let mut default_options = JsonValue::new_object();
        default_options["four_k_mode"] = false.into();
        default_options["borderless"] = false.into();
        default_options["vsync"] = false.into();
        default_options["fullscreen"] = false.into();

        let res = fs::write(OPTIONS_JSON_PATH, default_options.dump());
        if let Err(e) = res { println!("{}", e) }

        return Options { four_k_mode: false, borderless: false, vsync: true, fullscreen: false }
    }

    let contents = match fs::read_to_string(OPTIONS_JSON_PATH) {
        Ok(s) => s,
        _ => String::from(""),
    };

    let parsed = match parse(contents.as_str()) {
        Ok(j) => j,
        _ => JsonValue::new_object(),
    };

    // attempt to parser the values, but assume false if none are found
    let four_k_mode = match parsed["four_k_mode"].as_bool() {
        Some(b) => b,
        _ => false,
    };
    let borderless = match parsed["borderless"].as_bool() {
        Some(b) => b,
        _ => false,
    };
    let vsync = match parsed["vsync"].as_bool() {
        Some(b) => b,
        _ => false,
    };
    let fullscreen = match parsed["fullscreen"].as_bool() {
        Some(b) => b,
        _ => false,
    };

    Options {
        four_k_mode,
        borderless,
        vsync,
        fullscreen,
    }
}

pub fn set_option(key: String, value: bool) {

    let mut current_options = get_options();

    match key.as_str() {
        "4K Mode" => {
            current_options.four_k_mode = value;
        },
        "Borderless" => {
            current_options.borderless = value;
        },
        "V-sync" => {
            current_options.vsync = value;
        },
        "Fullscreen" => {
            current_options.fullscreen = value;
        },
        _ => {
        }
    }

    let mut options_as_json = JsonValue::new_object();
    options_as_json["four_k_mode"] = current_options.four_k_mode.into();
    options_as_json["borderless"] = current_options.borderless.into();
    options_as_json["vsync"] = current_options.vsync.into();
    options_as_json["fullscreen"] = current_options.fullscreen.into();

    let res = fs::write(OPTIONS_JSON_PATH, options_as_json.dump());
    if let Err(e) = res { println!("{}", e) }
}

pub fn toggle_option(key: String) {

    let current_options = get_options();

    let value = match key.as_str() {
        "4K Mode" => {
            current_options.four_k_mode
        },
        "Borderless" => {
            current_options.borderless
        },
        "V-sync" => {
            current_options.vsync
        },
        "Fullscreen" => {
            current_options.fullscreen
        },
        _ => {
            return;
        }
    };

    set_option(key, !value);
}