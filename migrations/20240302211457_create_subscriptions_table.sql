CREATE TABLE subscriptions (
	id uuid NOT NULL,
	PRIMARY KEY(id),
	email TEXT NOT NULL unique,
	name TEXT not null,
	subscribed_at timestamptz NOT NULL

);
