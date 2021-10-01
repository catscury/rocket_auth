pub(crate) const CREATE_TABLE: &str = "
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    email VARCHAR (254) UNIQUE NOT NULL,
	password VARCHAR ( 255 ) NOT NULL,
    is_admin BOOL DEFAULT FALSE
    is_confirmed BOOL DEFAULT FALSE
);
";

pub(crate) const INSERT_USER: &str = "
INSERT INTO users (email, password, is_admin, is_confirmed) VALUES ($1, $2, $3, $4);
";

pub(crate) const UPDATE_USER: &str = "
UPDATE table SET 
    email = $2,
    password = $3,
    is_admin = $4,
    is_confirmed = $5,
WHERE
    id = $1
";

pub(crate) const SELECT_BY_ID: &str = "
SELECT * FROM users WHERE id = $1;
";

pub(crate) const SELECT_BY_EMAIL: &str = "
SELECT * FROM users WHERE email = $1;
";

pub(crate) const REMOVE_BY_ID: &str = "
DELETE FROM users WHERE id =$1;
";
pub(crate) const REMOVE_BY_EMAIL: &str = "
DELETE FROM users WHERE email =$1;
";
