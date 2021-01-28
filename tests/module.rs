#[cfg(test)]
mod tests {
    use names_changer::*;



    #[test]
    fn check_return_data() {
        let contents = "\
/* перечень типов изменений */
CREATE TABLE TypeOfChangeRef (
  id integer NOT NULL, /* id типа изменения */
  TypeOfChange VARCHAR(100) NOT NULL, /*наименование изменения */
  CONSTRAINT TypeOfChangeRef_pk PRIMARY KEY (id)
);
ALTER TABLE FileRef ADD CONSTRAINT FileRef_fk0 FOREIGN KEY (idFile) REFERENCES FileRef(id);
ALTER TABLE FileRef ADD CONSTRAINT FileRef_fk1 FOREIGN KEY (idClientCreate) REFERENCES ClientRef(id);
ALTER TABLE FileRef ADD CONSTRAINT FileRef_fk2 FOREIGN KEY (idExt) REFERENCES ExtensionRef(id);
COMMENT ON TABLE ClientTokensRef IS 'токен сессии клиента';
COMMENT ON COLUMN ClientTokensRef.id IS 'id токена';
COMMENT ON COLUMN ClientTokensRef.idClient IS 'идентификатор пользователя';";
        let good_content = "\
/* перечень типов изменений */
CREATE TABLE type_of_change_ref (
  id integer NOT NULL, /* id типа изменения */
  type_of_change VARCHAR(100) NOT NULL, /*наименование изменения */
  CONSTRAINT type_of_change_ref_pk PRIMARY KEY (id)
);
ALTER TABLE file_ref ADD CONSTRAINT file_ref_fk0 FOREIGN KEY (id_file) REFERENCES file_ref(id);
ALTER TABLE file_ref ADD CONSTRAINT file_ref_fk1 FOREIGN KEY (id_client_create) REFERENCES client_ref(id);
ALTER TABLE file_ref ADD CONSTRAINT file_ref_fk2 FOREIGN KEY (id_ext) REFERENCES extension_ref(id);
COMMENT ON TABLE client_tokens_ref IS 'токен сессии клиента';
COMMENT ON COLUMN client_tokens_ref.id IS 'id токена';
COMMENT ON COLUMN client_tokens_ref.id_client IS 'идентификатор пользователя';";


        assert_eq!(good_content, camel_to_snake(&contents));
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}