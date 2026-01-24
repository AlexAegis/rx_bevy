use clap::{Parser, Subcommand};
use mdbook_preprocessor::book::{Book, BookItem};
use mdbook_preprocessor::errors::Result;
use mdbook_preprocessor::{Preprocessor, PreprocessorContext};
use regex::{Captures, Regex};
use std::io;
use std::process;

/// An mdbook preprocessor that rewrites GitHub crate links to internal book links.
///
/// This is useful when readme files are included in the book via `{{#include ...}}`,
/// as links in those readmes typically point to GitHub but should point to other
/// pages within the book when rendered.
#[derive(Parser)]
#[command(name = "mdbook-crate-links")]
#[command(about = "An mdbook preprocessor that rewrites GitHub crate links to internal book links")]
struct Cli {
	#[command(subcommand)]
	command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
	/// Check if the preprocessor supports a renderer
	Supports { renderer: String },
}

fn main() {
	let cli = Cli::parse();

	match cli.command {
		Some(Commands::Supports { renderer }) => {
			if renderer == "html" || renderer == "markdown" {
				process::exit(0);
			} else {
				process::exit(1);
			}
		}
		None => {
			if let Err(e) = handle_preprocessing() {
				eprintln!("Error during preprocessing: {}", e);
				process::exit(1);
			}
		}
	}
}

fn handle_preprocessing() -> Result<()> {
	let (ctx, book) = mdbook_preprocessor::parse_input(io::stdin())?;

	let processed_book = CrateLinkPreprocessor.run(&ctx, book)?;

	serde_json::to_writer(io::stdout(), &processed_book)?;

	Ok(())
}

struct CrateLinkPreprocessor;

impl Preprocessor for CrateLinkPreprocessor {
	fn name(&self) -> &str {
		"crate-links"
	}

	fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
		book.for_each_mut(|item| {
			if let BookItem::Chapter(chapter) = item
				&& let Some(path) = &chapter.path
			{
				let depth = path.components().count().saturating_sub(1);
				let prefix = if depth == 0 {
					String::new()
				} else {
					"../".repeat(depth)
				};

				chapter.content = rewrite_crate_links(&chapter.content, &prefix);
			}
		});

		Ok(book)
	}

	fn supports_renderer(&self, renderer: &str) -> Result<bool> {
		Ok(renderer == "html" || renderer == "markdown")
	}
}

/// Rewrites GitHub crate links to internal book links.
///
/// Matches patterns like:
/// - `https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_map`
/// - `https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_just`
/// - `https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_bevy_observable_event`
///
/// And rewrites them to relative mdbook links like:
/// - `operator/map.md`
/// - `observable/just.md`
/// - `observable_bevy/event.md`
fn rewrite_crate_links(content: &str, prefix: &str) -> String {
	let github_link_pattern = Regex::new(
		r"https://github\.com/AlexAegis/rx_bevy/tree/[^/]+/crates/(rx_(?:core|bevy)_([a-z]+)_([a-z_]+))",
	)
	.unwrap();

	github_link_pattern
		.replace_all(content, |caps: &Captures| {
			let package_prefix = if caps[1].starts_with("rx_bevy_") {
				"rx_bevy"
			} else {
				"rx_core"
			};
			let category = &caps[2];
			let name = &caps[3];

			let book_path = match (package_prefix, category) {
				("rx_core", "observable") => Some(format!("{}observable/{}.md", prefix, name)),
				("rx_core", "operator") => Some(format!("{}operator/{}.md", prefix, name)),
				("rx_core", "observer") => Some(format!("{}observer/{}.md", prefix, name)),
				("rx_core", "subject") => Some(format!("{}subject/{}.md", prefix, name)),
				("rx_core", "scheduler") => Some(format!("{}scheduler/{}.md", prefix, name)),
				("rx_bevy", "observable") => Some(format!("{}observable_bevy/{}.md", prefix, name)),
				_ => None,
			};

			match book_path {
				Some(path) => path,
				None => caps[0].to_string(),
			}
		})
		.to_string()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_rewrite_operator_link() {
		let content = r#"See [MapOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_map) for details."#;
		let result = rewrite_crate_links(content, "../");
		assert_eq!(
			result,
			r#"See [MapOperator](../operator/map.md) for details."#
		);
	}

	#[test]
	fn test_rewrite_observable_link() {
		let content = r#"- [JustObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_just)"#;
		let result = rewrite_crate_links(content, "");
		assert_eq!(result, r#"- [JustObservable](observable/just.md)"#);
	}

	#[test]
	fn test_rewrite_subject_link() {
		let content = r#"[BehaviorSubject](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_subject_behavior)"#;
		let result = rewrite_crate_links(content, "../");
		assert_eq!(result, r#"[BehaviorSubject](../subject/behavior.md)"#);
	}

	#[test]
	fn test_rewrite_bevy_observable_link() {
		let content = r#"[EventObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_bevy_observable_event)"#;
		let result = rewrite_crate_links(content, "");
		assert_eq!(result, r#"[EventObservable](observable_bevy/event.md)"#);
	}

	#[test]
	fn test_rewrite_multiple_links() {
		let content = r#"
## See Also

- [PublishSubject](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_subject_publish) -
  Forwards observed signals.
- [AsyncSubject](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_subject_async) -
  Reduces observed values.
"#;
		let result = rewrite_crate_links(content, "../");
		assert!(result.contains("(../subject/publish.md)"));
		assert!(result.contains("(../subject/async.md)"));
	}

	#[test]
	fn test_preserves_non_crate_links() {
		let content = r#"
[Documentation](https://docs.rs/rx_bevy)
[CI](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
"#;
		let result = rewrite_crate_links(content, "../");
		assert_eq!(result, content);
	}

	#[test]
	fn test_compound_operator_names() {
		let content = r#"[FilterMapOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_filter_map)"#;
		let result = rewrite_crate_links(content, "");
		assert_eq!(result, r#"[FilterMapOperator](operator/filter_map.md)"#);
	}
}
