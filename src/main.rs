use clap::Parser;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::thread;

#[derive(Parser, Clone, Debug)]
#[command(
    name = "me",
    version = "0.1.0",
    author = "梶塚太智, kajizukataichi@outlook.jp",
    about = "a music player"
)]
struct Cli {
    #[arg(index = 1)]
    path: Vec<String>,

    #[arg(long, short)]
    speed: Option<f32>,
}

fn main() {
    let cli = Cli::parse();
    println!("Play {} music", &cli.path.join(", "));
    play(cli.path, cli.speed.unwrap_or(1.0))
}

fn play(path: Vec<String>, speed: f32) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    for path in path {
        let sink = Sink::try_new(&stream_handle).unwrap();
        let source = Decoder::new(BufReader::new(File::open(path).unwrap())).unwrap();

        sink.append(source);
        sink.set_speed(speed);

        thread::spawn(move || {
            sink.sleep_until_end();
        });
    }
    loop {}
}
