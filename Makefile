start: .igniterc bin/igniter
	bin/igniter

test: db/test.sqlite
	cargo test

stop:
	bin/igniter stop meals

bin/igniter:
	cargo install --root . igniter

db/test.sqlite: bin/diesel
	mkdir -p db
	DATABASE_URL="$@" bin/diesel migration run
	DATABASE_URL="$@" bin/diesel migration --migration-dir migrations_test run

bin/diesel:
	cargo install diesel_cli --no-default-features --features sqlite --root .
