use case::CaseExt;
use regex::Regex;

pub trait NamesChanger {
    type Owned;

    /// Convert a names of sql schemes from camelcase to snake case.
    ///
    /// # Examples
    ///
    /// ```
    /// use names_changer::NamesChanger;
    ///
    /// assert_eq!(&"bus WildUnderscoreS oby martinLutherStringJr 言語".camel_to_snake(), "bus wild_underscore_s oby martin_luther_string_jr 言語");
    /// ```
    fn camel_to_snake(&self) -> Self::Owned;
}

impl NamesChanger for str {
    type Owned = String;

    fn camel_to_snake(&self) -> String {
        let re_word_for_change =
            Regex::new(r"([a-z]?[a-z]*[A-Z]+[a-z]+([A-Z]?[a-z]*[\w]){0,10})")
                .unwrap();
        let re_word_with_abbreviation =
            Regex::new(r"(\w(?<=[^a-z])(?<=[A-Z].)[A-Z]+)")
                .unwrap();
        let re_word_to_word =
            Regex::new(r"(?<=[a-zA-Z])([2][_](?=\w))")
                .unwrap();
        let mut result = Vec::new();

        for line in self.lines() {
            let mut flag = false;
            let mut change_word = String::new();
            let mut change_line = String::new();
            for word in line.split(' ') {
                if flag {
                    change_line.push(' ');
                }

                if re_word_for_change.is_match(word) {
                    change_word = word.parse().unwrap();


                    change_word = change_word.to_snake();

                    // searching "2_" in word received of word "Text2Text"
                    // for change to "text_to_text"
                    if re_word_to_word.is_match(&change_word) {
                        change_word = change_word.replace("2_", "_to_");
                    }
                    //println!("TRUE: {}", w);

                } else {
                    change_word = word.to_string();
                }
                change_line.push_str(&change_word);
                flag = true;
            }
            //println!("TRUE: {}", l);
            //println!("With text:\n{:#?}", result);
            result.push(change_line);
        }

        let result:String = result.join("\n");

        result
    }
}