// use std::{iter::Map, time::SystemTime};

// use num::Complex;

// fn greet_world() {
//     let sourthen_germany = "Grüß Gott!"; 
//     let chinese = "世界，你好";
//     let english = "World, hello";
//     let regions = [sourthen_germany, chinese, english];
//     for region in regions {
//         println!("{}", &region);
//     }
// }


// fn more_difficult_segment() {
//     // 这里加了\就不会被lines识别为新的一行，本来双引号这里的第一行也是会算一行的
//     let penguin_data = "\
//     common name,length (cm)
//     Little penguin,33
//     Yellow-eyed penguin,65
//     Fiordland penguin,60
//     Invalid,data
//     ";
    
//     let records = penguin_data.lines();
//     for (i, record) in records.enumerate() {
//         if i == 0 || record.trim().len() == 0 {
//             continue;
//         }
//         //println!("{}", record);
//         // for field in record.split(',') {
//         //     println!("{}", field);
//         // }
//         // for field in record.split(',').map(|f| f.trim()) {
//         //     println!("{}", field)
//         // }


        

//         let fields : Vec<_>= record.split(',').map(|field| field.trim()).collect();
//         if cfg!(debug_assertions) {
//             eprintln!("debug: {:?} -> {:?}", record, fields)
//         }
//       //  println!("{}", fields[0]);
//         let name = fields[0];

//         if let Ok(length) = fields[1].parse::<f32>() {
//             println!("{}, {}", name, length);
//         }
       
//     }

    
// }


// fn ch2_1() {
//     /*
//     显式声明变量类型
//     */
//     let a = 10;
//     let b: i32 = 10;
    
//     let mut c = 30i32;

//     let d = 30i32;

//     let e = add(add(a,b), add(c, d));

//     println!("{}", e);

//     /*
//     变量解构
//     */
//     let (a, mut b) = (true, false);
//     println!("{}, {}", a, b);

//     b = true;
//     b = false;

//     // assert_eq!(a, b);

//     /*
//     解构式赋值
//     */
//     struct S {
//         e: i32
//     }
//     let (a, b, c, d, e) : (i32, i32, i32, i32, i32);
//     (a, b) = (1, 2);

//     [c, .., d, _] = [1, 2, 3, 4, 5];

//     S {e, ..} = S {e: 5};    

//     assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);


//     const MAX_POINTS: i32= 100_000;
//     // static bb:i32 = 100000;
//     let mut c = MAX_POINTS;




//     println!("{}", c);


//     // let mut x = 5;
//     // let mut x = x + 1;
//     // let mut x = x + 1;

//     // let x = 5;
//     // let x = x + 1;
//     // let x = x + 2;

//     let space = "    ";
//     let space = space.len();
//     println!("{}", space);
// }

// fn add(i: i32, j: i32) -> i32 {
//     i+j
// }



// fn ch2_2_1() {
//     let guess = "42".parse::<i32>().unwrap();
//     let guess:String = "asdasdas".parse().expect("not a number");
//     println!("{:?}", guess);

//     let hexicode = "0xffff";
//     let purestring:Vec<_> = hexicode.split("0x").collect();

//     let str = purestring[1];
//     /*
//     测试uint8 的溢出处理
//     */
//     let mut a: u8 = 255;
//     let b = a.wrapping_add(20);

//     let (b, _) = a.overflowing_add(20);
//     let b = a.saturating_add(126);
//     println!("{}", b);

//     /*
//     浮点数 
//     */

//     let a: f32 = 1.0;
//     let b: f64 = 1.0;
//    // assert!(0.1 + 0.2 == 0.3);
//     // assert!((0.1_f64 + 0.2 - 0.3).abs() < 0.00000000000000000001);

//     // assert!((0.1_f32 + 0.2 - 0.3).abs() < 0.0000000000000000000000000000000000000000000000000001_f32);
//     let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);

//     let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

//     // println!("{}", abc.0.to_bits());
//     // println!("{}", xyz.0.to_bits());
//     // println!("abc (f32)");
//     // println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
//     // println!("         0.3: {:x}", (abc.2).to_bits());
//     // println!();

//     // println!("xyz (f64)");
//     // println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
//     // println!("         0.3: {:x}", (xyz.2).to_bits());
//     // println!();

//     // assert!(abc.0 + abc.1 == abc.2);

//     // assert!(xyz.0 + xyz.1 == xyz.2);

//     /*
//     数字运算
//     */
//     let f: f32 = 1.0002;
//     println!("{:.2}", f);

//     /* 
//     位运算
//     */

//     let a = 2;
//     let b = 3;
//     // println!("{}", a & b);
//     // println!("{}", a | b);
//     // println!("{}", a ^ b);
//     // let x = a << b;
//     // println!("{}", a << b);
//     // println!("{}", a >> b);

//     // let s = "asedasg";
//     // for i in 0..5 {
//     //     println!("{}: {:?}", i, s.chars().nth(i));
//     // }

//     /*
//     rust 数字库
//     引用web库两种方法
//     / cargo add + extern_lib
//     / 在Cargo.toml的dependencies中加入 extern_lib = "version"
//     */
//     let a = Complex { re: 2.1, im:-1.2};
//     let b = Complex::new(11.1, 22.2);
//     let result = a + b;

//     println!("{} {}", result.re, result.im);

    
// }


// fn ch2_2_2() {

//     let c = 'z';
//     let z = 'ℤ';
//     let g = '国';
//     let heart_eyed_cat = '😻';

//     println!("{}", std::mem::size_of_val(&z));

//     let t = true;
//     let f = false;
    
// }

// fn ch2_2_3() {
//     let a = 8;
//     let v1: Vec<i32> = Vec::new();
//     let mut v: Vec<i32> = [1,2,3].to_vec();
//     let (a, c) = (1, 3);

//     /*
//     块返回值 *** rust的特性
//     */
//     let y = {
//         let x = 3;
//         x + 1
//     };
//     println!("{}", y);

// }

// pub fn ch2_2_4() {
//     let mut s = "123";
//     test_function(s)
// }

// fn test_function(a: &'static str) {
//     println!("{}", a);
// }

// fn test_str(a: &mut String) {
//     *a = String::from("")


// }



// pub fn ch_2_3_1() {

//     {
//     let s = "abc";
//     }

//     let mut s = String::from("hello");
//     let c = s.get(0..3);
//     println!("{:?}", c.unwrap());

//     s.push_str(" world");
//     //println!("{}", s);
//     // println!("{}", s);
//     /*
//     基本数据类型的拷贝是栈内存拷贝
//     复杂类型的拷贝是 "移动"， 即在复杂类型拷贝后前面的那个引用失效，把所有权从s2转移到了s1
//     */
//     // let sy_time = SystemTime::now();
//     // for i in 0..100_000_0{
//     //     let a = 5;
//     //     let b = a;
//     // }
//     // let duration = SystemTime::now().duration_since(sy_time).unwrap().as_secs();
//     // println!("{}", duration);
//     // let sy_time = SystemTime::now();

//     /* 
//     这里看到两个s指向了同一堆内存，其中s1和s2的地址差了24个字节，刚好是8字节指针，8字节长度，8字节容量
//     */
//     // for i in 0..100_000_00{
//     //     let s1 = String::from("hello");
//     //     let s2 = s1;
//     //     println!("{}", s2);
//     // }
//     // let duration = SystemTime::now().duration_since(sy_time).unwrap().as_secs();
//     // println!("time consumed by {}", duration);

//     /*
//     如果不想要实现移动，则不能用自动的复制，要使用clone方法
//     此处堆内存有两个字符串hello world
//     */
    
//     let s1 = String::from("hello world");
//     let s2 = s1.clone();
    
//     println!("{}, {}", s1, s2);
//     // let x = "hello world";
//     // let y = x;
//     // println!("{}, {}", x, y);
//     /*
//     此处引用的都是同一个写在程序代码里的字符串"str", 在运行时被加载到内存的某个区域
//     */
//     let x = 5;
//     let y = x;
//     let a = "str";
//     let b = a;
//     let c = "str";

//     println!("x = {}, y = {}", x, y);
//     /* 
//     s在传入函数后就不再有效
//     */
//     // let s = String::from("123");
//     // takes_ownership(s);
//     /* 
//     函数内开辟了一块新内存存储字符串"123"，绑定了一个变量，并把所有权交给了s
//     */
//     // let s = gives_ownership();
//     /*
//     s把所有权交给了一个新的指针s，旧s失效
//     */
//     // let s = gives_ownership_and_back(s);
//     // println!("{}", s);
//     let x = String::from("123");
//     let y = &x;
//     takes_ownership(x);
//     // println!("{}", y);
// }


// fn takes_ownership(s: String) {
//     println!("{}", s);
// }

// fn gives_ownership() -> String {
//     let some_string = String::from("123");
//     some_string
// }

// fn gives_ownership_and_back(s: String) -> String {
//     s
// }



// pub fn ch2_3_2() {
//     let x = 5;
//     let y = &x;
//     println!("{}", x);
//     println!("{}", *y);


//     let mut s = String::from("reference");

//     let r = &mut s;
//     test_reference(r);

//     println!("{}", s);

//     // let r = &mut s;
//     // let r2 = &mut s;
//     /*
//     取引用后实际上就把所有权借用给了那个引用，在这之后不能修改数据的值直到该借用的生命周期结束
//     在rust中实际上是对指针做了语法层面的规定，即在编译期加上读写锁
//     因此
//     1. 不能在不可变引用作用域内使用可变引用修改值
//     2. 不能同时存在多个不可变引用
//     3. 也不能在可变引用作用域结束前去修改值
//     4. rust中使用NLL来判断一个变量的作用域结束，即在该变量的最后一次使用就结束
//     */
//     // let r_immutable = &s;
//     // s.push('1');
//     // println!("{}", r_immutable);
// }

// fn test_reference(s: &mut String) {
//     s.len();
//     s.push('1');
//     println!("{}", s);
// }


use core::panic;
use std::{string, ops::{Add, Deref, DerefMut}, str::from_utf8, fmt::Display, time::SystemTime, alloc::System, io::{self, Read}, process::Output, collections::HashMap, error::{self, Error}, fs::File};



pub fn ch2_4_1() {
    // let s = "str";
    // greet(s);


    // let mut s = String::from("hello,string");
    // let h = &s[0..5];
    // let p = &s[6..10];
    // println!("{}", h);


    /* 
    对字符串使用切片语法时，要注意字符串中存储的字符是什么，例如中文在UTF-8中占用三个字节，则下面的代码就会崩溃
    let s = String::from("我是CHN");
    let h = &s[0..2];
    println!("{}", h);

    */
    // let s = String::from("我是CHN");
    // let h = &s[0..9];
    // println!("{}", h);

    let a = [1,2,3,4,5];
    let a_slice = &a[0..2];
    let a_slice = &a[..2];
    let a_slice = &a[..];
    let a_slice = &a[0..];
}
    /*
    字符串字面量是切片
    其中 let s = "hello world";
    是一个&str切片，该切片指向了程序可执行文件中的某个点，这个点存储了"hello world"的字符串数据
    所以字符串字面量是不可变的，因为&str是一个不可变引用 
    */


    /*
    Rust 中的字符是 Unicode 类型，因此每个字符占据 4 个字节内存空间，
    但是在字符串中不一样，字符串是 UTF-8 编码，也就是字符串中的字符所占的字节数是变化的(1 - 4) 
    */
    
    // let s = String::from("नमस्ते");
    // println!("{:?}", s.into_bytes());

    // let s = String::from("我是CHN");
    // println!("{:?}", s.into_bytes())




    /*
    字符串追加方法
    */
    // let mut s = String::from("hello ");
    // s.push('r');
    // s.push_str("ust");

    // println!("{}", s);
    // /*
    // 字符串插入方法
    // */
    // s.insert(5, ',');
    // println!("{}", s);

    // s.insert_str(6,"there");
    // println!("{}", s);
    // /*
    // 字符串替换方法
    // */
    
    // let mut s =  String::from("rust , hello rust");
    // s = s.replace("rust", "hust");

    // s.replace_range(0..1, "H");
    // println!("{}", s);
    // String::replace_range(&mut s, 0..2,"hu");
    // println!("{}", s);
    
    
    // /*
    // 1. 字符串删除方法 pop 方法以utf8字符为单位
    // */
    // // let s = String::from("中国");
    // // let s = s.as_str();
    // // println!("{:?}", s.chars().nth(3));
    // let mut string_pop = String::from("rust pop 中文!");
    // let p1 = string_pop.pop();
    // let p2 = string_pop.pop();
    // dbg!(p1);
    // dbg!(p2);
    // dbg!(string_pop);


    // /*
    // 2. remove 方法是以字节为单位的
    // */
    
    // let mut string_remove = String::from("测试remove方法");
    // println!(
    //     "string_remove 占 {} 个字节",
    //     std::mem::size_of_val(string_remove.as_str())
    // );
    // // 删除第一个汉字
    // string_remove.remove(0);
    // // 下面代码会发生错误
    // //string_remove.remove(1);
    // // 直接删除第二个汉字
    // // string_remove.remove(3);
    // dbg!(string_remove);

    // /*
    // 3. truncate 方法以字节为单位， 是删除字符串从指定位置到结尾的全部
    // */
    // let mut string_truncate = String::from("测试truncate");
    // string_truncate.truncate(3);
    // dbg!(string_truncate);


    // /*
    // clear方法清空字符串
    // */
    
    // let mut string_clear = String::from("string clear");
    // string_clear.clear();
    // dbg!(string_clear);
    
/*     /* 
    字符串连接方法
    */

    /*
    + or +=
    */

    let mut string_append = String::from("hello ");
    string_append = string_append + "rust";
    
    let mut s = string_append+ "123";
    s.push('1');
    /*
    把String类型用在语句中也会转移所有权, 下面的一行代码会报错
    */
    
    //println!("{}", string_append);
    println!("{}", s);

    let mut string_add = String::from("hello");
    let s = string_add.add("123");
    /*
    string_add调用add方法后所有权move了
    */
    //println!("{}", string_add);
    /*
    format 返回String,输入不需要是String类型
    */
    // let s = format!("{} {}", "123", "345");
    // println!("{}", s);

    // /* 
    // 字符串转义  使用\x转义16进制 使用\u转义unicode
    // */

    // /*
    // 操作utf8字符串的方法
    // */

    // /*
    // 1. chars
    // */
    // for c in "中国".chars() {
    //     println!("{}", c);
    // }
    // for i in "中国".bytes() {
    //     println!("{}", i);
    // }

    // /*
    // 获取utf8子串即一些其他操作
    // utf8_slice库
    // */
    // let s = "holla中国人नमस्ते";
    // let usize = utf8_slice::len(s);
    // println!("{}", usize);

    // let mut s = String::from("hello");
    // let s2: &str = &s[0..1];
   
    // let bytestring = b"this is a byte string";
    // println!("{:?}", bytestring);



    // let arr = ['中', '国' , 'x' ];
    // let s = &arr[0..2];
    // let mut s = String::from("hello world");

    // // 这里, &s 是 `&String` 类型，但是 `first_word` 函数需要的是 `&str` 类型。
    // // 尽管两个类型不一样，但是代码仍然可以工作，原因是 `&String` 会被隐式地转换成 `&str` 类型，如果大家想要知道更多，可以看看 Deref 章节: https://course.rs/advance/smart-pointer/deref.html
    // let word = first_word(&s);
    
    // s.clear(); // error!

    // println!("the first word is: {}", word);
    // let mut v: Vec<i32> = Vec::new();
    // v.push(1);
    // v.push(2);
    // let s = &v;
    // println!("{:?}", v);


    // let mut s = String::from("hello, world");

    // let slice1: &str = &s; // 使用两种方法
    // assert_eq!(slice1, "hello, world");
 
    // let slice2 = &s[0..5];
    // assert_eq!(slice2, "hello");
 
    // let mut slice3 = s;
    // slice3.push('!');
    // assert_eq!(slice3, "hello, world!");
 
    // println!("Success!")

   
}




// fn first_word(s: &str) -> String {
//     let s = &s[0..1];
//     s[0..1].to_string()
// }

pub fn ch2_4_2() {
    // let (x, y, z): (i32, f64, u8);
    // (x, y, z) = (1, 5.0, 3);
    // println!("{}", x);

    // let mut tuple = (1, 3.0, String::from("123"));
    // let x = tuple.0;
    // let mut zz = &mut tuple.2;
    // println!("{}", zz);
    // let mut s = String::from("hello");
    // println!("{:?}", calculate_length(s));


}

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len();
//     (s, length)
// } 

#[derive(PartialEq)]
#[derive(Clone)]

#[derive(Hash)]
pub struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User").field("active", &self.active).field("username", &self.username).field("email", &self.email).field("sign_in_count", &self.sign_in_count).finish()
    }

}




impl User {
    fn new(active: bool, username: &str, email: &str, sign_in_count: u64) -> Self { Self { active, username: String::from(username), email: String::from(email), sign_in_count } }

    fn username_mut(&mut self) -> &mut String {
        &mut self.username
    }

    fn username(&self) -> &str {
        self.username.as_ref()
    }
    
    fn set_username(&mut self, username: String) {
        self.username = username;
    }

    

   

    pub fn email(&self) -> &str {
        self.email.as_ref()
    }

    

   
}






pub fn ch2_4_3() {
    // let s = String::from("123");
 
    
    // let mut count = 10;
    // let expr = count > 10;
    // match expr {
    //     _ => {},
    // }

    // while expr {
        
    // }

    // dbg!(expr);

  /*   let mut u = User {
        active: false,
        username: String::from("Hanzo"),
        email: String::from("xtt993989@gmail.com"),
        sign_in_count: 10,
    };

    let time_start = SystemTime::now();
    let mut v: Vec<User> = Vec::new();
    
    for i in 0..1_000_000 {
        let mut u = User::new(false, "h", "1", 10);
        
        
        //println!("{:p}", u.email());
        
        
        v.push(u);
        //println!("{:?}", v.len());
        //println!("{:p}", u.email());
    }
    

    let time_consumed = SystemTime::now().duration_since(time_start).unwrap().as_millis();
    println!("{}", time_consumed);
    
    let d = u.deref();
    println!("{}", d);
    println!("{}", *u);
    println!("{:?}", u); */
    // u.email = String::from("255@qq.com");
    
    // u.set_username(String::from("_"));

    // println!("{}", u.username());

    // let r = u.username_mut();
    // r.push('1');

    // println!("{}", u.username());

    let mut u = User::new(false, "h", "1", 10);

    // test_File();
    
    let mut u2 = u.clone();
    u2.active = true;
    assert_eq!(u, u2);

}   


#[derive(Debug)]
struct File {
    data: Vec<u8>,
    name: String,
}

impl File {
    /// Creates a new [`File`].
    fn new(name: String, data: Vec<u8>) -> Self { Self { name, data } }
}

fn test_file() {
    let mut f = File::new(String::from("234.out"), Vec::new());
    println!("{}", f.name);

}


struct LifeTime<'a> {
    data: &'a str,
    name: &'a Vec<u8>,
}

impl<'a> LifeTime<'a> {
    fn new(data: &'a str, name: &'a Vec<u8>) -> Self { Self { data, name } }
}




#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum PokerSuit {
    Clubs { field1: u8 },
    Spades { field1: u8 },
    Diamonds { field1: u8 },
    Hearts { field1: u8 },
    
}



#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct PokerCard {
    suit: PokerSuit,
    value: u8,
}

impl PokerCard {
    fn new(suit: PokerSuit, value: u8) -> Self { Self { suit, value } }
}





#[derive(Debug)]
#[derive(PartialEq, Eq)]
struct Ipv4Addr {
    network_addr: String,
}

#[derive(Debug)]
#[derive(PartialEq, Eq)]
struct Ipv6Addr {}

#[derive(Debug)]
#[derive(PartialEq, Eq)]

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
    
}

impl From<Ipv6Addr> for IpAddr {
    fn from(v: Ipv6Addr) -> Self {
        Self::V6(v)
    }
}

impl From<Ipv4Addr> for IpAddr {
    fn from(v: Ipv4Addr) -> Self {
        Self::V4(v)
    }
}


struct QuitMessage {}



impl Message for QuitMessage {
    fn act_message(&self) -> String {
        String::from("Quit")
    }
}
struct MoveMessage {}

impl Message for MoveMessage {
    fn act_message(&self) -> String {
        String::from("move")
    }
}

pub trait Message {
    fn act_message(&self) -> String;
}


/// .
fn test_trait(m: &dyn Message) {
    println!("{}", m.act_message());
}

pub fn ch2_4_4() {
    // let heart = PokerSuit::Hearts { field1: 8 };
    // let diamond = PokerSuit::Diamonds { field1: 7 };
    // println!("{:?}, {:?}", heart, diamond);
    
    // let p = PokerCard::new(PokerSuit::Clubs{field1: 7}, 1);
    // let q = PokerCard::new(PokerSuit::Clubs{field1: 8 }, 2);
    // println!("{}", p > q);

    // let i = IpAddr::from(Ipv4Addr{ network_addr: String::from("192.168.1.1")});
    // let j = IpAddr::V6(Ipv6Addr {});
    // assert_eq!(i, j);



    // println!("{:?}", i);

/* 
    let q = QuitMessage{};
    let m = MoveMessage{};
    test_trait(&q);
    test_trait(&m);

    let some_number = Some(5);
    let some_string = Some(String::from("123"));

    let absent_number: Option<i32> = None;
    

    let x = Some(5);
    let y = test_option(x);
    let z: Option<i32> = None;
    test_option(z);
    println!("{:?} {:?}", x, y); */

    let mut list = List::new();

    // 添加一些元素
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // 打印列表的当前状态
    println!("链表的长度是: {}", list.len());
    println!("{}", list.stringify());
}

/// .
fn test_option(x: Option<i32>) -> Option<i32> {
    println!("{:?}", x);
    match x {
        Some(i) => {Some(i + 1)},
        None => {panic!("null")},
    }
    
}


use crate::chapter_2::ch2::List::*;

enum List {
    // Cons: 链表中包含有值的节点，节点是元组类型，第一个元素是节点的值，第二个元素是指向下一个节点的指针
    Cons(u32, Box<List>),
    // Nil: 链表中的最后一个节点，用于说明链表的结束
    Nil,
}

// 为枚举实现一些方法
impl List {
    // 创建空的链表
    fn new() -> List {
        // 因为没有节点，所以直接返回 Nil 节点
        // 枚举成员 Nil 的类型是 List
        Nil
    }

    // 在老的链表前面新增一个节点，并返回新的链表
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    // 返回链表的长度
    fn len(&self) -> u32 {
        match *self {
            // 这里我们不能拿走 tail 的所有权，因此需要获取它的引用
            Cons(i, ref tail) => 1 + tail.len(),
            // 空链表的长度为 0
            Nil => 0
        }
    }

    // 返回链表的字符串表现形式，用于打印输出
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // 递归生成字符串
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}


/// .
pub fn ch2_4_5() {
    let a = [1; 5];

    let a = [1, 2, 3, 4, 5];


    let char_str = ['a', 'b', 'c'];
    
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("failed to readline");

    let index: usize = index.trim().parse().expect("not a number");

    let element = a[index];


    println!("{}", element);

    // let a = [String::from("cargo"); 8];

    let a: [String; 8] = core::array::from_fn(|i| String::from("rust")+ i.to_string().as_str());


    println!("{:?}", a);


    let a = [1, 2, 3];
    for i in a.iter() {
        println!("{}", i);

    }

    for i in a.iter().enumerate() {
        println!("{:?}", i);
    }

    for i in 0..a.len() {
        println!("{}", a[i]);
    }

 

}
 */

/**
. for 循环有三种形式
for item in collection 转移所有权 
for item in collection.iter() 不可变引用 等同于 for item in &collection
for item in &mut collection 可变引用 等同于 for item in collection.iter_mut()

由于for 循环可以不使用下标来控制边界，也不需要边界检查，因此性能好
*/
/* pub fn ch2_5() {
    /* let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    let mut a = [1, 2, 3];

    
    for i in &mut a {
        *i = 1 + *i;
    }
    println!("{:?}", a);
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter + 1;
        }
    };
    println!("{:?}", result); */

    /* let mut counter = 0;
    'outer: loop {
        'inner1: loop {
            counter += 1;
            if counter >=20 {
                break 'inner1;
            }
        }
        'inner2: loop {
            if counter >=
        }
    } */
}




enum Direction {
    East,
    West,
    South,
    North,
}
/// .match and iflet
pub fn ch2_6_1() {
    let dir = Direction::East;
    let s = match dir {
        Direction::West => {1},
        Direction::North | Direction::South => {2},
        _ => {3},
    };


    println!("{}", s);

    let s = Some(30);
    match s {
        Some(s) => {
            println!("{}", s);
        },
        None => {},
    }
    println!("{:?}", s);


} */




pub fn ch2_6_2() {
  /*   None::<i32>;
    let o = Option::Some(1);
    if let Some(i) = o {
        println!("{}", i);
    }


    
/*     match o {
        Some(_) => {panic!()},
        None => {},
    }

 */


    let y = 10;
    let x = Some(5);
    /* match x {
        Some(10) => {},
        Some(y) => {panic!()},

        None => {},
    }
 */

    let x = 5;
    match x {
        1..=5 => {},
        _ => {},
    }

    let c = 'c';
    match c {
        'a'..='z' => {},
        'A'..='Z' => {},
        _ => {},
    }

    let message = Message::ChangeColor(Color::Rgb(60, 255, 255));

    match message {
        Message::ChangeColor(Color::Rgb(a @0..=50, b, c))  => { println!("{}", "Module success")},
        _ => {},
    }

    let x = Some(10);
    match x {
        Some(x) if x < 5 => {},
        Some(x) => {},
        None => {},
    }

    let p = Point{x_axis: String::from("xaxis"), y_axis: String::from("yaxis")};
    let p2@Point { x_axis, y_axis } = &p;
    //let Point { x_axis, y_axis }: Point = Point::new(String::from("xaxis"), String::from("yaxis"));


    println!("{:p}", p2);

    let mut v = String::from("hello ");

    let s: &mut String = &mut v;

    let value = s;

    println!("{}", v);
    let r: String = (*v).to_string();
 */
  /*   let a = 5;
    let b = a;

    println!("{} {}", a, b); */



}




/* enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
 }
 
 enum Message {
     Quit,
     Move { x: i32, y: i32 },
     Write(String),
     ChangeColor(Color),
 }



 #[derive(Debug)]
struct Point {
    x_axis: String,
    y_axis: String
 }

impl Point {
    fn new(x_axis: String, y_axis: String) -> Self { Self { x_axis, y_axis } }
} */



pub struct Rectangle {
    x: i32,
    y: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.x * self.y
    }
}

impl Rectangle {
    pub fn new(x: i32, y: i32) -> Self { Self { x, y } }
}


pub fn ch2_7() {
    let r = &Rectangle::new(1, 2);

    let c = TrafficLightColor::Yellow;

    assert_eq!(c.color(), "yellow");

    println!("{:?}",c);
}

#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}



// 为 TrafficLightColor 实现所需的方法
impl TrafficLightColor {
    fn color(&self) -> &str {
        match self {
            TrafficLightColor::Green => {"Green"},
            TrafficLightColor::Yellow => {"yellow"},
            _ => {"Red"},
        }
    }
}

#[derive(Debug)]
struct TE {
    i: i32,
    j: i32,
}

impl TE {
    fn new(i: i32, j: i32) -> Self { Self { i, j } }
}

impl Add<&TE> for &TE {
    type Output = <TE as Add<TE>>::Output;

    fn add(self, rhs: &TE) -> <TE as Add<TE>>::Output {
        TE{
            i: self.i + rhs.i,
            j: self.j + rhs.j,
        }
    }
}


impl Add<TE> for TE {
    type Output = TE;

    fn add(self, rhs: TE) -> Self::Output {
        Self{
            i: self.i + rhs.i,
            j: self.j + rhs.j,
        }
    }
}











pub trait Summary {

    fn summarize(&self) -> String;

}


pub struct Post {
    title: String,
    author: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        self.title.clone() + self.author.as_str()
    }
}


pub fn notify(item: &impl Summary) -> String {
    item.summarize()
    
}

pub fn notify_display(item: &(impl Summary + Display)) -> String {
    item.summarize()
}


pub fn multi_notify<U, T>(item: &T, item2: &U) -> String 
    where T: Display + Summary,
          U: Display + Clone + Summary
{
    item.summarize() + item2.summarize().as_str()

}

impl Summary for i32{
    fn summarize(&self) -> String {
        self.to_string()
    }
}


/// .
pub fn return_summary() -> impl Summary {
    if true {
        1
    }
    else {
        // Post{ title: "".to_string(), author: "".to_string() }
        2
    }

}

pub fn ch2_8() {
    /* let t1 = TE::new(1, 2);
    let t2 = TE::new(2, 3);
    let t = &(&t1 + &t2);
    println!("{:?}", t1);
    println!("{:?}", t);


    let slice = b"hello";

    let c = slice.get(2);

    if let Some(i) = c {}


 */
    let s = multi_notify(&2, &1);
    println!("{}", s);
    println!("{}",format!("{}, {}", s, "123"));

    
    
    return_summary();
    
    let s = "123";
    let s1 = String::from(s);
    let s2 = String::from(&s[0..2]);

    let s = [s1, s2];

    let t = largest(&s);
    println!("{}", t);
    
        
    
    
    let b = Button;
    draw_test(&b);

    let s = Screen::new(vec![Box::new(Button), Box::new(Button), Box::new(Bar), Box::new(Button)]);
    s.draw()
}




/* fn returns_summary (item: bool) -> impl Summary 
    
{   
    if item{
        Box::new(1)
    }
    else {
        Box::new(Post{ author: "".to_string(), title: "".to_string() })
    }

}
 */
pub trait Draw {
    fn draw(&self);
}

pub struct Button;

impl Draw for Button {
    fn draw(&self) {
        println!("{}", "draw for button");
    }
}

pub struct Bar;

impl Draw for Bar {
    fn draw(&self) {
        println!("{}", "draw for bar");
    }
}




fn draw_test(d: &dyn Draw) {
    d.draw()
}


pub struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn new(components: Vec<Box<dyn Draw>>) -> Self { Self { components } }
}

impl Draw for Screen {
    fn draw(&self) {
        for component in self.components.iter() {
            component.draw()
        }
    }
}




fn largest<T> (list: &[T]) -> T
    where T: PartialOrd + Clone
{
    let mut largest = list[0].clone();

    for item in list.iter() {
        if item > &largest {
            largest = item.clone();
        }
    }

    largest
}


struct M {}

fn open_file(filepath: &String) -> core::result::Result<TE, M> {
    Ok(TE{i:1, j:2})
}

fn open_file2(filepath: &String) -> Option<TE> {
    Some(TE{i: 1, j: 2 })
}

struct Point<T> {
    a: T,
    b: T,
}

impl<T> Point<T> {
    fn new(a: T, b: T) -> Self { Self { a, b } }

    fn a(&self) -> &T {
        &self.a
    }

    fn a_mut(&mut self) -> &mut T {
        &mut self.a
    }

    fn set_a(&mut self, a: T) {
        self.a = a;
    }
}

impl<T: std::ops::Add + Add<Output = T>> Add for Point<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point { a: rhs.a + self.a, b: rhs.b + self.b }
    }
}

fn display_array<T: std::fmt::Debug> (arr: &[T]) {
    println!("{:?}", arr);

    
}





pub fn ch2_9() {
    let mut v: Vec<String> = Vec::new();
    let s = &mut v;

    s.push("value".to_string());
    
    let v: Vec<Box<dyn IpAddr>> = vec![Box::new(Ipv4Addr), Box::new(Ipv6Addr)];
    
    for component in v.iter() {
        component.display()
    }
    let mut map = HashMap::new();

    let mut m: HashMap<String, i32> = HashMap::with_capacity(100);

    map.insert("1", 1);

    map.insert("hashmap", 0);

    let s = map.get("1");
    if let Some(i) = s {
        std::mem::drop(*i);
        println!("{:?}", *i);
    }

    println!("{:?}", map.get("1"));

    for (k, v) in &map {
        println!("{}, {}",k, v);
    }

   // map.entry("1").or_insert(default)

    //println!("{:?}", map);




}   

trait IpAddr {
    fn display(&self);
}

struct Ipv4Addr;

impl IpAddr for Ipv4Addr {
    fn display(&self) {
        
    }
}


struct Ipv6Addr;

impl IpAddr for Ipv6Addr {
    fn display(&self) {

    }
}



pub fn ch2_10() {

    let mut values = [1, 2];

    let p = values.as_mut_ptr();

    let u = p as usize;

    println!("{}", u);

    let second = u + 4;

    unsafe{
        println!("{}", *((u + 4) as *mut i32));
    }
   

    let b = 1500;

    
    /* let b2: u8 = match b.try_into() {
        Ok(b1) => {b1},
        Err(e) => {
            println!("{:?}", e);
            0
        }
    }; */

    // let f = File::open(".txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Problem opening the file: {:?}", error)
    //     },
    // };

    let res = open_file3();
    
}



/// .
///
/// # Errors
/// 
/// This function will return an error if .
/// ```
/// rustlearning::chapter_2::ch2::open_file3();
/// 
pub fn open_file3() -> Result<String, Box<dyn error::Error>> {

    let mut s = String::new();
    File::open("path.txt")?.read_to_string(&mut s)?;

    // Ok(s)
    Ok(s)
}

/// `add_one` 返回一个[`Option`]类型
///
/// # Examples
///
/// ```
/// use rustlearning::chapter_2::ch2::add_one;
///
/// assert_eq!(add_one(1), Some(3));
/// ```
pub fn add_one(x: i32) -> Option<i32> {
    Some(x + 1)
}