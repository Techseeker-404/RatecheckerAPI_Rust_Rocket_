#![allow(unused_mut)]
#![feature(proc_macro_hygiene,decl_macro)]
#[macro_use]
extern crate rocket;
use rocket::response::Debug;
use rocket_contrib::json;
use rocket_contrib::json::{Json,JsonValue};

mod rate;
//Test intro page:
#[get("/")]
fn hello() -> String {
    let mut str = String::from("\u{1F41D}ShopeeBee "); 
    //str.push('\u{1F41D}');
    str = str + "\nEnter http://localhost:8000/rate/any value 
          eg: 100 or 3000..etc for its respective rates";

    return str
}
//Test operation to ensure its working.
#[get("/sum/<num1>/<num2>")]
fn add(num1: i32, num2: i32) -> String {
    let sum = num1 + num2;
    return sum.to_string()
}
//Implementation 0.1
#[get("/ratecheck/<rate_num>")]
fn rate_finder(mut rate_num:f64) -> Result<Json<String>,Debug<std::io::Error>> {
    let (total,vendor_fee) = rate::final_rates(rate_num);
    Ok(Json(serde_json::to_string(&(total,vendor_fee)).expect("Invalid JSON")))
}
//Implementation 0.1.1
#[get("/rate/<rate_num>")]
fn rate_findernew(mut rate_num:f64) -> JsonValue {
    let (total,vendor_fee) = rate::final_rates(rate_num);
    json!({
        "Total rate": total.to_string(),
        "Vendor to service-provider rate":vendor_fee.to_string()
    })
}
//main section to check on our API: 

fn main() {
    rocket::ignite().mount("/",routes![hello,add,rate_finder,rate_findernew]).launch();
}

