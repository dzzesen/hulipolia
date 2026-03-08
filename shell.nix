{
  pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/refs/tags/25.11.tar.gz") { },
}:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    nixd
    nil
    nixfmt-rfc-style

    rustup
    pkg-config
    openssl
    wasm-bindgen-cli
    binaryen
  ];

  shellHook = ''
    export RUSTUP_TOOLCHAIN=stable
    export PATH="$HOME/.cargo/bin:$PATH"

    echo "exec -> rustup toolchain install stable"
    rustup toolchain install stable

    echo "exec -> rustup default stable"
    rustup default stable

    echo "exec -> rustup target add wasm32-unknown-unknown"
    rustup target add wasm32-unknown-unknown

    echo "Rust + Dioxus web environment loaded"
    echo "If needed once: cargo install dioxus-cli"
    echo "Run app: dx serve --platform web"
  '';
}
