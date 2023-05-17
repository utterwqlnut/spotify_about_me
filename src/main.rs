use crate::grab::*;
pub mod grab;

fn main () {
    let result = grab::get_user_data(true);
    println!("{:?}",result);
}
