pub trait StaticTrait: 'static {}

pub trait Representative: 'static {
	type Rep<'a>;
}

pub struct Func<T: Representative> {
	f: Box<dyn FnMut(&T::Rep<'_>)>,
}

impl<T: Representative> StaticTrait for Func<T> {}

impl Representative for String {
	type Rep<'a> = String;
}

impl Representative for &'static u8 {
	type Rep<'a> = &'a u8;
}

pub struct FuncU8 {
	f: Box<dyn FnMut(&&u8)>,
}

impl StaticTrait for FuncU8 {}

fn foo(u: FuncU8) {
	let f = u.f;
	let t: Func<&'static u8> = Func { f };
	let _: Box<dyn StaticTrait + 'static> = Box::new(t);
}

fn bar(f: fn(&String)) {
	let t: Func<String> = Func { f: Box::new(f) };
	let _: Box<dyn StaticTrait + 'static> = Box::new(t);
}

fn asd() {
	let u = FuncU8 {
		f: Box::new(|_| {}),
	};
	foo(u);
	bar(|_| {});
}
