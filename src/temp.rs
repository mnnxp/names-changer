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
    fn re_next_changer(&self) -> Self::Owned;
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

    fn re_next_changer(&self) -> String {
        let result = String::with_capacity(self.len());
        result
    }
}
