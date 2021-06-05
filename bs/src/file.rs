use std::env::current_dir;
use std::string::String;
use std::fs::File;
use std::io::prelude::*;


pub fn open_file(path: &str) -> String {
    
    let cwd = match current_dir() {
        Err(why) => panic!("couldn't gett the current dir: {}",  why),
        Ok(file) => file,
    };

    // Create a path to the desired file
    let path = cwd.join(path);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

    s
}


#[cfg(test)]
mod tests {
    use super::open_file;

    #[test]
    fn it_reads_a_file_from_the_disk() {
        let contents = open_file("assets/hello_world.ts");
        assert_eq!("console.log(\"hello world\");\n", contents);
    }
}
