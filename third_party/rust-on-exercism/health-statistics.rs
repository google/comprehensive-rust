// ANCHOR: solution
// ANCHOR: setup
pub struct User {
    name: String,
    age: u32,
    height: f32,
    visit_count: usize,
    last_blood_pressure: Option<(u32, u32)>,
}

pub struct Measurements {
    height: f32,
    blood_pressure: (u32, u32),
}

pub struct HealthReport<'a> {
    patient_name: &'a str,
    visit_count: u32,
    height_change: f32,
    blood_pressure_change: Option<(i32, i32)>,
}

impl User {
    // ANCHOR_END: setup
    // ANCHOR: User_new
    pub fn new(name: String, age: u32, height: f32) -> Self {
        // ANCHOR_END: User_new
        Self {
            name,
            age,
            height,
            visit_count: 0,
            last_blood_pressure: None,
        }
    }

    // ANCHOR: User_name
    pub fn name(&self) -> &str {
        // ANCHOR_END: User_name
        &self.name
    }

    // ANCHOR: User_age
    pub fn age(&self) -> u32 {
        // ANCHOR_END: User_age
        self.age
    }

    // ANCHOR: User_height
    pub fn height(&self) -> f32 {
        // ANCHOR_END: User_height
        self.height
    }

    // ANCHOR: User_doctor_visits
    pub fn doctor_visits(&self) -> u32 {
        // ANCHOR_END: User_doctor_visits
        self.visit_count as u32
    }

    // ANCHOR: User_set_age
    pub fn set_age(&mut self, new_age: u32) {
        // ANCHOR_END: User_set_age
        self.age = new_age
    }

    // ANCHOR: User_set_height
    pub fn set_height(&mut self, new_height: f32) {
        // ANCHOR_END: User_set_height
        self.height = new_height
    }

    // ANCHOR: User_visit_doctor
    pub fn visit_doctor(&mut self, measurements: Measurements) -> HealthReport {
        // ANCHOR_END: User_visit_doctor
        self.visit_count += 1;
        let bp = measurements.blood_pressure;
        let report = HealthReport {
            patient_name: &self.name,
            visit_count: self.visit_count as u32,
            height_change: measurements.height - self.height,
            blood_pressure_change: match self.last_blood_pressure {
                Some(lbp) => Some((
                    bp.0 as i32 - lbp.0 as i32,
                    bp.1 as i32 - lbp.1 as i32
                )),
                None => None,
            }
        };
        self.height = measurements.height;
        self.last_blood_pressure = Some(bp);
        report
    }
}

// ANCHOR: main
fn main() {
    let bob = User::new(String::from("Bob"), 32, 155.2);
    println!("I'm {} and my age is {}", bob.name(), bob.age());
}
// ANCHOR_END: main

// ANCHOR: tests
#[test]
fn test_height() {
    let bob = User::new(String::from("Bob"), 32, 155.2);
    assert_eq!(bob.height(), 155.2);
}

#[test]
fn test_set_age() {
    let mut bob = User::new(String::from("Bob"), 32, 155.2);
    assert_eq!(bob.age(), 32);
    bob.set_age(33);
    assert_eq!(bob.age(), 33);
}

#[test]
fn test_visit() {
    let mut bob = User::new(String::from("Bob"), 32, 155.2);
    assert_eq!(bob.doctor_visits(), 0);
    let report = bob.visit_doctor(Measurements {
        height: 156.1,
        blood_pressure: (120, 80),
    });
    assert_eq!(report.patient_name, "Bob");
    assert_eq!(report.visit_count, 1);
    assert_eq!(report.blood_pressure_change, None);

    let report = bob.visit_doctor(Measurements {
        height: 156.1,
        blood_pressure: (115, 76),
    });

    assert_eq!(report.visit_count, 2);
    assert_eq!(report.blood_pressure_change, Some((-5, -4)));
}
// ANCHOR_END: tests
