start: .igniterc bin/igniter
	bin/igniter

build:
	cargo build

deploy:
	heroku create $(APP_NAME) --buildpack https://github.com/danipozo/heroku-buildpack-rust
	heroku addons:create heroku-postgresql:hobby-dev
	git push heroku master

test: db/test.sqlite
	ROCKET_DATABASES="{ meals = { url = \"$(DATABASE_URL)\" } }" RUST_TEST_THREADS=1 cargo test

stop:
	bin/igniter stop meals

bin/igniter:
	cargo install --root . igniter

db/test.sqlite: bin/diesel
	mkdir -p db
	bin/diesel migration run
	bin/diesel migration --migration-dir migrations_test run

bin/diesel:
	cargo install diesel_cli --no-default-features --features sqlite --root .
