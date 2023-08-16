CREATE TABLE IF NOT EXISTS "pokegotchis" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "name" TEXT NOT NULL,
    "pokedex_name" TEXT NOT NULL,
    "last_feed" TEXT,
    "last_played_with" TEXT
);