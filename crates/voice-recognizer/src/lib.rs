use anyhow::Result;
use colored::Colorize;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use log::{debug, error, info};
use std::sync::{Arc, Mutex};
use vosk::{Model, Recognizer};

mod commands;

pub async fn init(voice_model: String) -> Result<()> {
    vosk::set_log_level(vosk::LogLevel::Warn);

    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("No input device available");
    let config = device.default_input_config()?;

    let model = Model::new(&voice_model);
    let sample_rate = config.sample_rate().0 as f32;
    let recognizer = Arc::new(Mutex::new(Recognizer::new(&model.unwrap(), sample_rate)));
    info!(
        "Loaded model: {} at {} sample rate",
        voice_model.bright_blue(),
        sample_rate.to_string().bright_blue()
    );

    recognizer
        .lock()
        .unwrap()
        .as_mut()
        .unwrap()
        .set_max_alternatives(1);
    recognizer.lock().unwrap().as_mut().unwrap().set_words(true);
    recognizer
        .lock()
        .unwrap()
        .as_mut()
        .unwrap()
        .set_partial_words(true);

    info!(
        "Using input device: {}",
        device.name().ok().unwrap().bright_blue()
    );

    let recognizer_clone = recognizer.clone();
    let mut p_sentence = String::new(); // partial sentence
    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |data: &[f32], _| {
                debug!("Received {} samples", data.len());
                let samples: Vec<i16> = data
                    .iter()
                    .map(|s| (s * i16::MAX as f32).clamp(i16::MIN as f32, i16::MAX as f32) as i16)
                    .collect();

                let mut rec_guard = recognizer_clone.lock().unwrap();
                let rec = rec_guard.as_mut().unwrap();

                match rec.accept_waveform(&samples) {
                    Ok(vosk::DecodingState::Finalized) => {
                        let final_result = rec.final_result();
                        let sentence = final_result
                            .multiple()
                            .unwrap()
                            .alternatives
                            .iter()
                            .map(|r| r.text)
                            .collect::<Vec<_>>()
                            .join(" ");
                        if !sentence.is_empty() {
                            info!("{} {}", "FINN".green(), sentence);
                            commands::handler(sentence);
                        }
                    }
                    Ok(vosk::DecodingState::Running) => {
                        let partial = rec.partial_result();
                        if !partial.partial_result.is_empty() {
                            let partial_s = partial
                                .partial_result
                                .iter()
                                .map(|w| w.word)
                                .collect::<Vec<_>>()
                                .join(" ");
                            // control spam
                            if partial_s != p_sentence {
                                p_sentence = partial_s;
                                info!("{} {}", "PART".yellow(), p_sentence);
                            }
                        }
                    }
                    Ok(vosk::DecodingState::Failed) => {}
                    Err(e) => error!("Waveform error: {:?}", e),
                }
            },
            move |err| error!("Stream error: {}", err),
            None,
        ),
        _ => panic!("Unsupported format"),
    }
    .expect("Failed to build input stream");

    info!("is listening...");
    stream.play()?;

    // Block forever
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
