use std::io::prelude::*;
use std::fs::OpenOptions;
use std::io;

fn write_to_file(filename: &str) -> io::Result<()> {
     let name = filename.to_string() + ".txt";
     let mut file = try!(OpenOptions::new()
         .write(true)  //has write to file permission
         .append(true) //append buffer to end of file
         .open(name)); //open file with name 'name'
         

     try!(writeln!(file, "Bob was here twice\n"));
     
     return Ok(());
}

fn main() {
    //create/open file
    //list choices
    //  a) write string
    //  b) delete string

    let filename = "name";

    let x = write_to_file(&filename);

    //check for errors when writing to file
    if x.is_err() {
        println!("There was an error writing to the file");
    }

}
