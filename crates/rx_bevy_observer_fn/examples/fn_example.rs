use rx_bevy::prelude::*;

fn main() {
	of("world").subscribe(DynFnObserver::new().with_next(|next| println!("hello {next}")));
}
