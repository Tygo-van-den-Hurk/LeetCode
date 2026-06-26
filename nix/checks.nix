{
  perSystem = { pkgs, ... }: rec {

    checks."cargo-clippy" =
      pkgs.runCommand "cargo-clippy"
        {
          src = ../.;
          meta.description = "Ensure code follows linting rules.";
          buildInputs = with pkgs; [
            stdenv.cc
            clippy
            cargo
            rustc
          ];
        }
        /* SHELL */ ''
          cp --recursive -- "$src"/* "$PWD"
          cargo clippy -- --deny warnings
          touch "$out"
        '';

    checks."cargo-test" =
      pkgs.runCommand "cargo-test"
        {
          src = ../.;
          meta.description = "Ensure code passes all tests.";
          buildInputs = with pkgs; [ 
            stdenv.cc
            cargo
            rustc
          ];
        }
        /* SHELL */ ''
          cp --recursive -- "$src"/* "$PWD"
          cargo test --all-features
          touch "$out"
        '';

    checks."cargo-fmt" =
      pkgs.runCommand "cargo-fmt"
        {
          src = ../.;
          meta.description = "Ensure code is formatted.";
          buildInputs = with pkgs; [
            stdenv.cc
            rustfmt
            cargo
            rustc
          ];
        }
        /* SHELL */ ''
          cp --recursive -- "$src"/* "$PWD"
          cargo fmt -- --check
          touch "$out"
        '';
  };
}
