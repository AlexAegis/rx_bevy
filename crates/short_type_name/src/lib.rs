#[inline]
fn remove_leading_path(type_path: &'static str) -> &'static str {
	type_path.split("::").last().unwrap_or_default()
}

fn truncate_type_name(name: &'static str) -> String {
	let Some((base, untrimmed_generics)) = name.split_once('<') else {
		return remove_leading_path(name).to_owned();
	};
	let generics = untrimmed_generics.trim_end_matches(">").split(", ");
	let truncated_generics = generics
		.map(truncate_type_name)
		.collect::<Vec<_>>()
		.join(", ");
	remove_leading_path(base).to_owned() + "<" + &truncated_generics + ">"
}

/// Returns a type_name without the qualification, for the sake of brevity
pub fn short_type_name<T: ?Sized>() -> String {
	truncate_type_name(std::any::type_name::<T>())
}

#[cfg(test)]
mod test {
	use super::{short_type_name, truncate_type_name};

	pub mod foo {
		pub mod bar {
			use core::marker::PhantomData;

			pub struct Zed;

			pub struct Foo<T> {
				_phantom_data: PhantomData<T>,
			}

			pub struct Bar<T, P> {
				_phantom_data: PhantomData<(T, P)>,
			}
		}
	}

	#[test]
	fn should_print_and_actual_generic_type() {
		assert_eq!(
			short_type_name::<foo::bar::Bar<foo::bar::Foo<foo::bar::Zed>, foo::bar::Zed>>(),
			"Bar<Foo<Zed>, Zed>"
		);
	}

	#[test]
	fn should_do_nothing_with_empty_strings() {
		assert_eq!(truncate_type_name(""), "");
	}

	#[test]
	fn should_do_nothing_with_already_truncated_names() {
		assert_eq!(truncate_type_name("Type"), "Type");
	}

	#[test]
	fn should_truncate_non_generic_types() {
		assert_eq!(truncate_type_name("foo::bar::Type"), "Type");
	}

	#[test]
	fn should_truncate_generic_types_where_the_generic_is_already_short() {
		assert_eq!(truncate_type_name("foo::bar::Type<i32>"), "Type<i32>");
	}

	#[test]
	fn should_truncate_multiple_generics() {
		assert_eq!(
			truncate_type_name("foo::bar::Type<foo::bar::Foo, foo::bar::Bar>"),
			"Type<Foo, Bar>"
		);
	}

	#[test]
	fn should_truncate_multiple_generic_generics() {
		assert_eq!(
			truncate_type_name("foo::bar::Type<foo::bar::Foo, foo::bar::Bar<foo::Zed>>"),
			"Type<Foo, Bar<Zed>>"
		);
	}

	#[test]
	fn should_truncate_the_generic() {
		assert_eq!(
			truncate_type_name("foo::bar::Type<foo::bar::Zed>"),
			"Type<Zed>"
		);
	}

	#[test]
	fn should_truncate_generic_generics() {
		assert_eq!(
			truncate_type_name("foo::bar::Type<foo::bar::Zed<foo::bar::Yon>>"),
			"Type<Zed<Yon>>"
		);
	}
}
