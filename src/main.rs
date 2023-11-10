use std::fs;

#[derive(Debug)]
struct Spending {
    amt: i32,
    location: String,
    why: String,
}

enum Column {
    Earning,
    Location,
    Amt,
    Why,
    End,
    Saving,
    OverSpent,
    Thought,
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
const Earning: &str = "Earning";
const Location: &str = "Location";
const Amt: &str = "Amt";
const Why: &str = "Why";
const End: &str = "End";
const Saving: &str = "Saving";
const OverSpent: &str = "OverSpent";
const Thought: &str = "Thought";
impl Sheet {
    fn newSheet(file_path: &str) -> Result<Sheet, &str> {
        let content = match fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(e) => return Err("failed to ingest file"),
        };

        let mut sheet: Sheet = Sheet {
            month: 1,
            year: 2,
            earning: 1,
            spendings: vec![],
            saving: 1,
            over_spent: 1 as f32,
            thought: String::from("okish month"),
        };

        let mut spending: Spending = Spending {
            amt: 0,
            location: String::from("java"),
            why: String::from("fealt"),
        };

        for line in content.split("\n") {
            let row: Vec<_> = line.split(":").map(|v| v.trim()).collect();
            match row.len() {
                0 => {
                    println!("read EOF")
                }

                _ => match row[0] {
                    Earning => sheet.earning = row[1].parse().unwrap(),
                    OverSpent => sheet.over_spent = row[1].parse().unwrap(),
                    Saving => sheet.saving = row[1].parse().unwrap(),
                    Thought => sheet.thought = String::from(row[1]),
                    Location => spending.location = String::from(row[1]),
                    Amt => spending.amt = row[1].parse().unwrap(),
                    Why => spending.why = String::from(row[1]),
                    End => {
                        sheet.spendings.push(spending);
                        spending = Spending {
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

        Ok(sheet)
    }
}
fn main() {
    // let content = fs::read_to_string("11-23.txt").expect("file not found");

    // let mut earning: u32 = 1;
    // for line in content.split("\n") {
    //     let row: Vec<_> = line.split(":").map(|v| v.trim()).collect();
    //     match row.len() {
    //         0 => {
    //             println!("read EOF")
    //         }

    //         _ => {
    //             println!("{:?}", row)
    //         }
    //     }
    // }
    // println!("vyayah");
    let sheet = Sheet::newSheet("11-23.txt");
    match sheet {
        Ok(s) => println!("{:?}", s),
        Err(e) => println!("{:?}", e),
    }
}
