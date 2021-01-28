use case::CaseExt;
use regex::Regex;

pub trait NamesChanger {
    type Owned;

    /// Convert a names of sql file from camel case to snake case.
    ///
    /// # Examples
    ///
    /// ```
    /// use names_changer::NamesChanger;
    ///
    /// assert_eq!(&"bus WildUnderscoreS oby martinLutherStringJr 言語".to_changer_name(), "bus wild_underscore_s oby martin_luther_string_jr 言語");
    /// ```
    fn to_changer_name(&self) -> Self::Owned;
}

impl NamesChanger for str {
    type Owned = String;

    fn to_changer_name(&self) -> String {
        let re =
            Regex::new(r"([a-z]?[a-z]*[A-Z]+[a-z]+([A-Z]?[a-z]*[\w]){0,10})")
                .unwrap();
        let mut result = Vec::new();

        for line in self.lines() {
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
}
