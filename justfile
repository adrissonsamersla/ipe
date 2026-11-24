DATE := `date '+%Y-%m-%d'`

clean:
    cargo clean

lint:
    cargo +nightly fmt --all

run:
    cargo run

test:
    cargo test

dist:
    cargo build --release

backup: clean
    cd .. && tar czvf {{DATE}}.ipe.tar.xz ipe
    cd .. && sha512sum {{DATE}}.ipe.tar.xz
