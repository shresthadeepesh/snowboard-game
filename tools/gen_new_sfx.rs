// Generates coin, powerup, near-miss, and game-over sound effects.
// Usage: rustc tools/gen_new_sfx.rs -o gen_new_sfx && ./gen_new_sfx && rm gen_new_sfx

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
    println!("Generated {} ({:.2}s)", path, samples.len() as f64 / SAMPLE_RATE as f64);
}

fn gen_coin() {
    // Bright, short ping — two ascending tones
    let duration = 0.25;
    let num = (SAMPLE_RATE as f64 * duration) as usize;
    let mut samples = vec![0.0f64; num];

    for i in 0..num {
        let t = i as f64 / SAMPLE_RATE as f64;
        let env = (-t * 12.0).exp();
        // Two quick ascending notes
        let freq1 = 1200.0;
        let freq2 = 1800.0;
        let blend = if t < 0.08 { 0.0 } else { 1.0 };
        let wave = (1.0 - blend) * (2.0 * PI * freq1 * t).sin()
            + blend * (2.0 * PI * freq2 * t).sin();
        samples[i] = env * wave * 0.6;
        // Add sparkle harmonic
        samples[i] += env * 0.2 * (2.0 * PI * freq2 * 2.0 * t).sin();
    }
    write_wav("assets/coin.wav", &samples);
}

fn gen_powerup() {
    // Ascending chime — three quick notes going up
    let duration = 0.4;
    let num = (SAMPLE_RATE as f64 * duration) as usize;
    let mut samples = vec![0.0f64; num];

    let notes = [523.25, 659.25, 783.99]; // C5, E5, G5
    let note_dur = 0.12;

    for (idx, &freq) in notes.iter().enumerate() {
        let start = (idx as f64 * note_dur * SAMPLE_RATE as f64) as usize;
        let len = (note_dur * 2.0 * SAMPLE_RATE as f64) as usize;
        for j in 0..len {
            let si = start + j;
            if si >= num { break; }
            let t = j as f64 / SAMPLE_RATE as f64;
            let env = (-t * 8.0).exp();
            samples[si] += env * 0.5 * (2.0 * PI * freq * t).sin();
            samples[si] += env * 0.15 * (2.0 * PI * freq * 2.0 * t).sin();
        }
    }
    write_wav("assets/powerup.wav", &samples);
}

fn gen_near_miss() {
    // Quick whoosh — filtered noise sweep
    let duration = 0.2;
    let num = (SAMPLE_RATE as f64 * duration) as usize;
    let mut samples = vec![0.0f64; num];

    let mut rng_state: u64 = 98765;
    let mut lp = 0.0f64;

    for i in 0..num {
        let t = i as f64 / SAMPLE_RATE as f64;
        rng_state = rng_state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let noise = (rng_state >> 33) as f64 / (u32::MAX as f64 / 2.0) - 1.0;

        // Sweeping cutoff
        let cutoff = 0.05 + t / duration * 0.3;
        lp += cutoff * (noise - lp);

        let env = if t < 0.05 { t / 0.05 } else { (-(t - 0.05) * 10.0).exp() };
        samples[i] = lp * env * 0.8;

        // Add a subtle tone
        samples[i] += env * 0.15 * (2.0 * PI * 800.0 * t).sin();
    }
    write_wav("assets/near_miss.wav", &samples);
}

fn gen_game_over() {
    // Low descending tone with crunch
    let duration = 1.0;
    let num = (SAMPLE_RATE as f64 * duration) as usize;
    let mut samples = vec![0.0f64; num];

    let mut rng_state: u64 = 54321;

    for i in 0..num {
        let t = i as f64 / SAMPLE_RATE as f64;
        let env = (-t * 2.5).exp();

        // Descending frequency
        let freq = 300.0 - 200.0 * (t / duration);
        let wave = (2.0 * PI * freq * t).sin();
        let h2 = 0.4 * (2.0 * PI * freq * 2.0 * t).sin();

        // Noise crunch at the start
        rng_state = rng_state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let noise = (rng_state >> 33) as f64 / (u32::MAX as f64 / 2.0) - 1.0;
        let noise_env = (-t * 8.0).exp();

        samples[i] = env * (wave + h2) * 0.5 + noise * noise_env * 0.3;
    }
    write_wav("assets/game_over.wav", &samples);
}

fn main() {
    gen_coin();
    gen_powerup();
    gen_near_miss();
    gen_game_over();
}
