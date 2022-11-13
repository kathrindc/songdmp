use mpris::PlayerFinder;

fn main() {
    let player = PlayerFinder::new()
        .expect("Could not connect to D-Bus")
        .find_active()
        .expect("Could not find any player");
    let metadata = player.get_metadata().expect("Could not find metadata");

    println!("{:#?}", metadata);
}
