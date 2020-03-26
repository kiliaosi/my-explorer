use std::fs::File;
use std::io::ErrorKind;

pub fn calc_error()->Result<(),()>{
    let file_handle = File::open("target.txt");
    Ok(())
}