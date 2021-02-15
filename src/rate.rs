#![feature(exclusive_range_pattern)]
#![allow(unused_mut,unused_attributes)]

//Function to add rate accroding to percentage.
pub fn rate_adder(mut rate: &f64, mut per: f64) -> (f64,f64) {
    let vendor_rate =  (rate * per) / 100.0;
    let total = rate + vendor_rate;
    return (total,vendor_rate)
}
//Main.
pub fn final_rates(mut rate:f64) -> (f64, f64) {
    

    match rate {
        //ranges of some values.
        mut rate if (1.0..101.0).contains(&rate) => {let (total_prod_charge,vendor_fee) =  rate_adder(&mut rate,10.0);
                                                        return (total_prod_charge,vendor_fee)},                       
        mut rate if (101.0..301.0).contains(&rate) => {let (total_prod_charge,vendor_fee) =  rate_adder(&mut rate,11.0);
                                                        return (total_prod_charge,vendor_fee)},
        mut rate if (301.0..701.0).contains(&rate) => {let (total_prod_charge,vendor_fee) =  rate_adder(&mut rate,12.0);
                                                        return (total_prod_charge,vendor_fee)},
        mut rate if (701.0..1201.0).contains(&rate) => {let (total_prod_charge,vendor_fee) =  rate_adder(&mut rate,13.0);
                                                        return (total_prod_charge,vendor_fee)},                     
        mut rate if (1201.0..2001.0).contains(&rate) => {let (total_prod_charge,vendor_fee) =  rate_adder(&mut rate,14.0);
                                                        return (total_prod_charge,vendor_fee)},                     
        _            =>  {let (total_prod_charge,vendor_fee) =  rate_adder(&mut rate,15.0);
                                                        return (total_prod_charge,vendor_fee)}
    }
}
