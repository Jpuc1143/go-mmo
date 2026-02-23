CREATE TABLE groups (
	id INTEGER PRIMARY KEY NOT NULL,
	is_black BOOL NOT NULL,
	max_liberties INTEGER CHECK(max_liberties > 0) NOT NULL
);

CREATE TABLE group_contacts (
	low_group_id INTEGER NOT NULL,
	high_group_id INTEGER CHECK(high_group_id > low_group_id) NOT NULL,
	count INTEGER CHECK(count > 0) NOT NULL,

	FOREIGN KEY(low_group_id) REFERENCES groups(id) ON DELETE CASCADE,
	FOREIGN KEY(high_group_id) REFERENCES groups(id) ON DELETE CASCADE,
	PRIMARY KEY(low_group_id, high_group_id)
) WITHOUT ROWID;

CREATE TABLE stones (
	x BIGINT NOT NULL,
	y BIGINT NOT NULL,
	group_id INTEGER NOT NULL,

	FOREIGN KEY(group_id) REFERENCES groups(id) ON DELETE CASCADE,
	PRIMARY KEY(x, y)
) WITHOUT ROWID;
