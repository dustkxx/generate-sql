set @exist_check := (
  select count(*) from information_schema.tables
  where table_name='@tableName@'
  and table_schema= database()
) ;
set @sqlstmt := if(@exist_check<1,'create table `@tableName@` (
    `urid` varchar(32) collate utf8_bin  not null comment ''主键'',
  @tableColumn@    `createdon` datetime default current_timestamp not null comment ''创建日期'',
    `createdby` varchar (32) collate utf8_bin not null comment ''创建人'',
    `lastmodifiedon` datetime  default current_timestamp not null comment ''修改时间'',
    `lastmodifiedby` varchar (32) collate utf8_bin not null comment ''修改人'',
    `rowversion` int(11)  default 1 not null  comment ''版本号'',
    `tenantid` int(11) default 10001 not null comment ''租户'',
    constraint pk_@tableName@_urid primary key (`urid`) using btree
) engine=InnoDB default charset=utf8 collate=utf8_bin comment=''@tableComment@''', 'select ''''') ;
prepare stmt from @sqlstmt ;
execute stmt ;