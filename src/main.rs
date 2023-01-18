mod models;
mod services;

use crate::models::{Booking, Patient};
use std::io;

fn main() {
    println!("ご用件を入力してください（0:受診者登録,1:予約）");
    let mut service_type = String::new();
    io::stdin().read_line(&mut service_type).unwrap();
    let service_type: u8 = service_type.trim().parse().expect("入力された値が不正です");

    match service_type {
        0 => services::patient::run(),
        1 => services::booking::run(),
        _ => println!("入力された値が不正です"),
    }
}
