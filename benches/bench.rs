#![feature(test)]

extern crate test;

use names_changer::*;
use test::Bencher;
use std::fs;

#[bench]
fn bench_camel_to_snake_of_str(b: &mut Bencher) {
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

    b.iter(|| NamesChanger::camel_to_snake(contents));
}