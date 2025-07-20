use reqwest;
use urlencoding::encode;
use std::env;
use dotenv::dotenv;
use rodio::{source::Source, Decoder};
use hound::{WavWriter, WavSpec};
use std::io::Cursor;


struct ApiConfig {
    token: String,
}

impl ApiConfig {
    fn new() -> Self {
        dotenv().ok();
        let token = env::var("TTS_API_TOKEN")
            .expect("TTS_API_TOKEN must be set in environment variables");

        ApiConfig { token }
    }
}

#[tauri::command]
pub async fn generate_audio(text: String, voice: String) -> Result<Vec<u8>, String> {
    println!("Prompt {}", text);
    let api_config = ApiConfig::new();
    let encoded_text = encode(&text);
    let url = format!("https://text.pollinations.ai/{}", encoded_text);

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", api_config.token))
        .query(&[
            ("model", "openai-audio"),
            ("voice", &voice),
        ])
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;


    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("API error ({}): {}", status, error_text));
    }


    if let Some(content_type) = response.headers().get(reqwest::header::CONTENT_TYPE) {
        if content_type != "audio/mpeg" {
            return Err(format!("Unexpected content type: {:?}", content_type));
        }
    }

    let audio_data = response
        .bytes()
        .await
        .map_err(|e| e.to_string())?
        .to_vec();

    Ok(audio_data)

    // hayden_voice_mixer(audio_data).map_err(|e| e.to_string())
}

fn hayden_voice_mixer(input_audio: Vec<u8>) -> Result<Vec<u8>, String> {

    let cursor = Cursor::new(input_audio.clone());


    let source = match Decoder::new_mp3(cursor) {
        Ok(source) => source,
        Err(_) => {
            let cursor = Cursor::new(input_audio);
            Decoder::new_wav(cursor).map_err(|e| format!("Failed to decode audio: {}", e))?
        }
    };


    let sample_rate = source.sample_rate();
    let channels = source.channels();

    // Применяем эффекты для голоса Dr. Hayden
    let processed = source
        .speed(0.9)  // Замедление скорости (понижение тона)
        .map(|sample| {
            let sample = sample as f32 / i16::MAX as f32;
            // Добавляем искажение
            if sample.abs() > 0.3 {
                (sample.signum() * (0.3 + (sample.abs() - 0.3) / 1.3)) * i16::MAX as f32
            } else {
                sample * i16::MAX as f32
            }
        })
        .enumerate()
        .map(|(i, sample)| {

            if i % 100 == 0 {
                ((sample as f32 * 0.7) + (sample as f32 * 0.3 * (i % 3) as f32)) as i16
            } else {
                sample as i16
            }
        });


    let samples: Vec<i16> = processed.collect();


    let mut output_buffer = Vec::new();
    {
        let cursor = Cursor::new(&mut output_buffer);
        let spec = WavSpec {
            channels,
            sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut writer = WavWriter::new(cursor, spec)
            .map_err(|e| format!("Failed to create WAV writer: {}", e))?;

        for sample in samples {
            writer.write_sample(sample)
                .map_err(|e| format!("Failed to write sample: {}", e))?;
        }

        writer.finalize()
            .map_err(|e| format!("Failed to finalize WAV: {}", e))?;
    }

    Ok(output_buffer)
}

