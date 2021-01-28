mod temp;

use case::CaseExt;
use regex::Regex;

pub fn camel_to_snake(contents: &str) -> String {
    let re =
        Regex::new(r"([a-z]?[a-z]*[A-Z]+[a-z]+([A-Z]?[a-z]*[\w]){0,10})")
            .unwrap();
    let mut result = Vec::new();

    for line in contents.lines() {
        let mut f = false;
        let mut w = String::new();
        let mut l = String::new();
        for word in line.split(' ') {
            if f {
                l.push(' ');
            }

            if re.is_match(word) {
                w = word.parse().unwrap();
                w = w.to_snake();
                //println!("TRUE: {}", w);

            } else {
                w = word.to_string();
            }
            l.push_str(&w);
            f = true;
        }
        //println!("TRUE: {}", l);
        //println!("With text:\n{:#?}", result);
        result.push(l);
    }

    let result:String = result.join("\n");

    result
}

pub fn next_camel_to_snake(contents: &str) -> String {


    let result = String::from("\n");

    result
}