start: .igniterc bin/igniter
	bin/igniter

build:
	cargo build

heroku-login:
	heroku login

heroku-create: heroku-login
	heroku create $(APP_NAME) --buildpack https://github.com/danipozo/heroku-buildpack-rust
	heroku addons:create heroku-postgresql:hobby-dev

deploy: heroku-create
	git push heroku master

docker-deploy: heroku-create
	heroku container:login
	heroku container:push web
	heroku container:release web

test: migrations-target
	ROCKET_DATABASES="{ meals = { url = \"$(DATABASE_URL)\" } }" RUST_TEST_THREADS=1 cargo test

stop:
	bin/igniter stop meals

bin/igniter:
	cargo install --root . igniter

migrations-target: bin/diesel
	bin/diesel migration run
	bin/diesel migration --migration-dir migrations_test run

bin/diesel:
	cargo install diesel_cli --no-default-features --features postgres --root .
