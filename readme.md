# Aquastation

this project aims to allow a monitoring of personal aquaponics installation.

## Install

This project required :
- a working postgresql with a database created

### prepare database

we use [refinery_cli](https://crates.io/crates/refinery_cli) to execute migrations scripts.
As script are only in sql, you can use a another tool or do this manually.
To setup or update your db, store your db uri in a environment variable. then use command : `refinery migrate -e DB_URI -p ./sql_migrations/v1/update`


## run

- start backend  : `cargo run -p back` 
- start frontend : `cd leptos-front; trunk serve`
