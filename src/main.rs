use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    style::{Color, SetBackgroundColor, Print, ResetColor},
    terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType},
    ExecutableCommand,
    cursor::{MoveTo, Hide, Show},
};
use std::{
    io::{self, Write, Result, stdout},
    sync::{Arc, Mutex, mpsc::channel},
};
use wave::{ 
    settings::Settings,
    wave::{Wave, WaveType},
};
use cpal::{Device, SupportedStreamConfig};


fn switch_wave_type(wave_type: WaveType) -> WaveType {
    let wave_type = match wave_type {
        WaveType::Sine => WaveType::Square,
        WaveType::Square => WaveType::Sawtooth,
        WaveType::Sawtooth => WaveType::Triangle,
        WaveType::Triangle => WaveType::Sine,
    };   
    wave_type
}

struct TerminalState {
    current_row: u16,
    max_rows: u16,
    max_cols: u16,
}

impl TerminalState {
    fn new(max_rows: u16, max_cols: u16) -> Self {
        TerminalState {
            current_row: 0,
            max_rows,
            max_cols,
        }
    }

    fn next_position(&mut self) {
        self.current_row += 1;
            if self.current_row >= self.max_rows {
                self.current_row = 0;
            }
    }
}

lazy_static::lazy_static! {
    static ref TERMINAL_STATE: Mutex<TerminalState> = Mutex::new(TerminalState::new(24, 80));
}

fn print_color_square(color: Color) {
    let mut stdout = stdout();
    let mut state = TERMINAL_STATE.lock().unwrap();

    //  Move to the current position
    execute!(
        stdout,
        Hide,
        MoveTo(0, state.current_row),
        SetBackgroundColor(color),
        Print(" ".repeat(state.max_cols as usize)),
        Show
    ).unwrap();

    state.next_position();
}

const C4: f32 = 261.63;
const D4: f32 = 293.66;
const E4: f32 = 329.63;
const F4: f32 = 349.23;
const G4: f32 = 392.0;
const A4: f32 = 440.0;
const B4: f32 = 493.88;
const C5: f32 = 523.25;
const D5: f32 = 587.33;
const E5: f32 = 659.25;

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

    let duration: f32 = 2.0;
    let attack: f32 = 100.0;
    // Add decay rate (fade out time)
    


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

            // Add functionality for changing the octave (Z, X)
            // Add functionality for changing Velocity (C, V)
            // Add pitch bend functionality? (Maybe)
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('v') => {
                        wave_type = switch_wave_type(wave_type);
                    },
                    KeyCode::Char('a') => { 
                        // Sound
                        let (output_sine, input_playback) = channel();
                        let wave = Wave::new(C4, 1.0, sample_rate as u32, channels as usize, output_sine, buffer_size, wave_type, Some(duration), attack);
                        wave.play(input_playback, buffer_size, &device_guard, &config_guard, duration);
                        // Visual
                        print_color_square(Color::Blue);
                    },
                    KeyCode::Char('s') => {
                        let (output_sine, input_playback) = channel();
                        let wave = Wave::new(D4, 1.0, sample_rate as u32, channels as usize, output_sine, buffer_size, wave_type, Some(duration), attack);
                        wave.play(input_playback, buffer_size, &device_guard, &config_guard, duration);
                        // Visual
                        print_color_square(Color::Green);
                    },
                    KeyCode::Char('d') => {
                        let (output_sine, input_playback) = channel();
                        let wave = Wave::new(E4, 1.0, sample_rate as u32, channels as usize, output_sine, buffer_size, wave_type, Some(duration), attack);
                        wave.play(input_playback, buffer_size, &device_guard, &config_guard, duration);
                        // Visual
                        print_color_square(Color::Red);
                    }, 
                    KeyCode::Char('f') => {
                        let (output_sine, input_playback) = channel();
                        let wave = Wave::new(F4, 1.0, sample_rate as u32, channels as usize, output_sine, buffer_size, wave_type, Some(duration), attack);
                        wave.play(input_playback, buffer_size, &device_guard, &config_guard, duration);
                        // Visual
                        print_color_square(Color::Yellow);
                    },
                    KeyCode::Char('g') => {
                        let (output_sine, input_playback) = channel();
                        let wave = Wave::new(G4, 1.0, sample_rate as u32, channels as usize, output_sine, buffer_size, wave_type, Some(duration), attack);
                        wave.play(input_playback, buffer_size, &device_guard, &config_guard, duration);
                        // Visual
                        print_color_square(Color::Magenta);
                    }, 
                    KeyCode::Char('h') => {
                        let (output_sine, input_playback) = channel();
                        let wave = Wave::new(A4, 1.0, sample_rate as u32, channels as usize, output_sine, buffer_size, wave_type, Some(duration), attack);
                        wave.play(input_playback, buffer_size, &device_guard, &config_guard, duration);
                        // Visual
                        print_color_square(Color::Cyan);
                    },
                    KeyCode::Char('j') => {
                        let (output_sine, input_playback) = channel();
                        let wave = Wave::new(B4, 1.0, sample_rate as u32, channels as usize, output_sine, buffer_size, wave_type, Some(duration), attack);
                        wave.play(input_playback, buffer_size, &device_guard, &config_guard, duration);
                        // Visual
                        print_color_square(Color::White);
                    }, 
                    KeyCode::Char('k') => {
                        let (output_sine, input_playback) = channel();
                        let wave = Wave::new(C5, 1.0, sample_rate as u32, channels as usize, output_sine, buffer_size, wave_type, Some(duration), attack);
                        wave.play(input_playback, buffer_size, &device_guard, &config_guard, duration);
                        // Visual
                        print_color_square(Color::DarkBlue);
                    },
                    KeyCode::Char('l') => {
                        let (output_sine, input_playback) = channel();
                        let wave = Wave::new(D5, 1.0, sample_rate as u32, channels as usize, output_sine, buffer_size, wave_type, Some(duration), attack);
                        wave.play(input_playback, buffer_size, &device_guard, &config_guard, duration);
                        // Visual
                        print_color_square(Color::DarkGreen);
                    },
                    KeyCode::Char(';') => {
                        let (output_sine, input_playback) = channel();
                        let wave = Wave::new(E5, 1.0, sample_rate as u32, channels as usize, output_sine, buffer_size, wave_type, Some(duration), attack);
                        wave.play(input_playback, buffer_size, &device_guard, &config_guard, duration);
                        // Visual
                        print_color_square(Color::DarkRed);
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
