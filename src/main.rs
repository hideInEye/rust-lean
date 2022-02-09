// use ferris_says::say; // from the previous step
// use std::io::{stdout, BufWriter};

// fn main() {
//     let stdout = stdout();
//     let message = String::from("Hello fellow Rustaceans!");
//     let width = message.chars().count();

//     let mut writer = BufWriter::new(stdout.lock());
//     say(message.as_bytes(), width, &mut writer).unwrap();

//     let s1 = format!(
//         "{1}是个体重{0:<0width$}KG，身高{height:?}cm的矮胖子",
//         115,
//         "肥仔",
//         width = 5,
//         height = 163
//     );
//     println!("s1:{}", s1);
// }
// use std::{collections::HashMap, fs::read_to_string};

// fn main() {
//     let source = read_to_string("./README.md").unwrap();
//     let mut files = HashMap::new();
//     files.insert("README", source.clone());
//     files.insert("README2", source);

//     let files_ref = &mut files;

//     needs_mutable_ref(files_ref);

//     let files_ref2 = &mut files;

//     needs_mutable_ref(files_ref2);
// }

// fn needs_mutable_ref(_map: &mut HashMap<&str, String>) {}

// use std::collections::HashMap;

// fn main() {
//     let mut map = HashMap::new();
//     map.insert("小猪", "爱吃");
//     let value = map.values();
//     let keys = map.keys();
//     println!("{:?}", map.get("小猪"));
//     println!("{:?}",keys);
//     println!("value:{:?}", value);
// }

use std::fmt::Display;

fn main() {
  let traffic_light = TrafficLight::new();
  let house_light = HouseLight::new();

  print_state(&traffic_light);
  print_state(&house_light);
}

fn print_state(light: &impl Light) {
  println!("{}'s state is : {:?}", light.get_name(), light.get_state());
}

trait Light {
  fn get_name(&self) -> &str;
  fn get_state(&self) -> &dyn std::fmt::Debug;
}

impl Light for HouseLight {
  fn get_name(&self) -> &str {
    "House light"
  }

  fn get_state(&self) -> &dyn std::fmt::Debug {
    &self.on
  }
}

impl Light for TrafficLight {
  fn get_name(&self) -> &str {
    "Traffic light"
  }

  fn get_state(&self) -> &dyn std::fmt::Debug {
    &self.color
  }
}

impl std::fmt::Display for TrafficLight {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Traffic light is {}", self.color)
  }
}

#[derive(Debug)]
struct TrafficLight {
  color: TrafficLightColor,
}

impl TrafficLight {
  pub fn new() -> Self {
    Self {
      color: TrafficLightColor::Red,
    }
  }
}

#[derive(Debug)]
enum TrafficLightColor {
  Red,
  Yellow,
  Green,
}

impl Display for TrafficLightColor {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let color_string = match self {
      TrafficLightColor::Green => "green",
      TrafficLightColor::Red => "red",
      TrafficLightColor::Yellow => "yellow",
    };
    write!(f, "{}", color_string)
  }
}

#[derive(Debug)]
struct HouseLight {
  on: bool,
}

impl Display for HouseLight {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Houselight is {}", if self.on { "on" } else { "off" })
  }
}

impl HouseLight {
  pub fn new() -> Self {
    Self { on: false }
  }
}