use rocket::fs::TempFile;
use serde_json::{Result, Value};
use std::collections::HashMap;

use super::customers::{
    create_customer, create_customer_of_property, create_winner, Customer, CustomerOfProperty,
    Winner,
};

use super::sort::quicksort;

use crate::utilities::read_all_lines;

//Hashes the dpi
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
fn hash_u64(value: u64) -> String {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    let hashed_value = hasher.finish();
    format!("{:x}", hashed_value)
}

// Converts the line into a hashmap
fn get_map(ln: &String) -> Result<Value> {
    let v: Value = serde_json::from_str(ln)?;

    Ok(v)
}
fn load_db(customers_db_strings: Vec<String>) -> HashMap<u64, Customer> {
    let mut customers_db = HashMap::new();
    // get all data from the customers and then creates
    // a struct of each customer
    for customer in customers_db_strings {
        let map = get_map(&customer);
        if !map.is_ok() {
            println!("Couldn't read line");
            break;
        }
        let map = map.ok().unwrap();
        // Just some error handling LOL
        // I don't know why it's being dropped
        // Reminder: avoid lifetimes unless I know what the hell they do
        let dpi = match map["dpi"].as_u64() {
            Some(x) => x,
            None => {
                break;
            }
        };
        let first_name = match map["firstName"].as_str() {
            Some(x) => x.to_string(),
            None => {
                break;
            }
        };
        let last_name = match map["lastName"].as_str() {
            Some(x) => x.to_string(),
            None => {
                break;
            }
        };
        let birth_date = match map["birthDate"].as_str() {
            Some(x) => x.to_string(),
            None => {
                break;
            }
        };
        let job = match map["job"].as_str() {
            Some(x) => x.to_string(),
            None => {
                break;
            }
        };
        let place_job = match map["placeJob"].as_str() {
            Some(x) => x.to_string(),
            None => {
                break;
            }
        };
        let salary = match map["salary"].as_u64() {
            Some(x) => x,
            None => {
                break;
            }
        };
        let element = create_customer(
            dpi, first_name, last_name, birth_date, job, place_job, salary,
        );
        customers_db.insert(dpi, element);
    }
    customers_db
}

fn determine_winner(auction: &String, customers_db: &HashMap<u64, Customer>) -> Option<Winner> {
    let auction = get_map(auction);
    let auction = auction.unwrap();

    let property = auction["property"].as_str()?;
    let rejection = auction["rejection"].as_u64()?;
    let customers: Vec<CustomerOfProperty> = auction["customers"]
        .as_array()?
        .to_vec()
        .iter()
        .map(|value| {
            create_customer_of_property(
                value["dpi"].as_u64().unwrap(),
                value["budget"].as_u64().unwrap(),
                value["date"].as_str().unwrap().to_string(),
            )
        })
        .collect();
    //Gets the budget of the winner
    let mut customers_copy = customers.clone();
    let n = customers_copy.len();
    quicksort(&mut customers_copy, 0, (n - 1) as isize);
    let winner_budget = customers_copy[rejection as usize].budget;
    //Because when we sort the vector, it doesn't take into account
    //who comes first, we determine how many people with the same
    //budget where rejected.
    let mut i = 0;
    let mut order = 0;
    while i <= rejection as usize {
        if customers_copy[i].budget == winner_budget {
            order += 1;
        }
        i += 1;
    }
    //We finally determine the true winner
    i = 0;
    let mut winner_index = 0;
    while i < customers.len() && order > 0 {
        if customers[i].budget == winner_budget {
            winner_index = i;
            order -= 1;
        }
        i += 1;
    }
    let winner = customers[winner_index].clone();

    let customer = customers_db.get(&winner.dpi)?;

    Some(create_winner(
        winner.dpi,
        winner.date.clone(),
        customer.first_name.clone(),
        customer.last_name.clone(),
        customer.birth_date.clone(),
        customer.job.clone(),
        customer.place_job.clone(),
        customer.salary,
        property.to_string(),
        winner.budget,
        hash_u64(winner.dpi),
    ))
}

pub async fn get_winner<'f>(database: TempFile<'f>, file: TempFile<'f>) -> Vec<String> {
    let customers_db = load_db(read_all_lines(database).await);
    let auctions = read_all_lines(file).await;
    let mut ans: Vec<String> = vec![];

    for auction in auctions {
        let winner = determine_winner(&auction, &customers_db);
        if winner.is_some() {
            let winner = winner.unwrap();

            ans.push(serde_json::to_string(&winner).unwrap());
        }
    }
    ans
}
