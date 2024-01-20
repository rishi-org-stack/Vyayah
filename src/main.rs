use std::{fs::File, error::Error};
pub mod expense;
struct Connection<'a> {
    data: Vec<&'a str>,
}


#[derive(Debug)]
struct ErrFileNotFound{
    msg: String,
}
impl Error for ErrFileNotFound{
    
}

impl ErrFileNotFound{
    fn Display(){

    }
}
struct ErrorConnection{

}

impl<'a> Connection<'a> {
    fn new(file_path: &str) -> Connection<'a> {
        if let File::open(file_path);
    }
}

fn main() {
    
}
