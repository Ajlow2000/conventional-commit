{
    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
        cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
    };

    outputs = inputs: with inputs;
        let
           forAllSystems = nixpkgs.lib.genAttrs nixpkgs.lib.platforms.unix;
           nixpkgsFor = forAllSystems (system: import nixpkgs {
                inherit system;
                config = { };
                overlays = [ cargo2nix.overlays.default ]; # create nixpkgs that contains rustBuilder from cargo2nix overlay
            });

        in {
            apps = forAllSystems (system:
                let 
                    pkgs = nixpkgsFor."${system}"; 

                    # create the workspace & dependencies package set
                    rustPkgs = pkgs.rustBuilder.makePackageSet {
                        rustVersion = "1.84.1";
                        packageFun = import ./Cargo.nix;
                    };
                in {
                    default = (rustPkgs.workspace.hello-world {});
                }
            );
            devShells = forAllSystems (system:
                let pkgs = nixpkgsFor."${system}"; in {
                    default = pkgs.mkShell {
                        packages = with pkgs; [
                            rustc
                            cargo
                            rust-analyzer
                        ];
                    };
                }
            );
       };
}
