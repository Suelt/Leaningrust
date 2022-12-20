use std::{time::SystemTime, io, error, collections::HashMap};

pub fn test_chapter3() {
    // ch3_1();
    // testcase_2();
    test_inner_lifetime();
}

fn ch3_1() {




   
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }

    println!("{}", result);

    let s = String::from("111");

    let t = SystemTime::now();
    for i in 0..1_000_000 {
        consume_ref(&s)
    }

    let du = SystemTime::now().duration_since(t).unwrap().as_millis();
    println!("{}", du);
    for i in 0..1_000_000{
        consume(s.clone())
    }

   
    let du = SystemTime::now().duration_since(t).unwrap().as_millis();
    println!("{}", du);
}


fn longest<'a, 'b>(a: &'a str, b: &'b str) -> &'a str {
    a
}


fn consume_ref(a: &String) {

}

fn consume(a: String){

}
    


#[derive(Debug)]
struct Exert<'a> {
    info: &'a  String,
    index:  i32,
}

fn consume_lifetime<T>(e: T) {
    
}

fn test_inner_lifetime() {

    let i = 5;
    let s = "lifetime".to_string();
    let Exe = Exert{ info: &s, index: i};

    // std::mem::drop(s);

    println!("{:?}", Exe);

    // println!("{:?}", Exe);

    
    


    
}



impl<'a> Exert<'a> {
    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    fn test_exert<'b>(&'a self, b:&'b str) -> Result<&'b str, &dyn error::Error>
    
    {
        Ok(b)
    }

    fn test_exert2(&self, s: &str) -> &str {
        self.info
    }

}


    

fn testcase_2() {
   /*  let mut m = HashMap::new();
    m.insert("1", 2);
    

    match m.get_mut("1") {
        Some(i) => {println!("{}", i)},
        None => {
            m.insert("1", 2);
            m.get_mut("1").unwrap();
        },
    } */

    let i = 5;
    print_it(i);
    print_it1(i);
}
    
    
    
    
use std::vec::Vec;
/**
再借用的典例
*/
fn read_length(strings: &mut Vec<String>) -> usize {
   strings.len()
}

use std::fmt::Debug;

fn print_it<T: Debug + 'static>( input: T) {
    println!( "'static value passed in is: {:?}", input );
}

fn print_it1( input: impl Debug + 'static ) {
    println!( "'static value passed in is: {:?}", input );
}

