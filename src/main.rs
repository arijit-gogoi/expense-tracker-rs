use chrono::{Datelike, Local, NaiveDate};
use clap::{Arg, Command};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{
    fs::{File, OpenOptions},
    io::{self},
    path::Path,
};

// Define Expense struct
#[derive(Serialize, Deserialize, Debug)]
struct Expense {
    date: NaiveDate,
    category: String,
    amount: f64,
    description: String,
}

// Define the structure of the JSON data file
#[derive(Serialize, Deserialize, Debug)]
struct ExpenseTracker {
    expenses: Vec<Expense>,
}

impl ExpenseTracker {
    fn new() -> ExpenseTracker {
        ExpenseTracker {
            expenses: Vec::new(),
        }
    }

    fn add_expense(&mut self, expense: Expense) {
        self.expenses.push(expense);
    }

    fn delete_expense(&mut self, row_number: usize) {
        if self.expenses.is_empty() {
            println!("No expenses found.");
        }
        self.expenses.remove(row_number - 1);
    }

    fn total_expenses(&self, filter_by: &str) -> f64 {
        let mut sum = 0 as f64;

        if filter_by == "all" {
            return self.expenses.iter().map(|e| e.amount).sum();
        }
        for expense in self.expenses.iter() {
            if expense.category == *filter_by {
                sum = sum + expense.amount;
            }
        }
        sum
    }
    fn summary_by_date(&self, date: NaiveDate) -> f64 {
        let mut sum = 0 as f64;
        for expense in self.expenses.iter() {
            if expense.date == date {
                sum = sum + expense.amount;
            }
        }
        sum
    }
    fn summary_by_month(&self, month: u8) -> f64 {
        let mut sum = 0 as f64;
        for expense in self.expenses.iter() {
            if expense.date.month() as u8 == month {
                sum = sum + expense.amount;
            }
        }
        sum
    }

    fn save_to_json(&self, filename: &str) -> io::Result<()> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(filename)?;
        serde_json::to_writer(file, &self)?;
        Ok(())
    }

    fn load_from_json(filename: &str) -> Result<ExpenseTracker> {
        let path = Path::new(filename);
        if !path.exists() {
            return Ok(ExpenseTracker::new());
        }
        let file = File::open(filename).expect("File should exist");
        let tracker: ExpenseTracker = serde_json::from_reader(file)?;
        Ok(tracker)
    }

    fn print_all_expenses(&self) -> () {
        for (i, expense) in self.expenses.iter().enumerate() {
            println!(
                "{}. Date: {}, Category: {}, Amount: ₹{}, Description: {}",
                i + 1,
                expense.date,
                expense.category,
                expense.amount,
                expense.description,
            );
        }
    }
}

fn main() {
    let matches = Command::new("Expense Tracker CLI")
        .version("1.0")
        .author("Arijit Gogoi <arijit@email.com>")
        .about("Keeps track of your expenses")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Add a new expense")
                .short_flag('a')
                .long_flag("add")
                .visible_alias("a")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("category")
                        .required(true)
                        .short('c')
                        .long("category")
                        .help("The category of the expense"),
                )
                .arg(
                    Arg::new("amount")
                        .required(true)
                        .short('a')
                        .long("amount")
                        .help("The expense amount.")
                        .value_parser(clap::value_parser!(f64)),
                )
                .arg(
                    Arg::new("description")
                        .required(true)
                        .short('d')
                        .long("description")
                        .help("A description for the expense"),
                )
                .arg(
                    Arg::new("date")
                        .required(false)
                        .long("date")
                        .help("The date of expense")
                        .value_parser(clap::value_parser!(String)),
                ),
        )
        .subcommand(
            Command::new("delete")
                .about("Delete an expense by id (row number)")
                .short_flag('d')
                .long_flag("delete")
                .visible_alias("d")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("row_number")
                        .required(true)
                        .help("Delete an expense by id (row number)")
                        .value_parser(clap::value_parser!(usize)),
                ),
        )
        .subcommand(
            Command::new("summary")
                .about("View total expenses")
                .short_flag('t')
                .long_flag("total")
                .visible_alias("t")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("category")
                        .short('c')
                        .long("category")
                        .required(false)
                        .help("Filter by category")
                        .value_parser(clap::value_parser!(String)),
                )
                .arg(
                    Arg::new("date")
                        .short('d')
                        .long("date")
                        .required(false)
                        .help("Filter by date")
                        .value_parser(clap::value_parser!(String)),
                )
                .arg(
                    Arg::new("month")
                        .short('m')
                        .long("month")
                        .required(false)
                        .help("Filter by month")
                        .value_parser(clap::value_parser!(u8)),
                ),
        )
        .subcommand(
            Command::new("list")
                .about("List all expenses")
                .short_flag('l')
                .long_flag("list")
                .visible_alias("l"),
        )
        .get_matches();

    let filename = "expenses.json";
    let mut tracker = ExpenseTracker::load_from_json(filename).unwrap_or_else(|err| {
        eprintln!("Error loading data: {}", err);
        std::process::exit(1);
    });

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let date_string_opt = sub_matches.get_one::<String>("date");
            let date_string = match date_string_opt {
                Some(d) => d,
                None => &Local::now().date_naive().to_string(),
            };
            let date = NaiveDate::parse_from_str(date_string, "%Y-%m-%d")
                .expect("Should be correctly formatted: %Y-%m-%d (for example, 2025-12-31)");
            let category = sub_matches.get_one::<String>("category").unwrap();
            let amount: f64 = *sub_matches
                .try_get_one::<f64>("amount")
                .expect("amount should be a number")
                .expect("amount should be a float");
            let description = sub_matches.get_one::<String>("description").unwrap();

            let expense = Expense {
                date,
                amount,
                category: category.clone(),
                description: description.clone(),
            };

            tracker.add_expense(expense);
            if let Err(err) = tracker.save_to_json(filename) {
                eprintln!("Error saving data: {}", err);
                std::process::exit(1);
            }

            println!("Expense added successfully!\n");
            tracker.print_all_expenses();
        }
        Some(("delete", sub_matches)) => {
            let row_number = sub_matches
                .try_get_one::<usize>("row_number")
                .expect("row number should be a number")
                .expect("some number should be given");

            let length = &tracker.expenses.len();
            if row_number > length {
                eprintln!("Row number should be less than {length}")
            }
            tracker.delete_expense(*row_number);

            if let Err(err) = tracker.save_to_json(filename) {
                eprintln!("Error deleting: {}", err);
                std::process::exit(1);
            }
        }
        Some(("summary", sub_matches)) => {
            if let Some(category) = sub_matches.get_one::<String>("category") {
                println!("Total expenses: ₹{:.2}", tracker.total_expenses(&category));
            }

            if let Some(date) = sub_matches.get_one::<String>("date") {
                let date = NaiveDate::parse_from_str(date, "%Y-%m-%d")
                    .expect("Should be correctly formatted: %Y-%m-%d (for example, 2025-12-31)");
                println!("Expenses by date: ₹{:.2}", tracker.summary_by_date(date));
            } else if let Some(month) = sub_matches.get_one::<u8>("month") {
                println!(
                    "Expenses by month: ₹{:.2}",
                    tracker.summary_by_month(*month)
                );
            } else {
                println!("Total expenses: ₹{:.2}", tracker.total_expenses(&"all"));
            }
        }
        Some(("list", _)) => {
            if tracker.expenses.is_empty() {
                println!("No expenses found.");
            } else {
                tracker.print_all_expenses();
            }
        }
        _ => {
            eprintln!("Invalid command. Use 'add', 'list', 'delete', or 'total'.");
        }
    }
}
