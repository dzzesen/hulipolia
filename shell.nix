{
  pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/refs/tags/25.11.tar.gz") { },
}:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    nixd
    nil
    nixfmt-rfc-style
    
    python313
    uv
  ];

  shellHook = ''
    if [ ! -d ".venv" ]; then
      echo "exec -> uv venv .venv"
      uv venv .venv
    fi

    echo "exec -> source .venv/bin/activate"
    source .venv/bin/activate
    
    echo "exec -> uv pip install -r requirements.txt"
    uv pip install -r requirements.txt

    echo "🚀 Flet Dev Environment Loaded"
  '';
}
