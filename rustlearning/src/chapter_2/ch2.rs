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


use std::{string, ops::{Add, Deref}, str::from_utf8, fmt::Display, time::SystemTime, alloc::System};



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
    
    /* 
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




enum Pokersult {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

pub fn ch2_4_4() {

}

