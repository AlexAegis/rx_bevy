use rx_bevy::prelude::*;

fn main() {
	of("world").subscribe(DynFnObserver::default().with_next(|next| println!("hello {next}")));
}
