mod peek_search; 
use std::vec;
//use std::slice;

fn main() {
    let mut data = vec![1,2,3,5,1,2,3];
    let mut peek: i16 = peek_search::linear_peek_search(&data);
    println!("{:?}", data); 
    println!("{}",peek);

    data = vec![1,2,6,5,6,7,333];
    let peek: i16 = peek_search::linear_peek_search(&data);
    println!("{:?}", data); 
    println!("{}",peek);

    println!("{}",peek_search::binomial_peek_search(&data[..]));


}



