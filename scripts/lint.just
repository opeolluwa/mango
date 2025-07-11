@lint:
    cd server && cargo fmt&& cargo sort --workspace && cargo group-imports -- --fix 