use crate::models::Booking;
use std::io;

pub fn run() {
    println!("診察券番号を入力してください");
    let mut patient_id = String::new();
    io::stdin()
        .read_line(&mut patient_id)
        .expect("入力された値が不正です");
    let patient_id: u32 = patient_id.trim().parse().expect("数値で入力してください");
    let booking = Booking::new(patient_id);

    println!("{:?}", booking);
}
