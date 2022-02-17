

// fn main() {
//     let mut mobile = "iphone";
//     println!("{}", mobile);

//     std::println!("{}", Rectangle::new(1, 1).set_height(10))
// }

// pub struct Rectangle {
//     height: usize,
//     width: usize,
// }

// impl Rectangle {
//     pub fn new(height: usize, width: usize) -> Self {
//         Self { height, width }
//     }

//     pub fn height(&self) -> usize {
//         self.height
//     }

//     pub fn set_height(&mut self, height: usize) -> usize {
//         self.height = height;
//         self.height()
//     }

//     pub fn parts(self) -> (usize, usize) {
//         (self.height, self.width)
//     }
// }

// use std::{collections::HashMap, fs::read_to_string};

// fn main() {
//     let source = read_to_string("./README.md").unwrap();
//     println!("{}",source);
//     let mut files = HashMap::new();
//     files.insert("README", source.clone());
//     files.insert("README2", source);

//     let files_ref = &mut files;
//     print_borrowed_map(files_ref);

//     let files_ref2 = &mut files;
//     print_borrowed_map(files_ref2);
// }

// fn print_borrowed_map(map:&mut HashMap<&str,String>){
//   println!("{:?}",map)
// }

// pub mod test_mod;

// use test_mod::hosting::add_to_waitlist;
// fn main() {
//     add_to_waitlist();
//     let light = TrafficLight::new();
//     println!("{}", light);
//     println!("{:?}", light.color);
// }

// impl std::fmt::Display for TrafficLight {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Traffic light is {}", self.color)
//     }
// }

// #[derive(Debug)]
// pub struct TrafficLight {
//     color: String,
// }

// impl TrafficLight {
//     pub fn new() -> Self {
//         Self {
//             color: "blue".to_owned(),
//         }
//     }

//     pub fn get_color(&self) -> String {
//         self.color.clone()
//     }
// }

// use std::str::FromStr;
// fn main() {
//     let test_slice = "String slice assigned to variable".clone();
//     print_str(test_slice);
//     let rs = TestRs::new();
//     println!("{:?}", rs.get_color());

//     let ip_address = std::net::Ipv4Addr::from_str("127.0.0.1").unwrap();
//     println!("{:?}", ip_address);
//     // needs_string(ip_address);
// }

// // fn needs_string<T:ToString>(almost_string:T){
// //     let real_string = almost_string.to_string();
// //     println!("{}",real_string)
// // }

// // fn needs_string(almost_string: impl ToString) {
// //     let real_string = almost_string.to_string();
// //     println!("{}", real_string)
// // }

// fn print_str(msg: &str) {
//     println!("{}", msg)
// }

// #[derive(Debug)]

// struct TestRs {
//     color: String,
// }

// impl TestRs {
//     pub fn new() -> Self {
//         Self {
//             color: "blue".to_owned(),
//         }
//     }
//     pub fn get_color(&self) -> String {
//         (self.color).clone()
//     }
// // }
// #![allow(unused)]
// use std::collections::HashMap;
// mod test_mod;
// fn main() {
//     let mut number = vec![1, 2, 3, 4, 5];
//     number.push(6);
//     println!("{:?}", number);
//     let max = 10;
//     for i in 0..max {
//         println!("{}", i)
//     }

//     let mut obj = HashMap::from([("key1", "value1"), ("key2", "value2")]);
//     obj.insert("key3", "value3");
//     println!("{:?}", obj.get("key1"));
//     for prop in obj.keys() {
//         println!("{}:{}", prop, obj.get(prop).unwrap());
//     }

//     struct Worker {
//         data: Vec<&'static str>,
//     }

//     impl Worker {
//         fn doWork(&mut self) -> Option<&'static str> {
//             self.data.pop()
//         }
//     }

//     let mut obj = Worker {
//         data: vec!["a", "b", "c"],
//     };
//     while let Some(data) = obj.doWork() {
//         println!("{}", data)
//     }

//     // 迭代器
//     let list = [1, 2, 3];
//     let number: Vec<i32> = list.iter().map(|num| *num * 2).collect();
//     println!("{:?}", number);
//     let list_filter:Vec<_> = list.iter().filter(|x| *x % 2 == 0).collect();
//     println!("list_filter:{:?}", list_filter);
//     let list_find = list.iter().find(|x| *x % 2 == 0);
//     println!("list_find:{:?}", list_find.unwrap());
//     list.iter().for_each(|x|println!("{}",x));
//     run()
// }

// fn run (){
//     println!("{:?}",crate::test_mod::Rectangle::new(10,10).height())

// }

// use tokio::{
//     io::{AsyncReadExt, AsyncWriteExt},
//     net::TcpListener,
// };

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let listener = TcpListener::bind("127.0.0.1:8000").await?;
//     println!("created stream:{:?}",listener);

//     loop {
//         let (mut socket, _) = listener.accept().await?;
//         println!("{:?}",socket);
//         tokio::spawn(async move {
//             let mut buf = [0; 1024];

//             loop {
//                 let n = match socket.read(&mut buf).await {
//                     Ok(n) if n == 0 => return,
//                     Ok(n) => n,
//                     Err(err) => {
//                         eprintln!("failed to read from socket; err={:?}", err);
//                         return;
//                     }
//                 };
//                 if let Err(e) = socket.write_all(&buf[0..n]).await {
//                     eprintln!("failed to write to socket; err = {:?}", e);
//                     return;
//                 }
//             }
//         });
//     }
// }

// use tokio::{
//     io::{AsyncReadExt, AsyncWriteExt},
//     net::TcpListener,
// };

// #[tokio::main]
// async fn main (){
//     println!("one");
//     let future = prints_two().await;
//     println!("three");
// }

// async fn prints_two (){
//     println!("two");
// }

// use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let resp = reqwest::blocking::get("https://httpbin.org/ip")?.json::<HashMap<String,String>>()?;
    // println!("{:#?}",resp);
    // Ok(())
    let resp = reqwest::get("http://zanghaibin.press:8001/")
        .await
        ?.json()
        .await?;
    println!("{:#?}", resp);
    Ok(())

}
