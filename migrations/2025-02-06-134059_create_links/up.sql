-- Your SQL goes here

create table links (
    id serial primary key,
    created_at date default current_date not null,
    source varchar(32) not null,
    alias varchar(32) not null unique
)