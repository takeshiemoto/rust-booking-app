use crate::models::Patient;
use chrono::{DateTime, NaiveDate};
use std::io;

pub fn run() {
    println!("お名前を入力してください");
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.lines().collect::<String>();

    println!("生年月日を入力してください（例: 2019/05/24））");
    let mut birthday = String::new();
    io::stdin().read_line(&mut birthday).unwrap();
    let birthday = birthday.lines().collect::<String>();
    let birthday = NaiveDate::parse_from_str(&birthday, "%Y/%m/%d").unwrap();

    let patient = Patient::new(name, birthday);

    println!("{:?}", patient);
}
