#[cfg(test)]
mod tests {
    use names_changer::*;
    #[test]
    fn test_name_change() {
        let content = "TABLE ClientTokensRef IS 'text';";
        let change_content = content.camel_to_snake();

        assert_eq!("TABLE client_tokens_ref IS 'text';", change_content)
    }

    #[test]
    fn check_return_data() {
        let contents = "\
/* перечень типов изменений */
CREATE TABLE TypeOfChangeRef (
  id integer NOT NULL, /* id типа изменения */
  TypeOfChange VARCHAR(100) NOT NULL, /*наименование изменения */
  idNameCAD integer NOT NULL DEFAULT '0', /* САПР «по умолчанию» (для быстрой загрузки данных) */
  CONSTRAINT TypeOfChangeRef_pk PRIMARY KEY (id)
);
ALTER TABLE FileRef ADD CONSTRAINT FileRef_fk1 FOREIGN KEY (idClientCreate) REFERENCES ClientRef(id);
COMMENT ON TABLE Business2Client IS 'commented';";
        let good_content = "\
/* перечень типов изменений */
CREATE TABLE type_of_change_ref (
  id integer NOT NULL, /* id типа изменения */
  type_of_change VARCHAR(100) NOT NULL, /*наименование изменения */
  id_name_cad integer NOT NULL DEFAULT '0', /* САПР «по умолчанию» (для быстрой загрузки данных) */
  CONSTRAINT type_of_change_ref_pk PRIMARY KEY (id)
);
ALTER TABLE file_ref ADD CONSTRAINT file_ref_fk1 FOREIGN KEY (id_client_create) REFERENCES client_ref(id);
COMMENT ON TABLE business_to_client IS 'commented';";


        assert_eq!(good_content, NamesChanger::camel_to_snake(contents));
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}