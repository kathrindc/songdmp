use mpris::PlayerFinder;
use std::fs;
use std::time::Duration;

const INFO_PATH: &'static str = "/tmp/songdmp.txt";

fn format_duration(d: Duration) -> String {
    let seconds = d.as_secs() % 60;
    let minutes = (d.as_secs() / 60) % 60;

    format!("{}:{:02}", minutes, seconds)
}

fn main() {
    let player = PlayerFinder::new()
        .expect("Could not connect to D-Bus")
        .find_active()
        .expect("Could not find any player");
    let identity = player.identity();

    println!("Connected to {:#?}", identity);
    println!("Writing to {:#?}", INFO_PATH);

    let mut tracker = player.track_progress(100).unwrap();
    let mut track_id = "".to_string();
    let mut title: String = "".to_string();
    let mut artists: String = "".to_string();
    let mut length = Duration::new(0, 0);
    let mut position;

    loop {
        let (progress, _) = tracker.tick();

        match player.get_metadata() {
            Ok(metadata) => {
                let new_track_id = metadata.track_id().to_string();

                if track_id != new_track_id {
                    track_id = new_track_id;
                    length = metadata.length().unwrap();
                    title = match metadata.title() {
                        Some(value) => value.to_string(),
                        None => "No song playing".to_string(),
                    };
                    artists = match metadata.artists() {
                        Some(value) => value.join(", "),
                        None => " ".to_string(),
                    };

                    println!("New track \"{}\" ({})", title, track_id);
                }
            }

            Err(_) => {}
        }

        position = progress.position();

        if !title.is_empty() {
            let position_text = format_duration(position);
            let length_text = format_duration(length);
            let preamble = format!("Now playing ({} / {})", position_text, length_text);
            let content = format!("{}\n{}\n{}", preamble, title, artists);

            fs::write(INFO_PATH, content).expect("Could not write dump file");
        } else {
            fs::write(INFO_PATH, "No song playing").expect("Could not write dump file");
        }
    }
}
