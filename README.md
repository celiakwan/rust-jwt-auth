# rust-jwt-auth
An example of implementing JWT authentication in a web application using Rust web framework Rocket.

### Version
- [Rust](https://www.rust-lang.org/): 1.60.0
- [Rocket](https://rocket.rs/): 0.4.10

### Rust Installation
Install Rust.
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install a nightly version of Rust as Rocket is using Rust’s advanced features.
```
rustup override set nightly
```

### PostgreSQL
Install PostgreSQL.
```
brew install postgresql
```

Start the database server.
```
pg_ctl -D /usr/local/var/postgres start
```

Create a user.
```
createuser --interactive --pwprompt
```

Create a database.
```
createdb rust_jwt_auth
```

### Diesel
Set up Diesel.
```
diesel setup
```

Generate database migration files. The following command will generate two sql files. `up.sql` defines how database migration will be applied and `down.sql` defines how it will be reverted.
```
diesel migration generate add_users
```

Execute database migration.
```
diesel migration run
```

### Get Started
```
cargo run
```