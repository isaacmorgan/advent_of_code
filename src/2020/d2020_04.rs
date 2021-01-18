use crate::tools;
static FNAME: &str = "./input/2020/2020-04.txt";
static FNAME1: &str = "./input/2020/2020-04-01.txt";
static FNAME2: &str = "./input/2020/2020-04-02.txt";
pub fn main() {
  part1();
  part2();
}

#[derive(Debug, Clone)]
struct Passport {
  byr: String,
  iyr: String,
  eyr: String,
  hgt: String,
  hcl: String,
  ecl: String,
  pid: String,
  cid: String,
}

fn part1() {
  let p = load(FNAME);
  let cnt = p.iter().filter(|x|
    !x.byr.is_empty()
      && !x.ecl.is_empty()
      && !x.eyr.is_empty()
      && !x.hcl.is_empty()
      && !x.hgt.is_empty()
      && !x.iyr.is_empty()
      && !x.pid.is_empty()).count();
  println!("Number of valid passports: {}", cnt);
}

fn part2() {
  let p = load(FNAME);
  let cnt = p.iter()
    .filter(|x| validate(*x))
    .count();
  println!("Number of valid passports: {}", cnt);
}

impl Passport {
  fn new() -> Passport {
    Passport {
      byr: "".to_string(),
      iyr: "".to_string(),
      eyr: "".to_string(),
      hgt: "".to_string(),
      hcl: "".to_string(),
      ecl: "".to_string(),
      pid: "".to_string(),
      cid: "".to_string()
    }
  }
}

fn load(fname: &str) -> Vec<Passport> {
  let input = tools::load(fname);
  let mut passports = Vec::new();
  let mut p = Passport::new();
  for line in &input {
    if line.is_empty() {
      passports.push(p.clone());
      p = Passport::new();
    }
    line.split_whitespace().for_each(|x| {
      let mut y = x.split(":");
      let k = y.next().unwrap();
      let v = y.next().unwrap();
      match k {
        "byr" => p.byr = v.to_string(),
        "iyr" => p.iyr = v.to_string(),
        "eyr" => p.eyr = v.to_string(),
        "hgt" => p.hgt = v.to_string(),
        "hcl" => p.hcl = v.to_string(),
        "ecl" => p.ecl = v.to_string(),
        "pid" => p.pid = v.to_string(),
        "cid" => p.cid = v.to_string(),
        _ => (),
      }
    });
  }
  passports.push(p);
  return passports;
}

fn validate(pass: &Passport) -> bool {
  return validate_year(&pass.byr, 1920, 2002)
    && validate_year(&pass.iyr, 2010, 2020)
    && validate_year(&pass.eyr, 2020, 2030)
    && validate_height(&pass.hgt)
    && validate_hair(&pass.hcl)
    && validate_eye(&pass.ecl)
    && validate_passport_id(&pass.pid);
}

fn validate_year(s: &str, min: i32, max: i32) -> bool {
  let re = regex::Regex::new(r"^\d{4}$").unwrap();
  if re.is_match(s) {
    let n = s.parse::<i32>().unwrap();
    if n >= min && n <= max {
      return true;
    }
  }
  // println!("Failed validate year: {:?}", s);
  return false;
}

fn validate_height(s: &str) -> bool {
  let re = regex::Regex::new(r"^\d+(?:cm)|(?:in)$").unwrap();
  if re.is_match(s) {
    let n: i32 = s[..s.len()-2].parse().unwrap();
    if s.ends_with("cm") && n >= 150 && n <= 193 {
      return true;
    } else if s.ends_with("in") && n >= 59 && n <= 76 {
      return true;
    }
  }
  // println!("Failed validate height: {:?}", s);
  return false;
}

fn validate_hair(s: &str) -> bool {
  let re = regex::Regex::new(r"^#[0-9a-f]{6}$").unwrap();
  // if !re.is_match(s) {
  //   println!("Failed validate hair: {:?}", s);
  // }
  return re.is_match(s);
}

fn validate_eye(s: &str) -> bool {
  // if !["amb", "blu", "brn", "gry", "hzl", "oth"].contains(&s) {
  //   println!("Failed validate eyes: {:?}", s);
  // }
  return ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&s);
}

fn validate_passport_id(s: &str) -> bool {
  let re = regex::Regex::new(r"^\d{9}$").unwrap();
  // if !re.is_match(s) {
  //   println!("Failed validate pid: {:?}", s);
  // }
  return re.is_match(s);
}
