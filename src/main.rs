use std::env;
use std::fs::create_dir_all;
use std::process;

type Args = Vec<String>;

fn create_path(base: &str, name: &str) -> String {
    format!("{}/{}", base, name)
}

fn generate_dirs(base_path: &str) -> Result<(), std::io::Error> {
    let paths = vec![
        "actions",
        "components",
        "providers",
        "lib",
        "animations",
        "hooks",
        "store",
    ];
    let path_for_generate = paths.iter().map(|path| create_path(base_path, path));
    for path in path_for_generate {
        println!("Generated: {}", &path);
        create_dir_all(path)?;
    }

    println!("Directories generation completed successfully!");
    Ok(())
}
fn main() {
    let args: Args = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: create-react-setting | cres <base-path>");
        process::exit(1);
    }
    let base_path = &args[1];
    if let Err(e) = generate_dirs(base_path) {
        eprintln!("Error creating directory: {}", e);
        process::exit(1);
    }
}
