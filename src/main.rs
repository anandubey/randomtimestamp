use chrono::{
    NaiveDate,
    NaiveDateTime
};
use rand::Rng;
use clap::Parser;


#[derive(Default, Parser, Debug)]
#[clap(author="Anand Dubey <dubey.anandkr@gmail.com>", version, about)]
///Generate random Timestamp
struct Arguments {
    #[clap(short = 's', long = "sdate", validator = validate_cli_date_arg)]
    start_date: String,

    #[clap(short = 'e', long = "edate", validator = validate_cli_date_arg)]
    end_date: String
}

fn validate_cli_date_arg(start_date: &str) -> Result<(), String>{
    if start_date.len() != 10 {
        Err(String::from(
            "Date format should be YYYY-MM-DD"
        ))
    } else {
        Ok(())
    }
}


fn main() {
    let args = Arguments::parse();
    println!("{:?}", args);
    let (start_date, end_date) = (
        NaiveDate::parse_from_str(args.start_date.as_str() , "%Y-%m-%d").unwrap(),
        NaiveDate::parse_from_str(args.end_date.as_str() , "%Y-%m-%d").unwrap()
    );
    let random_date = random_date(start_date, end_date);
    println!("[+] Random date between {start_date} and {end_date}  is  {random_date}")
}


fn random_date(start_date: NaiveDate, end_date: NaiveDate) -> NaiveDate {
    validate_date_args(start_date, end_date);

    let epoch_start_date = generate_epoch(start_date);
    let epoch_end_date = generate_epoch(end_date);

    let rand_epoch_val = rand::thread_rng().gen_range(epoch_start_date..=epoch_end_date);
    NaiveDateTime::from_timestamp(rand_epoch_val, 0).date()
}

fn validate_date_args(start_date: NaiveDate, end_date: NaiveDate) -> bool {
    // TODO: Improve the validate function to assert the correctness of the date arguments
    let mut validation_status = true;
    if start_date > end_date {
        println!("[*] Start Date should be less than end date");
        validation_status = false;
    }
    validation_status
}

fn generate_epoch(date: NaiveDate) -> i64 {
    date.and_hms_milli(0, 0, 0, 0).timestamp()
}

