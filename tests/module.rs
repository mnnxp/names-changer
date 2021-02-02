#[cfg(test)]
mod tests {
    use names_changer::*;

    macro_rules! t {
        ($t:ident : $s1:expr => $s2:expr) => {
            #[test]
            fn $t() {
                assert_eq!($s1.camel_to_snake(), $s2)
            }
        }
    }

    t!(test1: "CamelCase" => "camel_case");
    // test2 different from typical camel to snake
    t!(test2: "idExt idEx dE" => "id_ext id_ex d_e");
    t!(test3: "MixedUP CamelCase, with some Spaces" => "mixed_up camel_case, with some spaces");
    // test4 different from typical camel to snake
    t!(test4: "mixed_up_ snake_case with some _spaces" => "mixed_up_ snake_case with some _spaces");
    t!(test5: "kebab-case" => "kebab_case");
    t!(test6: "SHOUTY_SNAKE_CASE" => "shouty_snake_case");
    t!(test7: "snake_case" => "snake_case");
    // test8 different from typical camel to snake
    t!(test8: "this-contains_ ALLKinds OfWord_Boundaries" => "this_contains_ all_kinds of_word_boundaries");
    // test9 different from typical camel to snake
    t!(test9: "XΣXΣ baﬄe" => "XΣXΣ baﬄe");
    t!(test10: "XMLHttpRequest" => "xml_http_request");
    t!(test11: "FIELD_NAME11" => "field_name11");
    t!(test12: "99BOTTLES" => "99bottles");
    t!(test13: "FieldNamE11" => "field_nam_e11");

    t!(test14: "abc123def456" => "abc123def456");
    t!(test16: "abc123DEF456" => "abc123_def456");
    t!(test17: "abc123Def456" => "abc123_def456");
    t!(test18: "abc123DEf456" => "abc123_d_ef456");
    t!(test19: "ABC123def456" => "abc123def456");
    t!(test20: "ABC123DEF456" => "abc123def456");
    t!(test21: "ABC123Def456" => "abc123_def456");
    t!(test22: "ABC123DEf456" => "abc123d_ef456");
    t!(test23: "ABC123dEEf456FOO" => "abc123d_e_ef456_foo");
    t!(test24: "abcDEF" => "abc_def");
    t!(test25: "ABcDE" => "a_bc_de");

    #[test]
    fn check_small_word() {
        let content = "idExt idEx dE";
        let change_content = content.camel_to_snake();

        assert_eq!("id_ext id_ex d_e", change_content)
    }

    #[test]
    fn check_classic_camel_word_change() {
        let content = "TABLE ClientTokensRef IS 'text';";
        let change_content = content.camel_to_snake();

        assert_eq!("TABLE client_tokens_ref IS 'text';", change_content)
    }

    #[test]
    fn check_change_word_with_abbreviation() {
        let content = "CONForSTRChangerRAINT TABLE idNameCAD IS 'text';";
        let change_content = content.camel_to_snake();

        assert_eq!("con_for_str_changer_raint TABLE id_name_cad IS 'text';", change_content)
    }

    #[test]
    fn check_change_word_with_to() {
        let content = "TABLE Client2KentsRef IS 'text';";
        let change_content = content.camel_to_snake();

        assert_eq!("TABLE client_to_kents_ref IS 'text';", change_content)
    }

    #[test]
    fn check_change_word_with_abbreviation_and_to() {
        let content = "TABLE Name-CAD Client2CADRef IS (idCADNameCAD!) 'text';";
        let change_content = content.camel_to_snake();

        assert_eq!("TABLE name_cad client_to_cad_ref IS (id_cad_name_cad!) 'text';", change_content)
    }


    #[test]
    fn check_change_word_with_abbreviation_and_other_chars() {
        let content = "ALTER TABLE FileRef @idCADNameCAD@ ADD CONSTRAINT FileRef_fk1 FOREIGN KEY (idClientCreate) REFERENCES";
        let change_content = content.camel_to_snake();

        assert_eq!("ALTER TABLE file_ref @id_cad_name_cad@ ADD CONSTRAINT file_ref_fk1 FOREIGN KEY (id_client_create) REFERENCES", change_content)
    }

    #[test]
    fn check_change_words_in_word() {
        let content = "ALTER ClientRef(id) REFERENCES";
        let change_content = content.camel_to_snake();

        assert_eq!("ALTER client_ref(id) REFERENCES", change_content)
    }

    #[test]
    fn check_change_full_data() {
        let contents = "\
/* перечень типов изменений */
CREATE TABLE TypeOfChangeRef2 (
  id integer NOT NULL, /* id типа изменения */
  2_TypeOfChange VARCHAR(100) NOT NULL, /*наименование изменения */
  idNameCAD integer NOT NULL DEFAULT '0', /* САПР «по умолчанию» (для быстрой загрузки данных) */
  CONSTRAINT TypeOfChangeRef_pk PRIMARY KEY (id)
);
ALTER TABLE FileRef @idCADNameCAD@ ADD CONSTRAINT FileRef_fk1 FOREIGN KEY (idClientCreate) REFERENCES ClientRef(id);
COMMENT (idCADNameCAD) ON TABLE Business2Client IS 'commented';";
        let good_content = "\
/* перечень типов изменений */
CREATE TABLE type_of_change_ref2 (
  id integer NOT NULL, /* id типа изменения */
  2_type_of_change VARCHAR(100) NOT NULL, /*наименование изменения */
  id_name_cad integer NOT NULL DEFAULT '0', /* САПР «по умолчанию» (для быстрой загрузки данных) */
  CONSTRAINT type_of_change_ref_pk PRIMARY KEY (id)
);
ALTER TABLE file_ref @id_cad_name_cad@ ADD CONSTRAINT file_ref_fk1 FOREIGN KEY (id_client_create) REFERENCES client_ref(id);
COMMENT (id_cad_name_cad) ON TABLE business_to_client IS 'commented';";


        assert_eq!(good_content, NamesChanger::camel_to_snake(contents));
    }

}