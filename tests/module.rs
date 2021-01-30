#[cfg(test)]
mod tests {
    use names_changer::*;
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

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}