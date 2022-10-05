{
  description = "bespoke web service 'framework'";

  # references
  # https://github.com/oxalica/rust-overlay
  # https://hoverbear.org/blog/a-flake-for-your-crate/
  # https://srid.ca/rust-nix

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    naersk.url       = "github:nmattia/naersk";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, naersk, rust-overlay, flake-utils, ... }:
    let 
      cargoToml = (builtins.fromTOML (builtins.readFile ./Cargo.toml));
    in (flake-utils.lib.eachDefaultSystem (system:
      let
        # overlay containing a package built from this repo's Cargo.toml
        mypkgoverlay = final: prev: {
          "${cargoToml.package.name}" = final.callPackage ./. { inherit naersk; };
        };
        overlays = [ (import rust-overlay) mypkgoverlay ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            openssl
            pkg-config
            sqlite
            diesel-cli
            (rust-bin.stable.latest.default.override {
              extensions = ["rust-src"];
            })
          ];

          shellHook = ''
          '';
        };

        packages = {
          "${cargoToml.package.name}" = pkgs."${cargoToml.package.name}";
        };
        defaultPackage = (import nixpkgs {
          inherit system overlays;
        })."${cargoToml.package.name}";
        
      }
    ))
    //
    (
      #flake-utils.lib.eachSystem [ "x86_64-linux" ] (system: {
      ({
        nixosModule = (import ./modules self);
      })
    );
}

