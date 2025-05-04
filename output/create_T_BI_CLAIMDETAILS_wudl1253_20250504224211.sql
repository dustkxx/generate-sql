set @exist_check := (
  select count(*) from information_schema.tables
  where table_name='T_BI_CLAIMDETAILS'
  and table_schema= database()
) ;
set @sqlstmt := if(@exist_check<1,'create table `T_BI_CLAIMDETAILS` (
    `urid` varchar(32) collate utf8_bin  not null comment ''主键'',
  `CLAIMRELATEDTYPE` CHAR(1) default 7 NOT NULL COMMENT ''认领关联单据类别'' ,
`CLAIMRELATEDID` varchar(32)collate utf8_bin default 8 NOT NULL COMMENT ''认领关联单据id'' ,
`CLAIMAMOUNT` decimal(19,2) default 0 NOT NULL COMMENT ''认领拆分金额'' ,
`MEMO` varchar(512)collate utf8_bin default   COMMENT ''备注'' ,
    `createdon` datetime default current_timestamp not null comment ''创建日期'',
    `createdby` varchar (32) collate utf8_bin not null comment ''创建人'',
    `lastmodifiedon` datetime  default current_timestamp not null comment ''修改时间'',
    `lastmodifiedby` varchar (32) collate utf8_bin not null comment ''修改人'',
    `rowversion` int(11)  default 1 not null  comment ''版本号'',
    `tenantid` int(11) default 10001 not null comment ''租户'',
    constraint pk_T_BI_CLAIMDETAILS_urid primary key (`urid`) using btree
) engine=InnoDB default charset=utf8 collate=utf8_bin comment='' 票据认领拆分金额表''', 'select ''''') ;
prepare stmt from @sqlstmt ;
execute stmt ;