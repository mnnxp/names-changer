# names-changer
Convert a names of sql schemes from camelcase to snake case.

 Taking data as str.
 This crate `#[names_changer]` provides trait method `.camel_to_snack()` that convert a names from camel case to snake case.
 The trait searches for words matching the pattern and converts them to snake case.

 # Getting Started

 First of all you have to add this dependency to your `Cargo.toml`:

 ```toml
 [dev-dependencies]
 names-changer = "0.1.0"
 ```

 Additionally, you have to import the procedural macro with `use` statement:

 ```rust
 use names_changer::NamesChanger;
 ```

 # Example usage:

 ```rust
 #[cfg(test)]
 mod tests {
     use names_changer::NamesChanger;

     // Not needed for this example, but useful in general
     use super::*;

     #[test]
     fn test_name_change() {
         let content = "TABLE ClientTokensRef IS 'text';";
         let change_content = content.camel_to_snack();

         assert_eq!("TABLE client_tokens_ref IS 'text';", change_content)
     }
 }
 ```
