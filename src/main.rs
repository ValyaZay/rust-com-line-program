use std::env;
use std::process;
use com_line_program::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {//will return either ok or err - 2 possible outcomes
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("Searching for '{0}' in file {1}: \n", config.query, config.file_path);

    if let Err(e) = com_line_program::run(config) { //'run' doesn't have a value to unwrap, so 'unwrap_or_else' cannot be used here
        println!("Application error: {e}");
        process::exit(1);
    }

    
    /*
    let config = Config::build(&args);
    match config {
        Ok(config) => {
            println!("Searching for {}", config.query);
            println!("In file {}", config.file_path);

            let contents = fs::read_to_string(config.file_path).expect("Should have been able to read the file");
            println!("With contents: \n{contents}");
        }
        Err(config) => {
            println!("{config}");
        }
    }
    */


   
}