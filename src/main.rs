mod models;

use crate::models::{Booking, Patient};
use std::io;

fn main() {
    println!("ご用件を入力してください（0: 新規予約）");
    let mut service_type = String::new();
    io::stdin().read_line(&mut service_type).unwrap();
    let service_type: u8 = service_type.trim().parse().expect("入力された値が不正です");

    if service_type == 0 {
        println!("診察券番号を入力してください");
        let mut patient_id = String::new();
        io::stdin()
            .read_line(&mut patient_id)
            .expect("入力された値が不正です");
        let patient_id: u32 = patient_id.trim().parse().expect("数値で入力してください");
        let booking = Booking::new(patient_id);

        println!("{:?}", booking);
    } else if service_type == 1 {
        println!("お名前を入力してください");
        let mut name = String::new();
        io::stdin().read_line(&mut name).unwrap();
        let name = name.lines().collect::<String>();

        println!("生年月日を入力してください（例: 2019/05/24））");
        let mut birthday = String::new();
        io::stdin().read_line(&mut birthday).unwrap();
        let birthday = birthday.lines().collect::<String>();

        let patient = Patient::new(name, birthday);

        println!("{:?}", patient);
    } else {
        println!("不正な入力値です");
    }
}
