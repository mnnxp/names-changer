use case::CaseExt;
use regex::Regex;

pub trait NamesChanger {
    type Owned;

    /// Create a new string from a string slice with replacing the first character with its
    /// to ASCII upper case counterpart (if one exists).
    ///
    /// # Examples
    ///
    /// ```
    /// use case::CaseExt;
    ///
    /// assert_eq!(&CaseExt::to_capitalized("stringy string"), "Stringy string");
    /// assert_eq!(&CaseExt::to_capitalized("言語"), "言語");
    /// ```
    fn to_capitalized(&self) -> Self::Owned;

    /// Replaces underscores with dashes in a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use case::CaseExt;
    ///
    /// assert_eq!(&"stringing_in_the_rain".to_dashed(), "stringing-in-the-rain");
    /// ```
    fn to_changer_name(&self) -> Self::Owned;
}

impl NamesChanger for str {
    type Owned = String;

    fn to_capitalized(&self) -> Self::Owned {
        let mut result = String::with_capacity(self.len());

        for (i, c) in self.chars().enumerate() {
            result.push(if i == 0 {
                c.to_ascii_uppercase()
            } else {
                c
            });
        }

        result
    }

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
