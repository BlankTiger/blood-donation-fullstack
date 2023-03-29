-- Add up migration script here
create table if not exists users (
	id int auto_increment primary key,
	email varchar(255) not null,
	password varchar(255) not null,
	created_at timestamp default current_timestamp
);
