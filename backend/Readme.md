# backend

## Prerequisites

 - Have rust nightly installed. If not
 	- install `rustup`
	- run `rustup toolchain add nightly`
	- run `rustup default nightly`
 - Have postresql installed and setup. For example en Arch/Manjaro
 	- install `postgresql`
	- run `sudo -iu postgres`
	- then run `initdb -D /var/lib/postgres/data`
	- then run `createuser --interactive` (able to create databases, and named for example `cafet-pay`)
	- then run `psql` in this prompt input `\password <USER>` to set a password
	- run `systemctl start/enable postgresql`
 - Set the corret user and password in `Rocket.toml`
 - Install mysql (mariadb on Arch/Manjaro for example)
 - run `cargo install diesel_cli`
 - (optional) add `~/.cargo/` to path
 - run `DATABASE_URL=<url> diesel setup` where the url is the same than the one in the `Rocket.toml`.

## Running

`cargo run`
