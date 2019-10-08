{ pkgs, channels, lib, config, ... }: with pkgs; with lib; let
  src = nix-gitignore.gitignoreSourcePure [ ''
    *.nix
    /.git
    /.github
  '' ./.gitignore ] ./.;
  cmd = name: command: ci.commandCC {
    inherit name;
    inherit src;
    nativeBuildInputs = [ channels.rust.stable.cargo ];
    command = ''
      export CARGO_TARGET_DIR=$NIX_BUILD_TOP/cargo
      ${command}
    '';
  };
in {
  name = "cfg-match";
  ci.gh-actions = {
    enable = true;
    emit = true;
  };
  cache.cachix.arc.enable = true;
  channels = {
    nixpkgs = "19.09";
    rust = "master";
  };
  tasks.test.inputs = {
    test = cmd "test" ''
      cargo test --manifest-path $src/Cargo.toml
    '';
    build = cmd "build" ''
      cargo build --manifest-path $src/Cargo.toml
    '';
    doc = cmd "doc" ''
      cargo doc --manifest-path $src/Cargo.toml
    '';
  };
  jobs = {
    shell = { channels, ... }: {
      ci.gh-actions.emit = mkForce false;
      channels.arc = "master";
      environment.shell.rust = channels.rust.stable.mkShell {
        cargoCommands = [ "clippy" ];
        rustTools = [ "rust-analyzer" ];
      };
    };
  };
}
