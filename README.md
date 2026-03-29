# Snow Rider - Escape the Mountain!

A 2D downhill skiing game built with [Bevy](https://bevyengine.org/) (Rust). Dodge obstacles, outrun the yeti, and rack up the highest score.

## Running the Game

```bash
cargo run
```

## Controls

| Key | Action |
|---|---|
| Arrow Left / A | Turn left |
| Arrow Right / D | Turn right |
| ESC | Pause |
| R | Restart |
| Space / Enter | Start / Retry |
| Q | Quit to menu (from pause) |

## Generating Sound Assets

All audio files are procedurally generated using standalone Rust scripts in the `tools/` directory. The generated WAV files are placed in `assets/`.

### Background Music

Ambient pentatonic soundtrack (~40s loop).

```bash
rustc tools/gen_music.rs -o gen_music && ./gen_music && rm gen_music
```

### Sound Effects (Yeti Roar & Hit)

Yeti roar for when the yeti spawns, and an impact sound for obstacle collisions.

```bash
rustc tools/gen_sfx.rs -o gen_sfx && ./gen_sfx && rm gen_sfx
```

### Skiing Loop

Filtered white noise with carving rhythm modulation (~6s loop).

```bash
rustc tools/gen_ski.rs -o gen_ski && ./gen_ski && rm gen_ski
```

### Regenerate All

```bash
for f in tools/gen_*.rs; do
  name=$(basename "$f" .rs)
  rustc "$f" -o "$name" && ./"$name" && rm "$name"
done
```
