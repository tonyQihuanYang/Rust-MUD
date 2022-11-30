CREATE TABLE players(
	user_id integer NOT NULL,
	id serial PRIMARY KEY,
	name VARCHAR ( 50 ) UNIQUE NOT NULL,
	exp integer NOT NULL,

  CONSTRAINT fk_user
      FOREIGN KEY(user_id) 
	  REFERENCES accounts(id)
);

alter sequence players_id_seq restart with 1000;
