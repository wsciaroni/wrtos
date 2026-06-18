use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Tell Cargo to rerun if schedule.toml changes
    println!("cargo:rerun-if-changed=schedule.toml");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("schedule.rs");

    let toml_content = fs::read_to_string("schedule.toml")
        .expect("Could not read schedule.toml");

    let config = wrtos_codegen::parse_config(&toml_content)
        .expect("Failed to parse schedule.toml");

    wrtos_codegen::validate_config(&config)
        .expect("Invalid schedule configuration");

    let generated_code = wrtos_codegen::generate_schedule_code(&config);

    fs::write(&dest_path, generated_code)
        .expect("Could not write generated schedule.rs");
}
