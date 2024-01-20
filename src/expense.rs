use crate::db::file_data::FileData;

#[derive(Debug)]
struct Spending {
    amt: u32,
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
pub struct Sheet {
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

impl Sheet {
    fn new_sheet(file_data: &FileData) -> Sheet {
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

        let content = file_data.get_content();
        for line in content.split("\n") {
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
    fn modify_saving(&mut self, spending_amt: u32) {
        self.saving = (self.earning - spending_amt) as i32;
    }

    fn modify_over_spent(&mut self, spending_amt: u32) {
        self.over_spent = (spending_amt - self.earning) as f32;
    }

    fn add_spending(&mut self, spending: Spending) -> String {
        let body = [
            format!("{}:{}\n", LOCATION, spending.location).as_str(),
            format!("{}:{}\n", AMT, spending.amt).as_str(),
            format!("{}:{}\n", WHY, spending.why).as_str(),
            format!("{}\n", END).as_str(),
        ]
        .join("");
        body
    }
}
