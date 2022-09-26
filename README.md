# daniela
## Para desarrollo
```sql
drop table cronogramas;
drop extension daniela;
create extension daniela;
create table cronogramas(datos CRONOGRAMA);
insert into cronogramas values(CREAR_CRONOGRAMA('Lun: 7:30 - 8:40; Mar: 7:30 - 8:50'));
insert into cronogramas values(CREAR_CRONOGRAMA('Lun: 7:30 - 9:40; Mar: 7:30 - 10:20'));
insert into cronogramas values(CREAR_CRONOGRAMA('Jue: 7:30 - 11:40; Mar: 7:30 - 13:20'));

select * from cronogramas where en_cronograma(datos, LOCALTIMESTAMP);
select * from cronogramas where en_cronograma_permisivo(datos, '2022-09-28 11:33:16');
```
