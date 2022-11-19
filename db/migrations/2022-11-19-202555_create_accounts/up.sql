-- Your SQL goes here
CREATE TABLE accounts (
	id serial PRIMARY KEY,
	username VARCHAR ( 50 ) UNIQUE NOT NULL,
	password VARCHAR NOT NULL,
	created_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	last_login TIMESTAMP
);

alter sequence accounts_id_seq restart with 1000;
