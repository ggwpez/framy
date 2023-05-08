mod context;
mod presets;

use context::*;

use convert_case::{Case, Casing};
use inquire::{Confirm, Select, Text};
use std::path::{Path, PathBuf};
use tera::Tera;
use tera_text_filters::{camel_case, snake_case};

mod templates {
	include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	env_logger::init();

	let mut tera = Tera::default();
	for name in templates::TEMPLATES.keys() {
		tera.add_raw_template(name, templates::TEMPLATES[name])?;
		println!("Loaded template: {}", name);
	}

	tera.register_filter("camel_case", camel_case);
	tera.register_filter("snake_case", snake_case);

	let (mod_name, pallet_name) = prompt_name()?;
	let path = prompt_path(&folder_name(&mod_name))?;
	let options: Vec<&str> = vec!["Parity/Substrate", "None"];
	let ans = Select::new("Preset:", options).prompt()?;

	let (cargo, pallet) = match ans {
		"Parity/Substrate" => (presets::substrate::cargo(), presets::substrate::pallet()),
		"None" => (
			presets::basic::cargo()
				.author("TODO author".into())
				.repository("TODO repository".into())
				.homepage("TODO homepage".into())
				.license(prompt_license()?),
			presets::basic::pallet().license_header("// TODO license_header".into()),
		),
		e => unreachable!("Invalid preset: {}", e),
	};

	let pallet = pallet.name(pallet_name.clone()).build();
	let description = Text::new("Description:").prompt()?;
	let cargo = cargo.description(description).module(mod_name.clone()).build();

	let context = presets::basic::context().pallet(pallet).cargo(cargo).build();

	let root_dir = PathBuf::from(path);
	// check if the directory exists
	if root_dir.exists() {
		log::warn!("Directory '{}' already exists!", root_dir.display());
	}
	let src = root_dir.join("src");
	std::fs::create_dir_all(&src)?;

	let root_files = vec![("Cargo.tera", "Cargo.toml"), ("README.tera", "README.md")];
	let src_files = vec![
		("benchmarking_v2.tera", "benchmarking.rs"),
		("lib.tera", "lib.rs"),
		("mock.tera", "mock.rs"),
		("tests.tera", "tests.rs"),
		("weights.tera", "weights.rs"),
	];

	for (template, file) in root_files {
		render_to_file(&tera, template, &context, &root_dir.join(file))?;
	}
	for (template, file) in src_files {
		render_to_file(&tera, template, &context, &src.join(file))?;
	}

	println!("🎉 Try out your pallet with: cd {} && cargo test --all-features", root_dir.display());

	Ok(())
}

fn render_to_file(
	tera: &Tera,
	template: &str,
	context: &Context,
	file: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
	let rendered = tera.render(template, &tera::Context::from_serialize(&context)?)?;
	std::fs::write(file, rendered)?;
	println!("+ {}", file.display());
	Ok(())
}

fn prompt_name() -> Result<(String, String), Box<dyn std::error::Error>> {
	let mod_name = loop {
		let name = Text::new("Module Name:").with_initial_value("pallet-").prompt()?;
		if name.starts_with("pallet-") {
			break name
		} else {
			let ans = Confirm::new("Are you sure?")
				.with_default(false)
				.with_help_message("Pallet names usually start with 'pallet-'.")
				.prompt();
			if matches!(ans, Ok(true)) {
				break name
			}
		}
	};

	Ok((
		mod_name.clone(),
		mod_name.strip_prefix("pallet-").unwrap().to_string().to_case(Case::Camel),
	))
}

fn prompt_path(initial: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
	loop {
		let path = Text::new("Folder:").with_initial_value(initial).prompt()?;
		let path = PathBuf::from(path);
		if !path.exists() {
			break Ok(path)
		} else {
			let ans = Confirm::new("Are you sure?")
				.with_default(false)
				.with_help_message("The folder already exists.")
				.prompt();
			if matches!(ans, Ok(true)) {
				break Ok(path)
			}
		}
	}
}

fn prompt_license() -> Result<String, Box<dyn std::error::Error>> {
	let options: Vec<&str> = vec!["Apache-2.0", "MIT", "GPL-3.0", "Custom"];
	let ans = Select::new("License:", options).prompt()?;
	if ans == "Custom" {
		let license = Text::new("License:").prompt()?;
		Ok(license)
	} else {
		Ok(ans.to_string())
	}
}
