# rust-api

## How do i run it?
In order to run this project, first you'll need to define the database url 
that this api is going to connect to, you can do that by setting a
_DATABASE_URL_ enviroment variable. The .env.example file shows you an example
of such a variable. Use the default configuration or change it to whatever you
desire.

After that, if it is the first time you're executing the project, you're
supposed to run the migrations to create the tables on the database, you can do
that by using sqlx. Run the command: 

```bash
sqlx migrate run --database-url <your database url>
```

finally, you can run your api directly using: 
```bash
cargo run
```
