use std::io::prelude::*;
use std::fs::OpenOptions;
use std::io;
use std::fs;

extern crate crypto;

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

     let mut record_ref = String::new();
     let mut record_content = String::new();

     println!("enter a password ref:\n");

     io::stdin().read_line(&mut record_ref)
            .ok()
            .expect("Failed to read your selection");

     println!("enter a password:\n");

     io::stdin().read_line(&mut record_content)
            .ok()
            .expect("Failed to read your selection");

     let new_record = format!(":{} - {}",record_ref,record_content);
         
     try!(writeln!(file,"{}",new_record));
     
     return Ok(());
}


/*
 *  read named file, displaying any matching strings
 *  
 *  params String filename
 *
 */
fn read_from_file(filename: &str) -> io::Result<()> {
    
    let name = filename.to_string() + ".txt";
    let mut record = String::new();
    let mut file_contents = String::new();

    let mut file = try!(OpenOptions::new()
                        .create(true)
                        .read(true)
                        .open(name));

    
    try!(Read::read_to_string(&mut file, &mut file_contents));
    
    let mut split: Vec<&str> = file_contents.split(":").collect();

    // get user defined record to find
    println!("Enter record name to retrieve: ");
        
    io::stdin().read_line(&mut record)
            .ok()
            .expect("Failed to read your selection");
    
    let record_substr = &record.trim();

    for s in split {
        if s.contains(record_substr) {
            println!("{}",s.trim());
        }
    }

    return Ok(());
}


/*
 *
 * delete file
 *
 * param String name
 *
 * return Rssult
 *
 */
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
    
    let filename = "name";
    //create/open file
    //list choices
    //  a) write string
    //  b) read from file
    //  c) delete file
    let mut selection = String::new();
    
    loop {
        println!("Please make a selection.\na) Store new password\nb) read stored password\nc) delete password file\nq) Quit PWD");

        io::stdin().read_line(&mut selection)
            .ok()
            .expect("Failed to read your selection");

        //check for matching selection
        match selection.trim() {
            "a" => { 
                let write = write_to_file(&filename);
                if write.is_err() {
                    print_error("writing");
                }
                
            },
            "b" => {
                let read = read_from_file(&filename);
                if read.is_err() {
                    println!("There was an error tryig to read the file, have you created on?");
                }
            }
            "c" => { 
                let _delete = delete_file(filename);
            },
            "q" => {
                println!("Goodbye");
                break;
            },
            _ => selection = "".to_string(),
        };
        //reset selection - will cause compiler warning
        selection = "".to_string(); 
    }
 
}
