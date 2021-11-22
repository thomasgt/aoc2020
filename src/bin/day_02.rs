use std::{
    env::args,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    str::FromStr,
};

#[derive(Debug, PartialEq)]
struct Rule {
    c: char,
    i0: usize,
    i1: usize,
}

impl Rule {
    fn check0(&self, password: &str) -> bool {
        let count = password.matches(self.c).count();
        self.i0 <= count && count <= self.i1
    }

    fn check1(&self, password: &str) -> bool {
        let mut chars = password.chars().skip(self.i0 - 1);
        let c0 = chars.next();
        let c1 = chars.skip(self.i1 - self.i0 - 1).next();

        match (c0, c1) {
            (Some(cc0), Some(cc1)) => {
                (cc0 == self.c || cc1 == self.c) && !( cc0 == self.c && cc1 == self.c)
            }
            _ => false,
        }
    }
}

impl FromStr for Rule {
    type Err = Box<dyn Error>;

    fn from_str(rule_str: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = rule_str.split(&[' ', '-'][..]).collect();
        let min: usize = v.get(0).ok_or("missing min")?.parse::<usize>()?;
        let max: usize = v.get(1).ok_or("missing max")?.parse::<usize>()?;
        let c: char = v.get(2).ok_or("missing character")?.parse::<char>()?;

        Ok(Rule {
            c,
            i0: min,
            i1: max,
        })
    }
}

#[derive(Debug, PartialEq)]
struct Record {
    rule: Rule,
    password: String,
}

impl FromStr for Record {
    type Err = Box<dyn Error>;

    fn from_str(record_str: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = record_str.split(':').map(|x| x.trim()).collect();

        let rule = Rule::from_str(v.get(0).ok_or("missing rule")?)?;
        let password = v.get(1).ok_or("missing password")?;
        Ok(Record {
            rule,
            password: password.to_string(),
        })
    }
}

fn main() {
    let args: Vec<String> = args().collect();

    let input_path = Path::new(&args[1]);

    let f = File::open(input_path).expect("failed to open input file");
    let r = BufReader::new(f);

    let records: Vec<Record> = r
        .lines()
        .map(|x| {
            x.expect("failed to read line")
                .parse::<Record>()
                .expect("failed to parse rule")
        })
        .collect();

    let n0 = records
        .iter()
        .map(|x| x.rule.check0(&x.password) as i32)
        .sum::<i32>();
    let n1 = records
        .iter()
        .map(|x| x.rule.check1(&x.password) as i32)
        .sum::<i32>();

    println!("number of valid passwords based on rule 1: {}", n0);
    println!("number of valid passwords based on rule 2: {}", n1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_rules() {
        assert_eq!(
            Rule {
                c: 'c',
                i0: 0,
                i1: 1
            },
            Rule::from_str("0-1 c").unwrap()
        );
        assert_ne!(
            Rule {
                c: 'a',
                i0: 0,
                i1: 1
            },
            Rule::from_str("0-1 c").unwrap()
        );
        assert_ne!(
            Rule {
                c: 'c',
                i0: 1,
                i1: 1
            },
            Rule::from_str("0-1 c").unwrap()
        );
        assert_ne!(
            Rule {
                c: 'c',
                i0: 0,
                i1: 2
            },
            Rule::from_str("0-1 c").unwrap()
        );

        assert_eq!(
            Rule {
                c: 'z',
                i0: 10,
                i1: 12
            },
            Rule::from_str("10-12 z").unwrap()
        );
    }

    #[test]
    fn parse_invalid_rules() {
        assert!(Rule::from_str("xyz").is_err());
        assert!(Rule::from_str("").is_err());
        assert!(Rule::from_str("12-s 2").is_err());
        assert!(Rule::from_str("12-12 12").is_err());
    }

    #[test]
    fn parse_example_records() {
        let r0 = Record::from_str("1-3 a: abcde").unwrap();
        assert_eq!(
            Record {
                rule: Rule {
                    c: 'a',
                    i0: 1,
                    i1: 3
                },
                password: "abcde".to_owned()
            },
            r0
        );
        assert!(r0.rule.check0(&r0.password));
        assert!(r0.rule.check1(&r0.password));

        let r1 = Record::from_str("1-3 b: cdefg").unwrap();
        assert_eq!(
            Record {
                rule: Rule {
                    c: 'b',
                    i0: 1,
                    i1: 3
                },
                password: "cdefg".to_owned()
            },
            r1
        );
        assert!(!r1.rule.check0(&r1.password));
        assert!(!r1.rule.check1(&r1.password));

        let r2 = Record::from_str("2-9 c: ccccccccc").unwrap();
        assert_eq!(
            Record {
                rule: Rule {
                    c: 'c',
                    i0: 2,
                    i1: 9
                },
                password: "ccccccccc".to_owned()
            },
            r2
        );
        assert!(r2.rule.check0(&r2.password));
        assert!(!r2.rule.check1(&r2.password));
    }
}
