DROP TABLE IF EXISTS household_account.app_user;

create table household_account.app_user (
	user_id varchar(12) NOT NULL , 
	last_name varchar(10) NOT NULL,
	first_name varchar(10) NOT NULL,
	create_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
	update_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
	PRIMARY KEY(user_id)
);