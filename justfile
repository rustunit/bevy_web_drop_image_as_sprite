
check:
    cargo c --target=wasm32-unknown-unknown

serve:
    trunk serve

build:
    trunk build --release
