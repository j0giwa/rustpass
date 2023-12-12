use clap::{App, Arg};
use rand::{distributions::Alphanumeric, Rng};

fn passwd_gen(length: u32) -> String {
    let passwd: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length.try_into().unwrap())
        .map(char::from)
        .collect();
    return passwd;
}

fn main() {

    let mut passwd_length: u32 = 8;

    let matches = App::new("Rustpass")
        .version("1.0")
        .author("j0giwa")
        .about("A simple CLI app with argument parsing")
        .arg(
            Arg::with_name("length")
                .short("l")
                .long("length")
                .value_name("LENGTH")
                .help("Sets the length")
                .takes_value(true),
        )
        .get_matches();
    
    // Extract the value of "length" argument and override the default value
    if let Some(length_str) = matches.value_of("length") {
        // Parse the string into an integer
        if let Ok(length) = length_str.parse::<u32>() {
            // Override the default value
            passwd_length = length;
        } else {
            eprintln!("Error: Unable to parse length argument as an integer");
            return;
        }
    }

     println!("{}", passwd_gen(passwd_length));
}
