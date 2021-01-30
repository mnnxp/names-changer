use heck::SnakeCase;
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
    /// assert_eq!(&"bus WildUnderscoreS oby martinLutherStringJr 言語".camel_to_snake(),
    /// "bus wild_underscore_s oby martin_luther_string_jr 言語");
    /// ```
    fn camel_to_snake(&self) -> Self::Owned;
}

impl NamesChanger for str {
    type Owned = String;

    fn camel_to_snake(&self) -> String {
        let re_word_for_change =
            Regex::new(r"([a-z]?[a-z]*[A-Z]+[a-z]+[A-Z]?[a-zA-Z0-9-]+[^\W])")
                .unwrap();
        let re_word_to_word =
            //Regex::new(r"([\w][2][a-zA-Z])")
            //Regex::new(r"([a-z])([2][_])([a-z])")
            Regex::new(r"(?P<word1>[a-z]+)(?P<to1>[2])(?P<word2>[a-z_]+)")
                .unwrap();
        let re_custom_word =
            Regex::new(r"(?P<nword1>[\W]*)(?P<word1>[a-zA-Z0-9-_]+[a-z]?[a-z]*[A-Z]*[a-z]+[A-Z]?[a-zA-Z0-9-_]*)(?P<nword2>[\W]*)")
                .unwrap();
        let re_non_any_word =
            Regex::new(r"([^a-zA-Z0-9-_])")
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

                    if re_non_any_word.is_match(&change_word) {
                        let rep_word = &re_custom_word.replace(&change_word.as_str(), "$word1");
                        let update_word = format!("{} {}{}", "$nword1", rep_word.to_snake_case(), "$nword2");
                        let new_word = &re_custom_word.replace(
                            &change_word.as_str(),
                            update_word.as_str());
                        let new_word = new_word.replace(" ", "");
                        change_word = new_word.to_string();
                    } else {
                        change_word = change_word.to_snake_case();
                    }

                    // searching "2_" in word received of word "Text2Text"
                    // for change to "text_to_text"
                    if re_word_to_word.is_match(&change_word) {
                        //let rep_word = &re_word_to_word.replace(&change_word.as_str(), "$to1");
                        //change_word = change_word.replace("2_", "_to_");
                        let new_word = &re_word_to_word.replace(&change_word,
                                            format!("{} _to{}", "$word1", "$word2").as_str());
                        let new_word = new_word.replace(" ", "");
                        change_word = new_word.to_string();
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