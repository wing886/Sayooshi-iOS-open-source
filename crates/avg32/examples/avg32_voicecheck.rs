//! Reports the voice-patch table found for a game and decodes one voice.
fn main() {
    if let Ok(nls) = std::env::var("AVG32_NLS") {
        avg32::nls::set(nls.parse().expect("AVG32_NLS: sjis, gbk, big5 or utf8"));
    }
    let root = std::env::args()
        .nth(1)
        .expect("usage: avg32_voicecheck <game-root>");
    let game = avg32::Avg32Game::open(&root).expect("open game");
    let start = std::time::Instant::now();
    let patch = avg32::voicepatch::VoicePatch::discover(&game);
    println!(
        "{:?} entries in {:?}",
        patch.as_ref().map(|patch| patch.len()),
        start.elapsed()
    );
    if let Some(voice) = patch.and_then(|patch| patch.voice_at(163, 2011)) {
        let start = std::time::Instant::now();
        match avg32::sound::decode_afs_voice(&game.layout.root, voice as u32) {
            Ok(clip) => println!(
                "voice {voice:#x}: {} bytes of WAV in {:?}",
                clip.wav.len(),
                start.elapsed()
            ),
            Err(error) => println!("voice {voice:#x}: {error:#}"),
        }
    }
}
