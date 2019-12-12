start: .igniterc bin/igniter
	bin/igniter

build:
	cargo build

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
