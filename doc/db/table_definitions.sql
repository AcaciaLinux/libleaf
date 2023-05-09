PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS packages (
    id              INTEGER PRIMARY KEY NOT NULL,
    name            TEXT UNIQUE NOT NULL,
    version         TEXT NOT NULL,
    real_version    INTEGER NOT NULL,
    description     TEXT,
    hash            TEXT
);

CREATE TABLE IF NOT EXISTS dependencies (
    depender        INTEGER NOT NULL,
    dependency      INTEGER NOT NULL,

    FOREIGN KEY(depender) REFERENCES packages(id) ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY(dependency) REFERENCES packages(id) ON UPDATE CASCADE ON DELETE RESTRICT,

    PRIMARY KEY(depender, dependency)
);

CREATE TABLE IF NOT EXISTS fsentries (
    id              INTEGER PRIMARY KEY,
    parent          INTEGER,
    package         INTEGER NOT NULL,
    name            TEXT NOT NULL,
    hash            TEXT,

    UNIQUE (parent, package, name),
    FOREIGN KEY (package) REFERENCES packages(id) ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY (parent) REFERENCES fsentries(id)
);

CREATE TABLE IF NOT EXISTS registry (
    reg_key         TEXT PRIMARY KEY NOT NULL,
    reg_value       TEXT
);
