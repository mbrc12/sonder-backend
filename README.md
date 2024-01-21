# Backend for Project Sonder

For setup, run 

```
mkdir db
cat sql/create_tables.sql | sqlite3 db/database.db
```

Then you can start the server with
```
cargo run --bin main 
```

which will listen on `localhost:3000`. You can stress test it using

```
cargo run --bin spam
```

which will aim to spawn 1000 threads connecting to the server, doing a write and then a read operation.
Things can be sped up using WAL, using `PRAGMA journal_mode=WAL`.
