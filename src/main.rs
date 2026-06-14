use std::{fs, fs::File};
use std::path::{Path, PathBuf};
use std::env;
use std::io::Read;
use std::ptr::null;

static NG_DIR: &str = "./.ng/";
static HOME_DIR: &str = "./";
static commit_amnt: u8 = 0;

fn help_msg() -> String {
    return { format!(
        "Default usage: \"cargo run [arguments]\"\n
        Arguments:
        \tinit = initialize a new \'git\' repository on your local machine ('{}')
        \tcommit = swap the last commited file with your new saved one
        \tkill = kills your \'git\' repository. cleans everything on the process
        ",
        NG_DIR
    )}
}

// ===============================

// THERE'S PROCESS TO THE "commit" op:
// - 1: READ CURRENT COMMIT TEMPORARY FILE
// - 2: COMPARE THE TWO BUFFERs THAT THEY MAKE
// - 3: THEN SWAP THE CURRENT TEMP FILE TO THE NEW ONE

fn iterate_dir(path: &PathBuf, op: Option<&str>) -> Result<(), ()> {
    let mut _read: bool = false;
    let mut _write: bool = false;
    let mut _copy: bool = false;
    // let op = op.unwrap_or("r");
    match op {
        Some(_c) => match _c {
                "r" => _read = true, // READ-ONLY
                "w" => _write = true, // WRITE AND READ
                "c" => _copy = true, // COPY FILES -> SPECIAL FOR INITIALIZING REPO AND NO MORE
                _ => {}
        },
        None => {
            eprintln!("Dev-focused error: missing walk_dir_files()'s option argument");
            std::process::exit(1);
        }
    };

    // FOR EACH FILE, WE'LL DO THE PROCESS DOCUMENTED ABOVE
    for entry in fs::read_dir(path).unwrap() {
        let entry = match entry {
            Ok(f) => f,
            Err(e) => {
                std::process::exit(1);
            }
        };


        let path = entry.path();
        if path.is_file() {
            println!("\n==\n{}", file_read(&mut File::open(path).unwrap()));
        } else { // should be a directory and we'd open that as well
            println!("{}", path.clone().into_os_string().into_string().unwrap());

            // IN CASE OF RUST PROJECTS (cargo), AND ALSO GIT INITIALIZED LOCAL REPOSITORIES
            // WE BLACKLIST THEM SINCE THEY HAVE A LOT OF FILES
            match path.file_name() {
                Some(name) if name == "target" => {},
                Some(name) if name == ".git" => {},
                _ => { iterate_dir(&path, Some("r")); }
            };
        }
    }

    Ok(())
}

fn commit() -> Result<(), ()> {
    // CHECK EVERY FILE POSSIBLE FOR DIFFERENCES
    // WE SHOULD NOW KEEP THEM SAVED SOMEWHERE AROUND ./.ng, JUST LIKE
    // RECREATIONS OF THE CURRENT PROJECT DIRECTORY
    iterate_dir(&PathBuf::from("."), Some("w"));

    /*
        ONE THING WE CAN DO TO SAVE THEM FILES WITHOUT TAKING A LOT OF MEMORY
        IS SAVING EACH BYTE TOGETHER

        FOR EXAMPLE:
        "abc" => WE HAVE TO SOMEHOW GET A NUMBER FROM THEM 3 BYTES, WHICH WILL BE QUITE UNIQUE
        say that for example, if we multiply by the index they're placed at the line:
        65 * 1 + 66 * 2 + 67 * 3 = 398
        we save 398 as the curnt line value
        in case it's different, then someone messed with this specific line, those specific 3 chars
    */
    Ok(())
    // file_read(); // FOR THE CURRENT SAVED FILE
    // file_read(); // FOR THE CURRENT CHANGED FILE
    // cmp_print_files(); // COMPARE THE TWO GENERATED BUFFERS
    // commit_file_swap(); // COMMIT THE FILE SWAP AND ALSO PRINT THE DIFFERENCES
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

fn commit_file_swap() {
    // parse that the commit temp file is indeed existant
    // the oldf_file ( last commit ) can be shadowed, no problem
    // since the first check is for the path, we use that as well to check
    // if the path is leading somewhere
    /*let oldf_file = match File::open(commit_path) {
        Ok(f) => f, // file indeed exist
    };*/

    // String buffer for first file ( old one )
}

fn initialize_repo(path: &PathBuf) -> Result<(), ()> {
    fs::create_dir(NG_DIR);
    iterate_dir(HOME_DIR, "c");
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // first is "cargo run", second (1) to inf is the indexed arguments => 1 to ..
    let op = match args.get(1) {
        Some(f) => match f.as_str() {
            "init" => {
                // WE HAVE TO HAVE A DIRECTORY FOR STORING TEMPORARY COMMIT FILES
                // AND PROBABLY OTHER STUFF WE'D NEED LATER ON
                initialize_repo(&PathBuf::from(NG_DIR));
            },
            "commit" => {
                commit();
            },
            "kill" => { // KILLS CURRENT NG REPOSITORY
                fs::remove_dir(NG_DIR); // removing the main directory kills everything
            }
            _ => ()
        },
        None => {
            eprintln!("Missing arguments:, {}", help_msg());
            std::process::exit(1);
        }
    };
    
}
