use chrono::{Datelike, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Patient {
    id: u32,
    name: String,
    birthday: NaiveDate,
    registration_date: NaiveDate,
}

impl Patient {
    pub fn new(name: String, birthday: NaiveDate) -> Self {
        let now = Utc::now();
        Patient {
            id: 0,
            name,
            birthday,
            registration_date: NaiveDate::from_ymd_opt(now.year(), now.month(), now.day()).unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct Booking {
    patient_id: u32,
}

impl Booking {
    pub fn new(patient_id: u32) -> Self {
        Booking { patient_id }
    }
}
