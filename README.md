names-changer
=====
Convert a names of sql schemes from camelcase to snake case.

 Taking data as str.
 This crate `#[names_changer]` provides trait method `.camel_to_snake()` that convert a names from camel case to snake case.
 The trait searches for words matching the pattern and converts them to snake case.

### Getting Started

 First of all you have to add this dependency to your `Cargo.toml`:

 ```toml
 [dev-dependencies]
 names-changer = "0.2.1"
 ```

 Additionally, you have to import the procedural macro with `use` statement:

 ```rust
 use names_changer::NamesChanger;
 ```

### Example usage:

 ```rust
 #[cfg(test)]
mod tests {
    use names_changer::NamesChanger;

    // Not needed for this example, but useful in general
    use super::*;

    #[test]
    fn test_name_change() {
        let content = "TABLE ClientTokensRef IS 'text';";
        let change_content = content.camel_to_snake();

        assert_eq!("TABLE client_tokens_ref IS 'text';", change_content)
    }
}
 ```
### Why is it?
This for update old sql schems with names include of upper case e.g.

### What's new
0.2.1
  - fixed bug with skipping small words, i.e. "idExt", "idEx", "dE".
0.2.0
  - fixed bug with method name
  - added recursive processing of segments without spaces: 
from "(ClientRefA (ClientRefB (ClientRefC ((ClientRefE (id)))))" 
we get "(client_ref_a (client_ref_b (client_ref_c ((client_ref_e (id)))))"
  - added tests
  - ```case``` crate (the have problems with abbreviations) replaced with ```heck```

Cons: requires a lot of resources, not optimized.

### Todos
 - fix warning
 - optimize code
 - add asynchronous processing?


### License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)
