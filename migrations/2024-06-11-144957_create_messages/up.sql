-- Your SQL goes here
create table messages
(
    id      serial
        constraint messages_pk
            primary key,
    room_id int
        constraint messages_rooms_id_fk
            references rooms,
    sender_username    varchar(128),
    chat    text
);

create unique index messages_id_uindex
    on messages (id);

