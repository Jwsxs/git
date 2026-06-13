use std::fs;
use std::fs::File;
use std::env;
use std::io::{Read, Write};

fn help_msg() -> String {
    return { format!("{}:\n{}",
        "Default usage: \"cargo run [arguments]\"\n",
        "\nArguments:
            \n\tinit = initialize a new \'git\' repository on your local machine
            \n\tcommit = swap the last commited file with your new saved one
        "
    )}
}

// ===============================

// THERE'S PROCESS TO THE "commit" op:
// - 1: READ CURRENT COMMIT TEMPORARY FILE
// - 2: COMPARE THE TWO BUFFERs THAT THEY MAKE
// - 3: THEN SWAP THE CURRENT TEMP FILE TO THE NEW ONE

fn _commit() {
    // FOR EACH FILE, WE'LL DO THE PROCESS DOCUMENTED ABOVE
    
    file_read();
}

// SETTING EACH FILE AS A READABLE BUFFER
// we'd read each file into a buffer and then compare both
fn file_read(file: &mut File) -> String {
    let mut buf = [0u8; 1]; // just for comparison with EOF
    let mut c_buf = String::new(); // will store the current file char buffer
    
    while file.read(&mut buf).unwrap() > 0 {
        c_buf.push(buf[0] as char);
    }

    return c_buf;
}

// COMPARE AND PRINT DIFFERENCES
fn cmp_print_files() {

}


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut commit_path = String::new();
    
    let mut commit_temp_file = File::open(commit_path)?;
    // first is "cargo run", second (1) to inf is the indexed arguments => 1 to ..
    let op = match args.get(1) {
        Some(f) => match f.as_str() {
            "init" => {
                fs::create_dir("./._git/");
            },
            "commit" => {
                commit_temp_file.write()?;
            },
            _ => {
                eprintln!("Missing arguments:, {}", help_msg());
                std::process::exit(1);
            }
        },
        None => {
            eprintln!("Missing arguments:, {}", help_msg());
            std::process::exit(1);
        }
    }
    
    // parse that the commit temp file is indeed existant
    // the oldf_file ( last commit ) can be shadowed, no problem
    // since the first check is for the path, we use that as well to check
    // if the path is leading somewhere
    let oldf_file = match File::open(commit_path) {
        Ok(f) => f, // file indeed exist
    }
    
    // String buffer for first file ( old one )
}
