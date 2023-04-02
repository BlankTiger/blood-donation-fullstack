-- Add up migration script here
create table if not exists stations (
	id int auto_increment primary key,
	name varchar(255) not null,
	address varchar(255) not null,
	city varchar(255) not null,
	phone varchar(255) not null,
	amount_a_plus float(10, 2) not null,
	amount_a_minus float(10, 2) not null,
	amount_b_plus float(10, 2) not null,
	amount_b_minus float(10, 2) not null,
	amount_ab_plus float(10, 2) not null,
	amount_ab_minus float(10, 2) not null,
	amount_o_plus float(10, 2) not null,
	amount_o_minus float(10, 2) not null,
	fulltext(name, address, city, phone)
);
