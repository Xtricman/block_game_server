BEGIN TRANSACTION;
DROP TABLE IF EXISTS "structures";
CREATE TABLE IF NOT EXISTS "structures" (
	"x_start"	INTEGER NOT NULL CHECK(x_start>=0),
	"y_start"	INTEGER NOT NULL CHECK(y_start>=0),
	"z_start"	INTEGER NOT NULL CHECK(z_start>=0),
	"x_end"	INTEGER NOT NULL CHECK(x_end>=x_start),
	"y_end"	INTEGER NOT NULL CHECK(y_end>=y_start),
	"z_end"	INTEGER NOT NULL CHECK(z_end>=z_start),
	"type"	INTEGER NOT NULL CHECK(type>=0),
	PRIMARY KEY("x_start","y_start","z_start","y_end","x_end","z_end")
) WITHOUT ROWID;
DROP TABLE IF EXISTS "entities";
CREATE TABLE IF NOT EXISTS "entities" (
	"uuid"	BLOB NOT NULL CHECK(length(uuid)==16),
	"x"	INTEGER NOT NULL,
	"y"	INTEGER NOT NULL,
	"z"	INTEGER NOT NULL,
	"type"	INTEGER NOT NULL CHECK(type>=0),
	"data"	BLOB NOT NULL CHECK(length(data)>=0),
	PRIMARY KEY("uuid")
) WITHOUT ROWID;
DROP TABLE IF EXISTS "blocks";
CREATE TABLE IF NOT EXISTS "blocks" (
	"x"	INTEGER NOT NULL CHECK(x>=0),
	"y"	INTEGER NOT NULL CHECK(y>=0),
	"z"	INTEGER NOT NULL CHECK(z>=0),
	"type"	INTEGER NOT NULL CHECK(type>=0),
	"data"	BLOB NOT NULL CHECK(length(data)>=0),
	PRIMARY KEY("x","y","z")
) WITHOUT ROWID;
DROP TABLE IF EXISTS "globals";
CREATE TABLE IF NOT EXISTS "globals" (
	"name"	TEXT NOT NULL CHECK(length(name)>0),
	"value"	BLOB NOT NULL CHECK(length(value)>=0),
	PRIMARY KEY("name")
) WITHOUT ROWID;
COMMIT;
