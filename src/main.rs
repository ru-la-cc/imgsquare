use std::process::ExitCode;
use imgsquare::params::Params;

fn main() -> ExitCode {
    println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    match Params::get_params() {
        Ok(param) => {
            println!("Source: {}", param.imagepath);
            println!("Position: {}", param.position);
            println!("Size: {}", param.size);
            println!("Expand: {}", param.expand);
            if let Err(e) = imgsquare::run(&param) {
                eprint!("{}", e);
                return ExitCode::FAILURE
            }
            println!("Done.");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{}", e);
            ExitCode::FAILURE
        }
    }
}
