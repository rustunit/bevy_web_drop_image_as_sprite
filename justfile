
check:
    RUSTFLAGS='--cfg getrandom_backend="wasm_js"' cargo c --target=wasm32-unknown-unknown

serve:
    RUSTFLAGS='--cfg getrandom_backend="wasm_js"' trunk serve

build:
    trunk build --release
