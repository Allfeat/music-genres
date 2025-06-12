{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [
          (import rust-overlay)

        ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {

        overlays.default = final: prev: rec {
          nodejs = prev.nodejs;
          yarn = (prev.yarn.override { inherit nodejs; });
        };

        devShells.default = pkgs.mkShell {
          packages =
            with pkgs;
            [
              # Rust
              (rust-bin.fromRustupToolchainFile ./rust/rust-toolchain.toml)
              clang
              protobuf
              openssl
              pkg-config

              # Node.js
              pnpm
              nodejs

              # IDE
              prettier-d-slim
              nodePackages.vscode-langservers-extracted
              yaml-language-server
              typescript-language-server
            ]
            ++ lib.optionals stdenv.hostPlatform.isLinux [ rust-jemalloc-sys-unprefixed ]
            ++ lib.optionals stdenv.hostPlatform.isDarwin [
              darwin.apple_sdk.frameworks.Security
              darwin.apple_sdk.frameworks.SystemConfiguration
            ];

          LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
          # New flag required since https://github.com/eigerco/polka-storage/pull/730
          CRATE_CC_NO_DEFAULTS = 1;
        };
      }
    );
}
