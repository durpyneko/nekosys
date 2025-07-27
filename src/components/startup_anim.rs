use std::io::Write;

pub fn animate() {
    let text = "Starting up..."; // ▷
    let throbber_frames = [
        "■□□□□",
        "□■□□□",
        "□□■□□",
        "□□□■□",
        "□□□□■",
        "□□□■□",
        "□□■□□",
        "□■□□□",
    ];
    let mut enu_text = String::new();
    let max_frame_len = throbber_frames.iter().map(|f| f.len()).max().unwrap_or(0);

    for (i, c) in text.chars().enumerate() {
        enu_text.push(c);

        if i < text.len() - 1 {
            let frame = throbber_frames[i % throbber_frames.len()];
            print!("\r{:width$}", frame, width = max_frame_len);
        } else {
            let clear_len = max_frame_len; // + 1 + enu_text.len()
            print!("\r{:width$}\r", "", width = clear_len);
        }

        std::io::stdout().flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
