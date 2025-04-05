DECLARE NUM NUMBER;
BEGIN
SELECT COUNT(1) INTO NUM FROM USER_TABLES WHERE TABLE_NAME = '@tableName@';
IF NUM = 0
	THEN
		EXECUTE IMMEDIATE '
create table   @tableName@
(
    URID                          VARCHAR2(32) NOT NULL ,
@tableColumn@    CREATEDON                     DATE DEFAULT SYSDATE NOT NULL,
    CREATEDBY                     VARCHAR2(32) NOT NULL ,
    LASTMODIFIEDON                DATE DEFAULT SYSDATE NOT NULL ,
    LASTMODIFIEDBY                VARCHAR2(32) NOT NULL ,
    ROWVERSION                    NUMBER(11) DEFAULT 1 NOT NULL ,
    TENANTID                      NUMBER(11) DEFAULT 10001 NOT NULL
)

tablespace @tableSpace@
  pctfree 10
  initrans 1
  maxtrans 255
  storage
  (
    initial 64K
    next 1M
    minextents 1
    maxextents unlimited
  )';
EXECUTE IMMEDIATE 'ALTER TABLE   @tableName@ ADD CONSTRAINT PK_@tableName@_URID PRIMARY KEY (URID)';
END IF;
END;
/

COMMENT ON TABLE   @tableName@ is '@tableComment@';

COMMENT ON COLUMN @tableName@.URID	 is 	'主键';
@columnComments@COMMENT ON COLUMN @tableName@.CREATEDON	 is 	'创建时间';
COMMENT ON COLUMN @tableName@.CREATEDBY	 is 	'创建人';
COMMENT ON COLUMN @tableName@.LASTMODIFIEDON	 is 	'修改时间';
COMMENT ON COLUMN @tableName@.LASTMODIFIEDBY	 is 	'修改人';
COMMENT ON COLUMN @tableName@.ROWVERSION	 is 	'版本号';
COMMENT ON COLUMN @tableName@.TENANTID	 is 	'租户';

COMMIT;
