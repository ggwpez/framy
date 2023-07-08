use std::{
	env,
	fs::File,
	io::{BufWriter, Write},
	path::Path,
};

fn main() {
	let out_path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
	// List all files in templates/
	let files = std::fs::read_dir("templates")
		.unwrap()
		.map(|res| res.map(|e| e.path()))
		.collect::<Result<Vec<_>, std::io::Error>>()
		.unwrap();

	for file in &files {
		println!("cargo:rerun-if-changed={}", file.display());
	}

	// Create a virtual filesystem map
	let mut map = phf_codegen::Map::new();
	for file in files.iter() {
		map.entry(
			file.file_name().unwrap().to_str().unwrap(),
			&format!("r#\"{}\"#", std::fs::read_to_string(file).unwrap().as_str()),
		);
	}
	let map = map.build();
	let mut file = BufWriter::new(File::create(out_path).unwrap());

	writeln!(
		&mut file,
		"pub static TEMPLATES: phf::Map<&'static str, &'static str> = \n{};\n",
		map
	)
	.unwrap();
}
