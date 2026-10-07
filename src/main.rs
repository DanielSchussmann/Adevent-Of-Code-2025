
use colored::Colorize;


include!("day_1.rs");
include!("day_2.rs");

fn main(){
    print!("{}\n{}\n\n","-=-=-=-=-=-=-=-=-=-=- DAY 1 -=-=-=-=-=-=-=-=-=-=-".to_string().blue(),day_1(0,false));
    print!("\n{}\n{}\n","-=-=-=-=-=-=-=-=-=-=- DAY 2 -=-=-=-=-=-=-=-=-=-=-".to_string().blue(),day_2(false));

}