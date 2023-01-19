use crate::models::Patient;
use chrono::NaiveDate;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Write;

const FILE_PATH: &str = "store/data.json";

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

    let mut data = read_data_or_create_new_data();
    data.push(patient);

    write_to_json(&data);
}

fn read_data_or_create_new_data() -> Vec<Patient> {
    let file = File::open(FILE_PATH);
    match file {
        Ok(f) => {
            let buf_read = BufReader::new(f);
            serde_json::from_reader(buf_read).expect("デシリアライズに失敗")
        }
        Err(_) => {
            println!("新規ファイルを作成");
            Vec::new()
        }
    }
}

pub fn write_to_json(data: &Vec<Patient>) {
    let json_data = serde_json::to_string_pretty(data).expect("JSONのシリアライズに失敗");
    let mut file = File::create(FILE_PATH).expect("書き込みのオープンに失敗");
    writeln!(file, "{}", json_data).expect("ファイルの書き込みに失敗");
    println!("受診者登録が完了しました");
}
