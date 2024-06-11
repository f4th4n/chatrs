-- Your SQL goes here
create table rooms
(
    id   serial
        constraint rooms_pk
            primary key,
    name varchar(264)
);

create unique index rooms_id_uindex
    on rooms (id);

