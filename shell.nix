# shell.nix
{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.git

    # Rust
    pkgs.rustc
    pkgs.cargo
    pkgs.rustfmt

    # Python
    pkgs.python311
    pkgs.python311Packages.pip

    # Rust ↔ Python integration
    pkgs.maturin
  ];

  shellHook = ''
    export CARGO_TERM_COLOR=always
    export PYTHONUTF8=1
    export VIRTUAL_ENV_DISABLE_PROMPT=1

    alias jinstall='make install'
    alias jtest='make test'
    alias jmeasure-memory='.venv/bin/python ./cli.py measure-memory'
    alias jcompact='.venv/bin/python ./cli.py compact'

    if [ ! -d .venv ]; then
      python -m venv .venv && make install
    fi
    source .venv/bin/activate

    # Clean PS1: remove leading blank lines from prompt
    export PS1="\[\033[1;32m\][nix-sh]\$\[\033[0m\] "
  '';
}
