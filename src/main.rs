use std::fs;
use std::fs::File;

#[derive(Debug)]
struct Spending {
    amt: i32,
    location: String,
    why: String,
}

enum Column {
    EARNING,
    LOCATION,
    AMT,
    WHY,
    END,
    SAVING,
    OVER_SPENT,
    THOUGHT,
}
#[derive(Debug)]
struct Sheet {
    month: u8,
    year: u32,
    earning: u32,
    spendings: Vec<Spending>,
    saving: i32,
    over_spent: f32,
    thought: String,
}
const EARNING: &str = "Earning";
const LOCATION: &str = "Location";
const AMT: &str = "Amt";
const WHY: &str = "Why";
const END: &str = "End";
const SAVING: &str = "Saving";
const OVER_SPENT: &str = "OverSpent";
const THOUGHT: &str = "Thought";

struct FileData {
    month: String,
    year: String,
    content: String,
}
impl FileData {
    fn new_file_data(file_path: &str) -> Result<FileData, &str> {
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
}
impl Sheet {
    fn new_sheet(file_data: FileData) -> Sheet {
        let mut sheet: Sheet = Sheet {
            month: 1,
            year: 2,
            earning: 1,
            spendings: vec![],
            saving: 1,
            over_spent: 1 as f32,
            thought: String::from("okish month"),
        };

        let mut spendings: Spending = Spending {
            amt: 0,
            location: String::from("java"),
            why: String::from("fealt"),
        };

        for line in file_data.content.split("\n") {
            let row: Vec<_> = line.split(":").map(|v| v.trim()).collect();
            match row.len() {
                0 => {
                    println!("read EOF")
                }

                _ => match row[0] {
                    EARNING => sheet.earning = row[1].parse().unwrap(),
                    OVER_SPENT => sheet.over_spent = row[1].parse().unwrap(),
                    SAVING => sheet.saving = row[1].parse().unwrap(),
                    THOUGHT => sheet.thought = String::from(row[1]),
                    LOCATION => spendings.location = String::from(row[1]),
                    AMT => spendings.amt = row[1].parse().unwrap(),
                    WHY => spendings.why = String::from(row[1]),
                    END => {
                        sheet.spendings.push(spendings);
                        spendings = Spending {
                            amt: 0,
                            why: String::from(""),
                            location: String::from(""),
                        }
                    }
                    _ => {
                        println!("unknown key")
                    }
                },
            }
        }

        sheet
    }

    fn encode(self) -> String {
        let header = [format!("{}:{}\n", EARNING, self.earning).as_str()].join("");

        let body = self.spendings.iter().fold(String::new(), |acc, spending| {
            let body = [
                format!("{}:{}\n", LOCATION, spending.location).as_str(),
                format!("{}:{}\n", AMT, spending.amt).as_str(),
                format!("{}:{}\n", WHY, spending.why).as_str(),
                format!("{}\n", END).as_str(),
            ]
            .join("");

            format!("{}{}", acc, body)
        });

        let footer = [
            format!("{}:{}\n", SAVING, self.saving).as_str(),
            format!("{}:{}\n", OVER_SPENT, self.over_spent).as_str(),
            format!("{}:{}", THOUGHT, self.thought).as_str(),
        ]
        .join("");

        format!("{}{}{}", header, body, footer)
    }
}
fn main() {
    let fs_data = FileData::new_file_data("11-23.txt").expect("");
    let sheet = Sheet::new_sheet(fs_data);

    println!("{:?}", sheet);

    let data = sheet.encode();
    // let mut file = File::create("12-2023.txt").expect("unable to write file");
    // writeln!(file, "{}", data);
    fs::write("12-2023.txt", data).expect("unable to write file")
}
