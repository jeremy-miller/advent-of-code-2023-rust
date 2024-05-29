default:
    @just --list

lint DAY="":
    @if [ -z "{{ DAY }}" ]; then \
        for directory in `ls -d */`; do \
            cd $directory; \
            cargo clippy --fix --allow-dirty --allow-staged -- -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used; \
            cd ..; \
        done \
    else \
        cd {{ DAY }} && cargo clippy --fix --allow-dirty --allow-staged -- -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used; \
    fi

format DAY="":
    @if [ -z "{{ DAY }}" ]; then \
        for directory in `ls -d */`; do \
            cd $directory; \
            cargo fmt; \
            cd ..; \
        done \
    else \
        cd {{ DAY }} && cargo fmt; \
    fi

build DAY="":
    @if [ -z "{{ DAY }}" ]; then \
        for directory in `ls -d */`; do \
            cd $directory; \
            cargo build; \
            cd ..; \
        done \
    else \
        cd {{ DAY }} && cargo build; \
    fi

test DAY="":
    @if [ -z "{{ DAY }}" ]; then \
        for directory in `ls -d */`; do \
            cd $directory; \
            cargo test --no-fail-fast --quiet; \
            cd ..; \
        done \
    else \
        cd {{ DAY }} && cargo test --no-fail-fast --quiet; \
    fi

run DAY PART:
    @cd {{ DAY }} && cargo run --bin {{PART}}

build-release DAY="":
    @if [ -z "{{ DAY }}" ]; then \
        for directory in `ls -d */`; do \
            cd $directory; \
            cargo build --release; \
            cd ..; \
        done \
    else \
        cd {{ DAY }} && cargo build --release; \
    fi

docs DAY:
    @cd {{ DAY }} && cargo doc --open
