use rx_bevy::prelude::*;

fn main() {
	of("world").subscribe(FnObserver::new(|next| println!("hello {next}")));
}
