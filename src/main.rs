use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    style::{Color, SetBackgroundColor, Print, ResetColor},
    terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType},
    ExecutableCommand,
};
use std::{
    io::{self, Write, Result},
    sync::{Arc, Mutex, mpsc::channel},
};
use wave::{ 
    settings::Settings,
    wave::{Wave, WaveType},
};
use cpal::{Device, SupportedStreamConfig};

const C4: f32 = 261.63;
#[allow(dead_code)]
const D4: f32 = 293.66;
#[allow(dead_code)]
const E4: f32 = 329.63;
#[allow(dead_code)]
const F4: f32 = 349.23;
#[allow(dead_code)]
const G4: f32 = 392.0;
#[allow(dead_code)]
const A4: f32 = 440.0;
#[allow(dead_code)]
const B4: f32 = 493.88;
#[allow(dead_code)]
const C5: f32 = 523.25;
#[allow(dead_code)]
const D5: f32 = 587.33;
#[allow(dead_code)]
const E5: f32 = 659.25;

fn switch_wave_type(wave_type: WaveType) -> WaveType {
    let wave_type = match wave_type {
        WaveType::Sine => WaveType::Square,
        WaveType::Square => WaveType::Sawtooth,
        WaveType::Sawtooth => WaveType::Triangle,
        WaveType::Triangle => WaveType::Sine,
    };   
    wave_type
}


fn main() -> Result<()> {

    enable_raw_mode()?; // Enable raw mode to capture key events directly
    let mut stdout = io::stdout();

    let settings = Settings::get_default_settings();
    let sample_rate = settings.get_sample_rate();
    let channels = settings.get_channels();
    let buffer_size = settings.get_buffer_size();
    let (_input_device, output_device) = settings.get_devices();
    let (_input_config, output_config) = settings.get_config_files();
    let device: Arc<Mutex<Device>> = Arc::new(Mutex::new(output_device));
    let config: Arc<Mutex<SupportedStreamConfig>> = Arc::new(Mutex::new(output_config));

    let duration: f32 = 3.0;


    execute!(stdout, Clear(ClearType::All))?;
    println!("Press 'a', 's', 'd', 'f', or 'j', 'k', 'l' to play notes. Press 'Esc' or 'q' to exit.");

    let mut wave_type: WaveType = WaveType::Sine;

    let device_clone = device.clone();
    let config_clone = config.clone();

    loop {
        let device_guard = device_clone.lock().unwrap();
        let config_guard = config_clone.lock().unwrap();
        // Wait for an event
        if event::poll(std::time::Duration::from_millis(500))? {
            // Read the event
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('v') => {
                        wave_type = switch_wave_type(wave_type);
                    },
                    KeyCode::Char('a') => { 
                        // Sound
                        let (output_sine, input_playback) = channel();
                        let wave = Wave::new(C4, 1.0, sample_rate as u32, channels as usize, output_sine, buffer_size, wave_type, Some(duration));
                        wave.play(input_playback, buffer_size, &device_guard, &config_guard, duration);
                        // Visual
                        stdout.execute(SetBackgroundColor(Color::Blue))?;
                        stdout.execute(Print("Do4"))?;
                    },
                    KeyCode::Char('s') => {
                        let (output_sine, input_playback) = channel();
                        let wave = Wave::new(D4, 1.0, sample_rate as u32, channels as usize, output_sine, buffer_size, wave_type, Some(duration));
                        wave.play(input_playback, buffer_size, &device_guard, &config_guard, duration);
                        // Visual
                        stdout.execute(SetBackgroundColor(Color::Green))?;
                        stdout.execute(Print("Re4"))?;
                    },
                    KeyCode::Char('d') => {
                        let (output_sine, input_playback) = channel();
                        let wave = Wave::new(E4, 1.0, sample_rate as u32, channels as usize, output_sine, buffer_size, wave_type, Some(duration));
                        wave.play(input_playback, buffer_size, &device_guard, &config_guard, duration);
                        // Visual
                        stdout.execute(SetBackgroundColor(Color::Red))?;
                        stdout.execute(Print("Mi4"))?;
                    }, 
                    KeyCode::Char('f') => {
                        let (output_sine, input_playback) = channel();
                        let wave = Wave::new(F4, 1.0, sample_rate as u32, channels as usize, output_sine, buffer_size, wave_type, Some(duration));
                        wave.play(input_playback, buffer_size, &device_guard, &config_guard, duration);
                        // Visual
                        stdout.execute(SetBackgroundColor(Color::Yellow))?;
                        stdout.execute(Print("Fa4"))?;
                    },
                    KeyCode::Char('g') => {
                        let (output_sine, input_playback) = channel();
                        let wave = Wave::new(G4, 1.0, sample_rate as u32, channels as usize, output_sine, buffer_size, wave_type, Some(duration));
                        wave.play(input_playback, buffer_size, &device_guard, &config_guard, duration);
                        // Visual
                        stdout.execute(SetBackgroundColor(Color::Magenta))?;
                        stdout.execute(Print("Sol4"))?;
                    }, 
                    KeyCode::Char('h') => {
                        let (output_sine, input_playback) = channel();
                        let wave = Wave::new(A4, 1.0, sample_rate as u32, channels as usize, output_sine, buffer_size, wave_type, Some(duration));
                        wave.play(input_playback, buffer_size, &device_guard, &config_guard, duration);
                        // Visual
                        stdout.execute(SetBackgroundColor(Color::Cyan))?;
                        stdout.execute(Print("La4"))?;
                    },
                    KeyCode::Char('j') => {
                        let (output_sine, input_playback) = channel();
                        let wave = Wave::new(B4, 1.0, sample_rate as u32, channels as usize, output_sine, buffer_size, wave_type, Some(duration));
                        wave.play(input_playback, buffer_size, &device_guard, &config_guard, duration);
                        // Visual
                        stdout.execute(SetBackgroundColor(Color::White))?;
                        stdout.execute(Print("Si4"))?;
                    }, 
                    KeyCode::Char('k') => {
                        let (output_sine, input_playback) = channel();
                        let wave = Wave::new(C5, 1.0, sample_rate as u32, channels as usize, output_sine, buffer_size, wave_type, Some(duration));
                        wave.play(input_playback, buffer_size, &device_guard, &config_guard, duration);
                        // Visual
                        stdout.execute(SetBackgroundColor(Color::DarkBlue))?;
                        stdout.execute(Print("Do5"))?;
                    },
                    KeyCode::Char('l') => {
                        let (output_sine, input_playback) = channel();
                        let wave = Wave::new(D5, 1.0, sample_rate as u32, channels as usize, output_sine, buffer_size, wave_type, Some(duration));
                        wave.play(input_playback, buffer_size, &device_guard, &config_guard, duration);
                        // Visual
                        stdout.execute(SetBackgroundColor(Color::DarkGreen))?;
                        stdout.execute(Print("Re5"))?;
                    },
                    KeyCode::Char(';') => {
                        let (output_sine, input_playback) = channel();
                        let wave = Wave::new(E5, 1.0, sample_rate as u32, channels as usize, output_sine, buffer_size, wave_type, Some(duration));
                        wave.play(input_playback, buffer_size, &device_guard, &config_guard, duration);
                        // Visual
                        stdout.execute(SetBackgroundColor(Color::DarkRed))?;
                        stdout.execute(Print("Mi5"))?;
                    },
                    KeyCode::Char('q') | KeyCode::Esc => {
                        execute!(stdout, Clear(ClearType::All))?;
                        writeln!(stdout, "Exiting...")?;
                        break;
                    },
                    _ => (),
                }
                stdout.execute(ResetColor)?; // Reset the color after printing the text
                writeln!(stdout)?; // Ensure the output is displayed and move to the next line
                stdout.flush()?; // Ensure the output is displayed immediately
            }
        }
    }

    disable_raw_mode()?; // Disable raw mode and return terminal to normal state
    Ok(())
}
