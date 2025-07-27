use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;
use tokio::sync::broadcast::{self, Receiver, Sender};

static CHANNELS: Lazy<Mutex<HashMap<String, Sender<String>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Create a named broadcast channel. Fails if it already exists.
pub fn create(name: &str) -> Result<Receiver<String>, String> {
    let mut channels = CHANNELS.lock().unwrap();
    if channels.contains_key(name) {
        return Err(format!("Channel '{}' already exists", name));
    }

    let (tx, rx) = broadcast::channel(100); // buffer size
    channels.insert(name.to_string(), tx);
    Ok(rx)
}

/// Listen to an existing named channel. Returns a new receiver.
pub fn listen<T>(name: T) -> Result<Receiver<String>, String>
where
    T: AsRef<str>,
{
    let channels = CHANNELS.lock().unwrap();
    if let Some(sender) = channels.get(name.as_ref()) {
        Ok(sender.subscribe())
    } else {
        Err(format!("Channel '{}' not found", name.as_ref()))
    }
}
/// Listen to an existing named channel in a new thread, calling a callback on each message.
///
/// The callback is called with each received message. Returns an error if the channel doesn't exist.
/// The thread runs until the receiver is dropped or the channel is closed.
pub fn listen_spawn<T, F>(name: T, mut callback: F) -> Result<(), String>
where
    T: AsRef<str> + Send + 'static,
    F: FnMut(String) + Send + 'static,
{
    let rx = listen(name)?;
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let mut rx = rx;
        rt.block_on(async move {
            loop {
                match rx.recv().await {
                    Ok(msg) => callback(msg),
                    Err(broadcast::error::RecvError::Closed) => break,
                    Err(broadcast::error::RecvError::Lagged(_)) => continue,
                }
            }
        });
    });
    Ok(())
}

/// Send a message to a named channel
pub fn send(name: &str, msg: String) -> Result<(), String> {
    let channels = CHANNELS.lock().unwrap();
    if let Some(sender) = channels.get(name) {
        sender.send(msg).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err(format!("Channel '{}' not found", name))
    }
}
