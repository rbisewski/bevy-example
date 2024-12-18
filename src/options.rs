use std::{fs, path::Path};

pub struct Options {
    pub four_k_mode: bool,
    pub borderless: bool,
    pub vsync: bool,
    pub fullscreen: bool,
}

const OPTIONS_JSON_PATH: &str = "options.json";

pub fn get_options() -> Options {

    if !Path::new(OPTIONS_JSON_PATH).exists() {
        let default_options = r#"
        {
            "four_k_mode": false,
            "borderless": false,
            "vsync": true,
            "fullscreen": false
        }"#;

        let res = fs::write(OPTIONS_JSON_PATH, default_options);
        if let Err(e) = res { println!("{}", e) }

        return Options { four_k_mode: false, borderless: false, vsync: true, fullscreen: false }
    }

    let contents = fs::read_to_string(OPTIONS_JSON_PATH).unwrap_or_default();

    let parsed: serde_json::Value = serde_json::from_str(contents.as_str()).expect("Unable to open the options file.");

    // attempt to parser the values, but assume false if none are found
    let four_k_mode = parsed["four_k_mode"].as_bool().unwrap_or(false);
    let borderless = parsed["borderless"].as_bool().unwrap_or(false);
    let vsync = parsed["vsync"].as_bool().unwrap_or(false);
    let fullscreen = parsed["fullscreen"].as_bool().unwrap_or(false);

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

    let options_as_json = format!(r#"
        {{
            "four_k_mode": {},
            "borderless": {},
            "vsync": {},
            "fullscreen": {}
        }}"#,
        current_options.four_k_mode,
        current_options.borderless,
        current_options.vsync,
        current_options.fullscreen
    );

    let res = fs::write(OPTIONS_JSON_PATH, options_as_json);
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