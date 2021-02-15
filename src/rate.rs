#![feature(exclusive_range_pattern)]
#![allow(unused_mut,unused_attributes)]

//Function to add rate accroding to percentage.
pub fn rate_adder(mut rate: &f64, mut per: f64) -> (f64,f64) {

    let vendor_rate =  (rate * per) / 100.0;
    let total = rate + vendor_rate;
    return (total,vendor_rate)

}
/*Conditional matching with various rate margin 
 * and finding respective rates for total charge and service rates.*/
pub fn final_rates(mut rate:f64) -> (f64, f64) {

    match rate {
        //ranges of some values.
        rate if (1.0..101.0).contains(&rate) => {

            let (total_prod_charge,vendor_fee) =  rate_adder(&rate,10.0);
            return (total_prod_charge,vendor_fee)

        },
                                                                               
        rate if (101.0..301.0).contains(&rate) => {

            let (total_prod_charge,vendor_fee) =  rate_adder(&rate,11.0);
            return (total_prod_charge,vendor_fee)

        },
                                                       
        rate if (301.0..701.0).contains(&rate) => {

            let (total_prod_charge,vendor_fee) =  rate_adder(&rate,12.0);
            return (total_prod_charge,vendor_fee)
        },

        rate if (701.0..1201.0).contains(&rate) => {

            let (total_prod_charge,vendor_fee) =  rate_adder(&rate,13.0);
            return (total_prod_charge,vendor_fee)
        },

        rate if (1201.0..2001.0).contains(&rate) => {

            let (total_prod_charge,vendor_fee) =  rate_adder(&rate,14.0);
            return (total_prod_charge,vendor_fee)
        },  
        //Any values other than the above mentioned values.
                  _         =>  {

            let (total_prod_charge,vendor_fee) =  rate_adder(&rate,15.0);
            return (total_prod_charge,vendor_fee)

        }

    }

}
