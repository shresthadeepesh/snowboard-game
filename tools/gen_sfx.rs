// Generates yeti roar and obstacle hit sound effects as WAV files.
// Usage: rustc tools/gen_sfx.rs -o gen_sfx && ./gen_sfx

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

    let mut file = std::fs::File::create(path).expect("Failed to create file");
    file.write_all(&buf).expect("Failed to write");
    println!("Generated {} ({:.2}s, {} bytes)", path, samples.len() as f64 / SAMPLE_RATE as f64, buf.len());
}

fn noise(seed: u64) -> f64 {
    let mut x = seed;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    (x as f64 / u64::MAX as f64) * 2.0 - 1.0
}

fn gen_yeti_roar() {
    let duration = 2.0;
    let num = (SAMPLE_RATE as f64 * duration) as usize;
    let mut samples = vec![0.0f64; num];

    for i in 0..num {
        let t = i as f64 / SAMPLE_RATE as f64;

        // Overall envelope: quick attack, long decay
        let env = if t < 0.1 {
            t / 0.1
        } else {
            (-((t - 0.1) * 1.8)).exp()
        };

        // Low growl: frequency drops from ~150Hz to ~60Hz
        let freq = 150.0 - 90.0 * (t / duration).min(1.0);
        // Frequency modulation for gritty texture
        let fm = 8.0 * (2.0 * PI * 5.0 * t).sin();
        let growl = (2.0 * PI * (freq + fm) * t).sin();

        // Add harmonics for roughness
        let h2 = 0.5 * (2.0 * PI * (freq * 2.0 + fm * 1.5) * t).sin();
        let h3 = 0.3 * (2.0 * PI * (freq * 3.0) * t).sin();

        // Add noise burst for "breath"
        let n = noise((i as u64).wrapping_mul(2654435761));
        let noise_env = if t < 0.3 { (0.3 - t) / 0.3 } else { 0.0 } * 0.4;

        // Amplitude tremolo
        let tremolo = 0.8 + 0.2 * (2.0 * PI * 12.0 * t).sin();

        samples[i] = env * tremolo * (growl + h2 + h3 + n * noise_env);
    }

    write_wav("assets/yeti_roar.wav", &samples);
}

fn gen_hit() {
    let duration = 0.5;
    let num = (SAMPLE_RATE as f64 * duration) as usize;
    let mut samples = vec![0.0f64; num];

    for i in 0..num {
        let t = i as f64 / SAMPLE_RATE as f64;

        // Sharp impact envelope
        let env = (-t * 15.0).exp();

        // Low thud
        let thud_freq = 80.0 * (-t * 8.0).exp() + 40.0;
        let thud = (2.0 * PI * thud_freq * t).sin();

        // Noise burst (crunch)
        let n = noise((i as u64).wrapping_mul(6364136223846793005));
        let noise_env = (-t * 25.0).exp();

        // Mid crack
        let crack = 0.4 * (2.0 * PI * 400.0 * t).sin() * (-t * 30.0).exp();

        samples[i] = env * thud * 0.7 + n * noise_env * 0.5 + crack;
    }

    write_wav("assets/hit.wav", &samples);
}

fn main() {
    gen_yeti_roar();
    gen_hit();
}
