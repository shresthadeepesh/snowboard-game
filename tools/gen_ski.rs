// Generates a skiing-on-snow loop sound effect as a WAV file.
// Uses filtered white noise with carving rhythm modulation.
// Usage: rustc tools/gen_ski.rs -o gen_ski && ./gen_ski

use std::f64::consts::PI;
use std::io::Write;

const SAMPLE_RATE: u32 = 44100;

fn write_wav(path: &str, samples: &[f64]) {
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
    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(&buf).unwrap();
    println!("Generated {} ({:.2}s, {} bytes)", path, samples.len() as f64 / SAMPLE_RATE as f64, buf.len());
}

fn main() {
    let duration = 6.0;
    let num = (SAMPLE_RATE as f64 * duration) as usize;
    let mut samples = vec![0.0f64; num];

    // Proper LCG white noise
    let mut rng_state: u64 = 123456789;
    let mut white_noise = vec![0.0f64; num];
    for i in 0..num {
        rng_state = rng_state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        white_noise[i] = (rng_state >> 33) as f64 / (u32::MAX as f64 / 2.0) - 1.0;
    }

    // ── Layer 1: Low-pass filtered noise (main "shush" of snow) ──
    let mut lp1 = 0.0f64;
    let cutoff1 = 0.08;
    for i in 0..num {
        lp1 += cutoff1 * (white_noise[i] - lp1);
        samples[i] += lp1 * 0.6;
    }

    // ── Layer 2: Band-pass filtered noise (crispy snow texture) ──
    let mut bp_low = 0.0f64;
    let mut bp_high = 0.0f64;
    let bp_cutoff = 0.15;
    let bp_res = 0.3;
    for i in 0..num {
        bp_low += bp_cutoff * bp_high;
        bp_high += bp_cutoff * (white_noise[i] - bp_low - bp_res * bp_high);
        let bandpass = bp_high;
        samples[i] += bandpass * 0.35;
    }

    // ── Layer 3: Gentle high-frequency hiss (snow spray) ──
    let mut hp_lp = 0.0f64;
    let hp_cutoff = 0.02;
    for i in 0..num {
        hp_lp += hp_cutoff * (white_noise[i] - hp_lp);
        let highpass = white_noise[i] - hp_lp;
        samples[i] += highpass * 0.1;
    }

    // ── Slow rhythmic modulation (carving turns) ──
    for i in 0..num {
        let t = i as f64 / SAMPLE_RATE as f64;
        let carve = 0.65 + 0.35 * (2.0 * PI * 0.7 * t).sin();
        let chatter = 0.92 + 0.08 * (2.0 * PI * 2.5 * t + 0.5).sin();
        samples[i] *= carve * chatter;
    }

    // ── Crossfade loop edges ──
    let fade_len = (1.5 * SAMPLE_RATE as f64) as usize;
    let start_copy: Vec<f64> = samples[..fade_len].to_vec();
    for i in 0..fade_len {
        let fade = i as f64 / fade_len as f64;
        let end_idx = num - fade_len + i;
        samples[end_idx] = samples[end_idx] * (1.0 - fade) + start_copy[i] * fade;
    }

    let intro = (0.3 * SAMPLE_RATE as f64) as usize;
    for i in 0..intro {
        samples[i] *= i as f64 / intro as f64;
    }

    write_wav("assets/ski_loop.wav", &samples);
}
