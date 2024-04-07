#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plan {
    pub year: i32,
    pub min: f64,
    pub max: f64,
}

impl Plan {
    pub fn new(year: i32, min: f64, max: f64) -> Self {
        Self { year, min, max }
    }
}

pub async fn all_plan() -> Vec<Plan> {
    vec![
        Plan::new(2015, 68000.0, 74000.0),
        Plan::new(2016, 54000.0, 59000.0),
        Plan::new(2017, 69600.0, 77300.0),
        Plan::new(2018, 72700.0, 78200.0),
        Plan::new(2019, 76000.0, 86000.0),
        Plan::new(2020, 88500.0, 100000.0),
        Plan::new(2021, 81000.0, 110250.0),
        Plan::new(2022, 52000.0, 64000.0),
        Plan::new(2023, 67750.0, 88000.0),
        Plan::new(2024, 90000.0, 116000.0),
        Plan::new(2025, 96500.0, 124000.0),
        Plan::new(2026, 96500.0, 124000.0),
    ]
}
