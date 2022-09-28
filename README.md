# daniela
## Para desarrollo
Para correr la extensión en un entorno de prueba:
```bash
cargo pgx run pg14
```

Los siguiente muestra una prueba de su funcionalidad:
```sql
drop table cronogramas;
drop extension daniela;
create extension daniela;
create table cronogramas(datos CRONOGRAMA);
insert into cronogramas values(CREAR_CRONOGRAMA('Lun: 7:30 - 8:40, Mar: 7:30 - 8:50'));
insert into cronogramas values(CREAR_CRONOGRAMA('Lun: 7:30 - 9:40, Mar: 7:30 - 10:20'));
insert into cronogramas values(CREAR_CRONOGRAMA('Jue: 7:30 - 11:40, Mar: 7:30 - 13:20'));

-- Se puede buscar contra la hora actual
select * from cronogramas where en_cronograma(datos, LOCALTIMESTAMP);
-- 
select * from cronogramas where en_cronograma(datos, '2022-09-29 11:33:16');
select * from cronogramas where en_cronograma(datos, '2022-09-29 11:33:16');
select * from cronogramas where en_cronograma_permisivo(datos, '2022-09-28 11:33:16');
-- Como la precisión esta en segundos, pueden omitirse en la fecha a comparar
select * from cronogramas where en_cronograma(datos, '2022-09-29 11:33');
```
