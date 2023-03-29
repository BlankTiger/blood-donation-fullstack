-- Add up migration script here
create table if not exists user_permissions (
	user_id int not null,
	permission varchar(255) not null,
	foreign key (user_id) references users(id)
);
