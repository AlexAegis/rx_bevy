use rx_bevy::prelude::*;

fn main() {
	of("world").subscribe(DynFnObserver::new().with_on_push(|next| println!("hello {next}")));
}
