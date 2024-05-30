use serde::Serialize;
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomerOfProperty {
    pub budget: u64,
    pub dpi: u64,
    pub date: String,
}

pub fn create_customer_of_property(dpi: u64, budget: u64, date: String) -> CustomerOfProperty {
    CustomerOfProperty { dpi, budget, date }
}

impl Ord for CustomerOfProperty {
    fn cmp(&self, other: &Self) -> Ordering {
        self.budget.cmp(&other.budget)
    }
}

impl PartialOrd for CustomerOfProperty {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
pub struct Customer {
    pub dpi: u64,
    pub first_name: String,
    pub last_name: String,
    pub birth_date: String,
    pub job: String,
    pub place_job: String,
    pub salary: u64,
}

pub fn create_customer(
    dpi: u64,
    first_name: String,
    last_name: String,
    birth_date: String,
    job: String,
    place_job: String,
    salary: u64,
) -> Customer {
    Customer {
        dpi,
        first_name,
        last_name,
        birth_date,
        job,
        place_job,
        salary,
    }
}
#[derive(Serialize)]
pub struct Winner {
    pub dpi: u64,
    pub budget: u64,
    pub date: String,
    pub first_name: String,
    pub last_name: String,
    pub birth_date: String,
    pub job: String,
    pub place_job: String,
    pub salary: u64,
    pub property: String,
    pub signature: String,
}

pub fn create_winner(
    dpi: u64,
    date: String,
    first_name: String,
    last_name: String,
    birth_date: String,
    job: String,
    place_job: String,
    salary: u64,
    property: String,
    budget: u64,
    signature: String,
) -> Winner {
    Winner {
        dpi,
        date,
        first_name,
        last_name,
        birth_date,
        job,
        place_job,
        salary,
        property,
        budget,
        signature,
    }
}
