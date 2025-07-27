use lazy_static::lazy_static;
use log::debug;
use log::info;
use nyannel::listen_spawn;
use std::sync::Arc;
use std::sync::Mutex;
use tray_item::{IconSource, TrayItem};

use crate::utils::snips::open_url;

lazy_static! {
    static ref TRAY_ICON: Arc<Mutex<Option<TrayItem>>> = Arc::new(Mutex::new(None));
}

// ! Couln't figure out tray-icon so i used tray-item, which is shit cuz not much custom stuff?
// * Might just make my own https://tenor.com/view/iron-man-iron-man-hammer-iron-hammer-robert-downey-robert-downey-jr-gif-15959050
pub fn init() -> tokio::task::JoinHandle<()> {
    tokio::spawn(async {
        debug!("Initializing tray icon...");

        let mut tray = TrayItem::new("NekoSys", IconSource::Resource("default")).unwrap();
        let ip_port = Arc::new(Mutex::new(String::new()));

        listen_spawn("tray", {
            let ip_port = ip_port.clone();
            move |msg| {
                if let Ok(mut ip) = ip_port.lock() {
                    if let Ok(msg_value) = serde_json::from_str::<serde_json::Value>(&msg) {
                        if let Some(location) = msg_value.get("location").and_then(|l| l.as_str()) {
                            *ip = location.to_string();
                        }
                    };
                    debug!("Updated tray IP:port to: {}", ip);
                }
            }
        })
        .expect("Failed to spawn tray listener");

        let tray_label_id = tray
            .inner_mut()
            .add_label_with_id("「 ✦ NekoSys ✦ 」")
            .unwrap();

        tray.add_menu_item("╰┈➤ Web UI", {
            let ip_port = ip_port.clone();
            move || {
                if let Ok(url) = ip_port.lock() {
                    if !url.is_empty() {
                        open_url(&*url);
                    }
                }
            }
        })
        .unwrap();
        tray.add_menu_item("╰┈➤ Config", || {
            // TODO!
            // * Open passed config in default editor
            // * getting the path via ipc (todo)
        })
        .unwrap();

        // tray.inner_mut().add_separator().unwrap();
        tray.add_label("───  ⋅ ∙ ∘ ☽ ༓ ☾ ∘ ⋅ ⋅  ───").unwrap();
        // ⋆ ⋅ ────────────────── ⋅ ⋆
        // ✦•·············•✦•·············•✦
        // ₊˚ ‿︵‿︵‿︵୨୧ · · ♡ · · ୨୧‿︵‿︵‿︵ ˚₊
        // ┈┈・୨ ✦ ୧・┈┈
        // ───  ⋅ ∙ ∘ ☽ ༓ ☾ ∘ ⋅ ⋅  ───
        // ❖≔﴾═══════ﺤ

        tray.add_menu_item("Quit", || {
            info!("Quitting application...");
            std::process::exit(0);
        })
        .unwrap();

        *TRAY_ICON.lock().unwrap() = Some(tray);

        set_tray_icon("online");
        set_tray_label("「 ✦ NekoSys - Online ✦ 」", tray_label_id);
        set_tray_tooltip("Online");

        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    })
}

pub fn set_tray_icon(icon: &'static str) {
    if let Some(tray) = TRAY_ICON.lock().unwrap().as_mut() {
        tray.set_icon(tray_item::IconSource::Resource(icon))
            .unwrap();
    }
}

pub fn set_tray_label(label: &str, label_id: u32) {
    if let Some(tray) = TRAY_ICON.lock().unwrap().as_mut() {
        tray.inner_mut().set_label(label, label_id).unwrap();
    }
}

pub fn set_tray_tooltip(label: &str) {
    if let Some(tray) = TRAY_ICON.lock().unwrap().as_mut() {
        tray.inner_mut().set_tooltip(label).unwrap();
    }
}
