use std::fs;
use std::io::Write;

pub struct FileData {
    month: String,
    year: String,
    content: String,
}
impl FileData {
    pub fn get_content(&self) -> &str {
        &self.content.as_str()
    }

    pub fn new_from_file(file_path: &str) -> Result<FileData, &str> {
        if let Some(date) = file_path.split(".").next() {
            if let [month, year] = date.split("-").collect::<Vec<&str>>().as_slice() {
                let content = match fs::read_to_string(file_path) {
                    Ok(c) => c,
                    Err(_) => return Err("failed to ingest file error: {}"),
                };

                Ok(FileData {
                    month: String::from(*month),
                    year: String::from(*year),
                    content: String::from(content),
                })
            } else {
                Err("invalid file name expected: mm-yyyy.txt")
            }
        } else {
            Err("invalid file type")
        }
    }

    pub fn write_to_file(self) -> Result<(), &'static str> {
        let path = format!("{}-{}.txt", self.month, self.year);
        let mut file = match fs::OpenOptions::new().append(true).open(path) {
            Ok(file) => file,
            Err(e) => return Err("error opening file"),
        };

        match writeln!(file, "{}", self.content) {
            Ok(_) => {
                println!("write completed");
                Ok(())
            }

            Err(e) => return Err("failed to write file"),
        } //self.content);
    }
}
