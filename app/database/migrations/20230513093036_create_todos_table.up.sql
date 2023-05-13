-- Add up migration script here
-- Add migration script here
-- Project Name : rust-todo
-- Date/Time    : 2023/05/12 22:42:20
-- Author       : tonod
-- RDBMS Type   : PostgreSQL
-- Application  : A5:SQL Mk-2

/*
  << 注意！！ >>
  BackupToTempTable, RestoreFromTempTable疑似命令が付加されています。
  これにより、drop table, create table 後もデータが残ります。
  この機能は一時的に $$TableName のような一時テーブルを作成します。
  この機能は A5:SQL Mk-2でのみ有効であることに注意してください。
*/

-- todoテーブル
--* RestoreFromTempTable
create table todos (
  id serial not null
  , title varchar(256) not null
  , body text not null default ''
  , registed_at timestamp with time zone not null default CURRENT_TIMESTAMP
  , updated_at timestamp with time zone not null default CURRENT_TIMESTAMP
  , constraint todos_PKC primary key (id)
) ;

comment on table todos is 'todoテーブル';
comment on column todos.id is 'ID:ID';
comment on column todos.title is 'タイトル:タイトル';
comment on column todos.body is '詳細:詳細';
comment on column todos.registed_at is '登録日時:登録日時';
comment on column todos.updated_at is '更新日時:更新日時';