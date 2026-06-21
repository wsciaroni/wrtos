fn main() {
    println!("cargo:rerun-if-changed=schedule.toml");
    let toml = std::fs::read_to_string("schedule.toml").unwrap();
    let config = wrtos_codegen::parse_config(&toml).unwrap();
    wrtos_codegen::validate_config(&config).unwrap();
    
    let code = wrtos_codegen::generate_schedule_code(&config);
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest = std::path::Path::new(&out_dir).join("schedule.rs");
    std::fs::write(dest, code).unwrap();
}
