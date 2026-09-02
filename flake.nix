{
    description = "光子智能综合实验室";

    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

        rust-overlay = {
            url = "github:oxalica/rust-overlay";
            inputs.nixpkgs.follows = "nixpkgs";
        };

        flake-utils.url = "github:numtide/flake-utils";
    };

    outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
        flake-utils.lib.eachDefaultSystem ( system:
            let
                pkgs = import nixpkgs {
                    inherit system;
                    config = {
                        allowUnfree = true;
                    };
                    overlays = [ rust-overlay.overlays.default ];
                };

                rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

                nodejs = pkgs.nodejs_24;
            in
            {
                devShells.default = pkgs.mkShell {
                    buildInputs = with pkgs; [
                        # Rust 工具链
                        rustToolchain
                        cargo-leptos
                        just

                        # Node.js 运行时、pnpm 包管理器
                        nodejs
                        pnpm

                        # Podman 工具
                        podman
                        podman-compose

                        # 本地 K8s 集群工具
                        kind
                        ctlptl
                        tilt

                        # pixi 包管理工具
                        (pkgs.buildFHSEnv {
                            name = "pixi";
                            runScript = "pixi";
                            targetPkgs = pkgs: with pkgs; [
                                pixi
                                zlib
                            ];
                        })

                        # 系统工具
                        lld
                        clang
                    ];

                    shellHook = ''
                        # 打印当前环境信息
                        echo "Rust + pnpm + pixi dev environment"
                        echo "Rust version: $(rustc --version)"
                        echo "Node.js version: $(node --version)"
                        echo "pnpm version: $(pnpm --version)"
                        pixi info

                        # 配置环境变量
                        export KIND_EXPERIMENTAL_PROVIDER=podman
                        export CTLPTL_DOCKER_HOST=podman
                    '';
                };
            };
        );
}