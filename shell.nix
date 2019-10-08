{ ci ? import <ci> { config = ./ci.nix; } }:
  ci.config.jobs.shell.environment.shell.rust
