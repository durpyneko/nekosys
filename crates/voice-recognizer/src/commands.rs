use log::info;
use std::collections::HashMap;

pub fn handler(text: String) {
    let mut word_map: HashMap<&str, fn(String)> = HashMap::new();
    word_map.insert("open", open);

    let tokens: Vec<&str> = text.split_whitespace().collect();

    for (i, &token) in tokens.iter().enumerate() {
        if let Some(&command_fn) = word_map.get(token.to_lowercase().as_str()) {
            let target = tokens.get(i + 1).unwrap_or(&"<missing>").to_string();
            info!("CMD: '{}', Target: '{}'", token, target);
            if target != "<missing>" {
                command_fn(target);
            }
        }
    }
}

fn open(target: String) {
    info!("Opening: {}", target);
    // Get install apps and iter. otherwise fallback to config.json with predefined apps
    // on windows use C:\ProgramData\Microsoft\Windows\Start Menu\Programs as a list to match apps
}
