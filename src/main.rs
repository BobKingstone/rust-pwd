use std::io::prelude::*;
use std::fs::OpenOptions;
use std::io;
use std::fs;


/*
 *
 * creates/writes to file of given name returning ok/err
 *
 * param &str
 * returns Result
 *
 */
fn write_to_file(filename: &str) -> io::Result<()> {
     let name = filename.to_string() + ".txt";
     let mut file = try!(OpenOptions::new()
         .create(true) //create if does not exist
         .write(true)  //has write to file permission
         .append(true) //append buffer to end of file
         .open(name)); //open file with name 'name'
         

     try!(writeln!(file, "Bob was here twice"));
     
     return Ok(());
}


fn read_from_file(filename: &str) -> io::Result<()> {
    let name = filename.to_string() + ".txt";
    let mut file = try!(OpenOptions::new()
                        .create(true)
                        .read(true)
                        .open(name));

    let mut file_contents = String::new();
    
    try!(Read::read_to_string(&mut file, &mut file_contents));

    println!("{}",file_contents);

    return Ok(());
}

fn delete_file(filename: &str) -> io::Result<()> {
    let name = filename.to_string() + ".txt";

    try!(fs::remove_file(name));

    return Ok(());
}

/*
 *
 * Helper function to print error message
 *
 */
fn print_error(error: &str) {
    println!("There was an error {} to the file", error);
}

fn main() {
    //create/open file
    //list choices
    //  a) write string
    //  b) read from file
    //  c) delete file

    let filename = "name";

    let write = write_to_file(&filename);

    //check for errors when writing to file
    if write.is_err() {
        print_error("writing");
    }

    let read = read_from_file(&filename);

    if read.is_err() {
        print_error("reading");
    }

    let _delete = delete_file(filename);

}
