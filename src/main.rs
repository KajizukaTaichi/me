use clap::Parser;
use clearscreen::clear;
use rodio::{Decoder, OutputStream, Sink};
use rustyline::DefaultEditor;
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

#[derive(Parser, Clone, Debug)]
#[command(
    name = "me",
    version = "0.1.0",
    author = "梶塚太智, kajizukataichi@outlook.jp",
    about = "a music player"
)]
struct Cli {
    #[arg(index = 1)]
    path: String,

    #[arg(long, short)]
    speed: Option<f32>,
}

fn main() {
    clear().unwrap();
    let cli = Cli::parse();
    println!("Play {} music\n", &cli.path);
    play(&cli.path, cli.speed.unwrap_or(1.0))
}

fn play(path: &str, speed: f32) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let file = File::open(path).unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();

    sink.append(source);
    sink.set_speed(speed);
    sink.play();

    let mut rl = DefaultEditor::new().unwrap();

    loop {
        println!("Commands: [p]Pause [r]Restart [q]Quit [s]Speed [c]Current [v]Volume");
        let command = rl.readline("> ").unwrap().trim().to_string();
        rl.add_history_entry(&command).unwrap_or_default();
        let command: Vec<&str> = command.split_whitespace().collect();
        clear().unwrap(); // Clear screen

        if let Some(order) = command.get(0) {
            match order.to_owned() {
                "p" => {
                    sink.pause();
                    println!("Playing is paused")
                }
                "r" => {
                    sink.play();
                    println!("Playing is restarted")
                }
                "c" => {
                    if let Some(pos) = command.get(1) {
                        if let Ok(pos) = pos.parse() {
                            let result = sink.try_seek(Duration::from_secs_f64(pos));
                            if result.is_ok() {
                                println!("Current playing position is changed")
                            } else {
                                println!("Error! {result:?}");
                            }
                        } else {
                            println!("Error! it's not a number");
                        }
                    } else {
                        println!("Current playing position is {:?}", sink.get_pos())
                    }
                }
                "q" => {
                    sink.stop();
                    println!("Playing was quit");
                    break;
                }
                "s" => {
                    if let Some(speed) = command.get(1) {
                        if let Ok(speed) = speed.parse() {
                            if 1.5 >= speed && speed >= 0.5 {
                                sink.set_speed(speed);
                                println!("Playing speed is changed");
                            } else {
                                println!("Error! specified speed is too extreme");
                            }
                        } else {
                            println!("Error! it's not a number");
                        };
                    } else {
                        println!("Playing speed is {}", sink.speed());
                    }
                }
                "v" => {
                    if let Some(speed) = command.get(1) {
                        if let Ok(speed) = speed.parse() {
                            if 1.5 >= speed && speed >= 0.0 {
                                sink.set_volume(speed);
                                println!("Volume is changed");
                            } else {
                                println!("Error! specified volume is too extreme");
                            }
                        } else {
                            println!("Error! it's not a number");
                        };
                    } else {
                        println!("Volume is {}", sink.volume());
                    }
                }
                other => {
                    println!("Error! unknown command: `{other}`");
                }
            }
            println!();
        } else {
            println!("Please enter command\n");
        }
    }
}
