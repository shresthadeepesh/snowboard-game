// Generates a pleasant ambient pentatonic soundtrack as a WAV file.
// Usage: rustc tools/gen_music.rs -o gen_music && ./gen_music

use std::f64::consts::PI;
use std::io::Write;

const SAMPLE_RATE: u32 = 44100;
const DURATION_SECS: f64 = 40.0;
const NUM_SAMPLES: u32 = (SAMPLE_RATE as f64 * DURATION_SECS) as u32;

fn main() {
    let mut samples = vec![0.0f64; NUM_SAMPLES as usize];

    // ── Gentle pad drone (low C3 + G3) ──
    let pad_freqs = [130.81, 196.0];
    for &freq in &pad_freqs {
        for i in 0..NUM_SAMPLES as usize {
            let t = i as f64 / SAMPLE_RATE as f64;
            let env = 0.12 * (0.5 + 0.5 * (2.0 * PI * t / 10.0).sin());
            samples[i] += env * (2.0 * PI * freq * t).sin();
            samples[i] += env * 0.3 * (2.0 * PI * freq * 2.0 * t).sin();
        }
    }

    // ── Pentatonic melody notes ──
    let notes = [261.63, 293.66, 329.63, 392.00, 440.00, 523.25, 587.33, 659.25];

    let melody: Vec<usize> = vec![
        0, 2, 4, 3, 5, 4, 2, 1, 0, 3, 2, 4, 5, 7, 6, 4,
        3, 5, 4, 2, 0, 1, 3, 2, 4, 6, 5, 3, 2, 0, 1, 4,
    ];
    let note_duration = 1.25;
    let fade_time = 0.4;

    for (idx, &note_idx) in melody.iter().enumerate() {
        let freq = notes[note_idx];
        let start_sample = (idx as f64 * note_duration * SAMPLE_RATE as f64) as usize;
        let note_samples = (note_duration * SAMPLE_RATE as f64 * 1.5) as usize;

        for j in 0..note_samples {
            let si = start_sample + j;
            if si >= NUM_SAMPLES as usize {
                break;
            }
            let t = j as f64 / SAMPLE_RATE as f64;
            let total_t = t;

            let env = if total_t < fade_time {
                total_t / fade_time
            } else if total_t > note_duration * 1.5 - fade_time {
                (note_duration * 1.5 - total_t) / fade_time
            } else {
                1.0
            }
            .max(0.0);

            let env = env * env;
            let volume = 0.15;

            let wave = (2.0 * PI * freq * t).sin()
                + 0.2 * (2.0 * PI * freq * 2.0 * t).sin()
                + 0.08 * (2.0 * PI * freq * 3.0 * t).sin();

            samples[si] += volume * env * wave;
        }
    }

    // ── Soft high sparkle notes ──
    let sparkle_times = [
        2.5, 5.8, 8.2, 11.0, 14.5, 17.3, 20.1, 23.7, 26.0, 29.5, 32.8, 35.5, 38.0,
    ];
    let sparkle_notes = [5, 6, 7, 4, 7, 5, 6, 4, 7, 5, 6, 7, 4];

    for (i, &time) in sparkle_times.iter().enumerate() {
        let freq = notes[sparkle_notes[i]] * 2.0;
        let start = (time * SAMPLE_RATE as f64) as usize;
        let dur = (0.8 * SAMPLE_RATE as f64) as usize;

        for j in 0..dur {
            let si = start + j;
            if si >= NUM_SAMPLES as usize {
                break;
            }
            let t = j as f64 / SAMPLE_RATE as f64;
            let env = (-t * 4.0).exp();
            samples[si] += 0.06 * env * (2.0 * PI * freq * t).sin();
        }
    }

    // ── Crossfade loop edges ──
    let crossfade_samples = (2.0 * SAMPLE_RATE as f64) as usize;
    for i in 0..crossfade_samples {
        let fade = i as f64 / crossfade_samples as f64;
        let end_idx = NUM_SAMPLES as usize - crossfade_samples + i;
        let blended = samples[end_idx] * (1.0 - fade) + samples[i] * fade;
        samples[end_idx] = blended;
    }
    let intro_fade = (1.0 * SAMPLE_RATE as f64) as usize;
    for i in 0..intro_fade {
        let fade = i as f64 / intro_fade as f64;
        samples[i] *= fade;
    }

    // ── Normalize and write WAV ──
    let max_val = samples.iter().map(|s| s.abs()).fold(0.0f64, f64::max);
    let scale = if max_val > 0.0 { 0.85 / max_val } else { 1.0 };
    let pcm: Vec<i16> = samples.iter().map(|&s| (s * scale * 32767.0) as i16).collect();

    let data_size = pcm.len() as u32 * 2;
    let file_size = 36 + data_size;
    let mut buf: Vec<u8> = Vec::with_capacity(file_size as usize + 8);
    buf.extend_from_slice(b"RIFF");
    buf.extend_from_slice(&file_size.to_le_bytes());
    buf.extend_from_slice(b"WAVE");
    buf.extend_from_slice(b"fmt ");
    buf.extend_from_slice(&16u32.to_le_bytes());
    buf.extend_from_slice(&1u16.to_le_bytes());
    buf.extend_from_slice(&1u16.to_le_bytes());
    buf.extend_from_slice(&SAMPLE_RATE.to_le_bytes());
    buf.extend_from_slice(&(SAMPLE_RATE * 2).to_le_bytes());
    buf.extend_from_slice(&2u16.to_le_bytes());
    buf.extend_from_slice(&16u16.to_le_bytes());
    buf.extend_from_slice(b"data");
    buf.extend_from_slice(&data_size.to_le_bytes());
    for sample in &pcm {
        buf.extend_from_slice(&sample.to_le_bytes());
    }

    let mut file = std::fs::File::create("assets/music.wav").expect("Failed to create WAV file");
    file.write_all(&buf).expect("Failed to write WAV");
    println!("Generated assets/music.wav ({:.1}s, {} bytes)", DURATION_SECS, buf.len());
}
