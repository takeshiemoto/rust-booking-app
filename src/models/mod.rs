#[derive(Debug)]
pub struct Booking {
    patient_id: u32,
}

impl Booking {
    pub fn new(patient_id: u32) -> Self {
        Booking { patient_id }
    }
}

#[derive(Debug)]
pub struct Patient {
    id: u32,
    name: String,
    birthday: String,
}

impl Patient {
    pub fn new(name: String, birthday: String) -> Self {
        Patient {
            id: 0,
            name,
            birthday,
        }
    }
}
