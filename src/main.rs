use std::io;

#[derive(Debug)]
pub struct Booking {
    date_time: String,
    num_guest: u8,
    name: String,
}

impl Booking {
    pub fn new(date_time: String, num_guest: u8, name: String) -> Self {
        Booking {
            date_time,
            num_guest,
            name,
        }
    }
}

fn main() {
    println!("ご用件を入力してください（0: 新規予約）");
    let mut service_type = String::new();
    io::stdin().read_line(&mut service_type).unwrap();
    let service_type: u8 = service_type.trim().parse().expect("不正な値");

    if service_type == 0 {
        // 日時型およびバリデーションを実施
        println!("ご予約日時を入力してください（yyyy-MM-dd HH:mm）");
        let mut date_time = String::new();
        io::stdin().read_line(&mut date_time).expect("不正な値");
        let date_time = date_time.lines().collect::<String>();

        println!("利用人数を数値で入力してください");
        let mut num_guest = String::new();
        io::stdin().read_line(&mut num_guest).expect("不正な値");
        let num_guest: u8 = num_guest.trim().parse().expect("不正な値");

        println!("お名前を入力してください");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("不正な値");
        let name = name.lines().collect::<String>();

        let booking = Booking::new(date_time, num_guest, name);
        println!("{:?}", booking);
    }
}
