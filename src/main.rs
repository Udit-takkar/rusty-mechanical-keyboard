use rand::Rng;
use rdev::{EventType, Key, listen};
use rodio::{Decoder, OutputStream, Sink};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::sync::{Arc, mpsc};
use std::thread;

struct SoundPool {
    sinks: VecDeque<Arc<Sink>>,
    current_index: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct SoundConfig {
    id: String,
    name: String,
    key_define_type: String,
    includes_numpad: bool,
    sound: String,
    defines: HashMap<String, Option<String>>,
}

impl SoundPool {
    fn new(stream_handle: &rodio::OutputStreamHandle, pool_size: usize) -> Self {
        let mut sinks = VecDeque::new();
        for _ in 0..pool_size {
            sinks.push_back(Arc::new(Sink::try_new(stream_handle).unwrap()));
        }

        SoundPool {
            sinks,
            current_index: 0,
        }
    }

    fn play_sound(&mut self, sound_data: Vec<u8>) {
        let sink = &self.sinks[self.current_index];
        let cursor = std::io::Cursor::new(sound_data);
        if let Ok(source) = Decoder::new(cursor) {
            sink.append(source);
        }

        // Round-robin through sinks - use len() instead of the VecDeque itself
        self.current_index = (self.current_index + 1) % self.sinks.len();
    }
}

struct SoundManager {
    sounds: HashMap<String, Vec<u8>>,
    sound_pool: SoundPool,
    available_sounds: Vec<String>,
}

impl SoundManager {
    fn new(stream_handle: &rodio::OutputStreamHandle) -> Result<Self, Box<dyn std::error::Error>> {
        let mut sounds = HashMap::new();
        let mut available_sounds = Vec::new();

        let config_paths = [
            "nk-cream/config.json",
            "./nk-cream/config.json",
            "../nk-cream/config.json",
            "rusty-mechanical-keyboard/nk-cream/config.json",
            "./rusty-mechanical-keyboard/nk-cream/config.json",
        ];

        println!("Current working directory: {:?}", std::env::current_dir());
        println!("Contents of current directory:");
        if let Ok(entries) = std::fs::read_dir(".") {
            for entry in entries {
                if let Ok(entry) = entry {
                    println!("  - {:?}", entry.path());
                }
            }
        }
        println!("Looking for config file in:");
        for path in &config_paths {
            println!(
                "  - {} (exists: {})",
                path,
                std::path::Path::new(path).exists()
            );
        }

        let mut config_file = None;
        for path in &config_paths {
            if let Ok(file) = File::open(path) {
                println!("Found config at: {}", path);
                config_file = Some(file);
                break;
            }
        }

        let config_file =
            config_file.ok_or("Could not find nk-cream/config.json in any expected location")?;
        let config: SoundConfig = serde_json::from_reader(config_file)?;

        println!("Found config file, loading sounds...");

        for (key, filename_opt) in &config.defines {
            if let Some(filename) = filename_opt {
                let sound_paths = [
                    format!("nk-cream/{}", filename),
                    format!("./nk-cream/{}", filename),
                    format!("../nk-cream/{}", filename),
                    format!("rusty-mechanical-keyboard/nk-cream/{}", filename),
                    format!("./rusty-mechanical-keyboard/nk-cream/{}", filename),
                ];

                for sound_path in &sound_paths {
                    if let Ok(sound_data) = std::fs::read(sound_path) {
                        sounds.insert(key.clone(), sound_data);
                        available_sounds.push(key.clone());
                        break;
                    }
                }
            }
        }

        println!("Loaded {} NK Cream sounds", sounds.len());

        let sound_pool = SoundPool::new(stream_handle, 8);

        Ok(SoundManager {
            sounds,
            sound_pool,
            available_sounds,
        })
    }

    fn play_sound_by_key(&mut self, sound_key: &str) {
        if let Some(sound_data) = self.sounds.get(sound_key) {
            self.sound_pool.play_sound(sound_data.clone());
        } else {
            self.play_random_sound();
        }
    }

    fn play_random_sound(&mut self) {
        if self.available_sounds.is_empty() {
            return;
        }

        let mut rng = rand::thread_rng();
        let randome_key = &self.available_sounds[rng.gen_range(0..self.available_sounds.len())];

        if let Some(sound_data) = self.sounds.get(randome_key) {
            self.sound_pool.play_sound(sound_data.clone());
        }
    }
}

fn get_sound_key_for_key(key: &Key) -> String {
    match key {
        Key::KeyQ => "1".to_string(),
        Key::KeyW => "2".to_string(),
        Key::KeyE => "3".to_string(),
        Key::KeyR => "5".to_string(),
        Key::KeyT => "6".to_string(),
        Key::KeyY => "7".to_string(),
        Key::KeyU => "8".to_string(),
        Key::KeyI => "9".to_string(),
        Key::KeyO => "10".to_string(),
        Key::KeyP => "11".to_string(),
        Key::KeyA => "30".to_string(),
        Key::KeyS => "31".to_string(),
        Key::KeyD => "32".to_string(),
        Key::KeyF => "33".to_string(),
        Key::KeyG => "34".to_string(),
        Key::KeyH => "35".to_string(),
        Key::KeyJ => "36".to_string(),
        Key::KeyK => "37".to_string(),
        Key::KeyL => "38".to_string(),
        Key::KeyZ => "44".to_string(),
        Key::KeyX => "45".to_string(),
        Key::KeyC => "46".to_string(),
        Key::KeyV => "47".to_string(),
        Key::KeyB => "48".to_string(),
        Key::KeyN => "49".to_string(),
        Key::KeyM => "50".to_string(),
        Key::Space => "57".to_string(),
        Key::Return => "28".to_string(),
        Key::Backspace => "14".to_string(),
        Key::Tab => "15".to_string(),
        Key::CapsLock => "58".to_string(),
        Key::ShiftLeft | Key::ShiftRight => "42".to_string(),
        _ => "1".to_string(),
    }
}

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let sound_manager = match SoundManager::new(&stream_handle) {
        Ok(manager) => manager,
        Err(e) => {
            println!("Error loading sounds: {}", e);
            return;
        }
    };

    let (tx, rx) = mpsc::channel::<String>();
    let mut sound_manager_clone = sound_manager;

    thread::spawn(move || {
        for sound_key in rx {
            sound_manager_clone.play_sound_by_key(&sound_key);
        }
    });

    println!("NK Cream Mechanical Keyboard Sound Simulator");
    println!("Press Ctrl+C to exit");
    println!("Listening for keyboard input...");

    if let Err(error) = listen(move |event| {
        if let EventType::KeyPress(key) = event.event_type {
            println!("Key pressed: {:?}", key);

            // Send sound key to audio thread (non-blocking)
            let sound_key = get_sound_key_for_key(&key);
            let _ = tx.send(sound_key);
        }
    }) {
        println!("Error: {:?}", error);
    }
}
