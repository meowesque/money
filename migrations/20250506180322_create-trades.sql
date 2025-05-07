CREATE TABLE trades (
  ask REAL NOT NULL,
  ask_qty REAL NOT NULL,
  bid REAL NOT NULL,
  bid_qty REAL NOT NULL,
  change REAL NOT NULL,
  change_pct REAL NOT NULL,
  high REAL NOT NULL,
  last REAL NOT NULL,
  low REAL NOT NULL,
  symbol TEXT NOT NULL,
  volume REAL NOT NULL,
  vwap REAL NOT NULL,
  time TIMESTAMPTZ NOT NULL
);