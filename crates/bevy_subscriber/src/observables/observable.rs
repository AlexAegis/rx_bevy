pub trait Observable<Destination> {
	type Out;

	fn subscribe(self, observer: Destination);
}
