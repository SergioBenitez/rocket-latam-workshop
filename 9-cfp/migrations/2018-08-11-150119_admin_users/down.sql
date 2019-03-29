PRAGMA defer_foreign_keys = ON;

CREATE TABLE old_users (
	id INTEGER PRIMARY KEY NOT NULL,
	github_id TEXT NOT NULL,
	email TEXT NOT NULL,
	name TEXT NOT NULL
);

INSERT INTO old_users (id, github_id, email, name)
	SELECT id, github_id, email, name
	FROM users;

DROP TABLE users;
ALTER TABLE old_users RENAME TO users;
