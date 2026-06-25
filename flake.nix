{
  description = "dev shell for game-inventory-tracker";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = {
    self,
    nixpkgs,
  }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    devShells.${system}.default = pkgs.mkShell {
      packages = with pkgs; [
        # Rust
        cargo
        rustc
        rust-analyzer
        rustfmt
        clippy

        # Node / web
        nodejs
        pnpm
        biome

        # NixOS OpenSSL fix
        pkg-config
        openssl

        #postgres
        sqlx-cli
      ];
    };
  };
}
