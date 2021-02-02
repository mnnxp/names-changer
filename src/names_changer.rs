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
    /// assert_eq!(&"TABLE ClientTokensRef IS 'text';".camel_to_snake(),
    /// "TABLE client_tokens_ref IS 'text';");
    /// ```
    fn camel_to_snake(&self) -> Self::Owned;
}

impl NamesChanger for str {
    type Owned = String;

    fn camel_to_snake(&self) -> String {
        let re_word_for_change =
            Regex::new(r"([a-z0-9-_]+[A-Z]+)|([A-Z]+[a-z0-9-_]+)")
                .unwrap();
        let re_word_to_word =
            Regex::new(r"(?P<word1>[a-z]+)(?P<to1>[2])(?P<word2>[_][a-z]+)")
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

                    match Some(change_word.as_str()) {
                        Some(slice) if re_non_any_word.is_match(&change_word) => {
                            change_word = new_words_of_word(&slice, flag);
                        }
                        _ => change_word = change_word.to_snake_case(),
                    };
                    if re_word_to_word.is_match(&change_word) {
                        let new_word = &re_word_to_word.replace(&change_word,
                                            format!("{} _to{}", "$word1", "$word2").as_str());
                        let new_word = new_word.replace(" ", "");
                        change_word = new_word.to_string();
                    }
                } else {
                    change_word = word.to_string();
                }

                change_line.push_str(&change_word);
                flag = true;
            }
            result.push(change_line);
        }

        let mut result:String = result.join("\n");

        if Regex::new(r"[-]").unwrap().is_match(&result) {
            result = result.replace("-", "_");
        }

        result
    }
}

// обработка нескольких слов разделённых спец.знаками
fn new_words_of_word(slice: &str, flag: bool) -> String {
    let re_custom_word =
        Regex::new(r"(?P<not_word1>[\W0-9a-z_]*[_]*)(?P<word1>[a-zA-Z0-9-]*)(?P<not_word2>[_]*[\W0-9a-z_]*[_]*)(?P<word2>[a-zA-Z0-9-]*)(?P<next_text>.*)")
            .unwrap();
    let re_end_word = Regex::new(r"[\W0-9a-z_]+$").unwrap();
    let re_add_char = Regex::new(r"^[\W0-9_]*[a-z]+[\W0-9_]*").unwrap();
    let caps = re_custom_word.captures(slice).unwrap();

    let mut update_word = String::new();

    // обработка первой группы символов
    let not_word1 = &re_custom_word.replace(slice,
                                            caps.name("not_word1").unwrap().as_str());
    update_word.push_str(not_word1);

    if flag && re_add_char.is_match(slice) && re_add_char.is_match(not_word1) {
        update_word.push_str("_");
    }

    // обработка первого слова в полученном отрезке
    let rep_word1 = &re_custom_word.replace(slice,
                                            caps.name("word1").unwrap().as_str());
    update_word.push_str(rep_word1.to_snake_case().as_str());

    // обработка второй группы символов
    let not_word2 = &re_custom_word.replace(slice,
                                            caps.name("not_word2").unwrap().as_str());
    update_word.push_str(not_word2);

    // обработка второго слова в полученном отрезке
    let rep_word2 = &re_custom_word.replace(slice,
                                            caps.name("word2").unwrap().as_str());
    let rep_word2 = match_word(rep_word2);
    update_word.push_str(&rep_word2);

    // обработка конца отрезка, если там есть слово для обработки - функция вызывается рекурсивно
    if Regex::new(r"([a-zA-Z0-9-_]*[A-Z]+[a-zA-Z0-9-_]*)").unwrap().is_match(caps.name("next_text").unwrap().as_str()) {
        let next_text = &re_custom_word.replace(slice,
                                                    caps.name("next_text").unwrap().as_str());

        if Regex::new(r"([a-zA-Z0-9-]+)").unwrap().is_match(next_text) {
            let next_text = match_word(next_text);
            update_word.push_str(&next_text);
        } else {
            update_word.push_str(&next_text);
        }

        // возвращяем конец строки отрезок, который не нужнается в обработке
        if flag && re_end_word.is_match(slice)  {
            let end_word = &re_end_word.captures(slice).unwrap();
            update_word.push_str(&end_word[0]);
        }
    }

    return update_word;
}

fn match_word(text_word: &str) -> String {
    return match text_word {
        x if Regex::new(r"([a-zA-Z0-9-_]*[A-Z]+[a-zA-Z0-9-_]*)").unwrap()
            .is_match(x) => {
            new_words_of_word(x, false)
        },
        _ => text_word.to_string(),
    }
}

#[cfg(test)]
mod mod_tests {
    use super::*;

    #[test]
    fn check_new_words_of_word() {
        let content = "ClientRefA(ClientRefB(ClientRefC((ClientRefE(id)))))";
        let change_content = new_words_of_word(content, true);

        assert_eq!("client_ref_a(client_ref_b(client_ref_c((client_ref_e(id)))))".to_string(),
                   change_content)
    }

    #[test]
    fn check_more_words() {
        let content = ")))ClientRefX11ClientRefA(ClientRefB(ClientRefC((ClientRefE(id))))))@)";
        let change_content = new_words_of_word(content, true);

        assert_eq!(")))client_ref_x11_client_ref_a(client_ref_b(client_ref_c((client_ref_e(id))))))@)".to_string(),
                   change_content)
    }
}