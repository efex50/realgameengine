#!/bin/bash
set -e

echo "=== Building SDL3 + wgpu Example ==="


linux_target="x86_64-unknown-linux-gnu"
windows_target="x86_64-pc-windows-gnu"
wasm_target="wasm32-unknown-unknown"

releases=""

get_config() {
    local target_arch=$1
    
    cat <<EOF
{
    "rust-analyzer.check.command": "check",
    "rust-analyzer.cargo.buildScripts.enable": true,
    "rust-analyzer.cargo.target": "$target_arch",
    "rust-analyzer.check.extraArgs": [
        "-Z", "build-std=std,panic_abort",
        "--target", "$target_arch"
    ],
    "rust-analyzer.server.extraEnv": {
        "RUSTUP_TOOLCHAIN": "nightly"
    }
}
EOF
}

rust_verr="nightly-x86_64-unknown-linux-gnu"
native_build="rustup run $rust_verr cargo build --release --bin native --features=\"native-bin,\$release\"  --target \$linux_target"
native_run="rustup run $rust_verr cargo run -r --features=\"native-bin,\$release\" --bin native  --target \$linux_target"


help_msg(){
    echo "Usage: ./build.sh [native|web]"
    echo ""
    echo "  native     - Build desktop version with SDL3"
    echo "  native static - staticly link sdl"
    echo "  native run - Build and run"
    echo "  web        - Build WebAssembly version with wasm-pack"
    echo "  web run    - Build and run"
    echo "  target (target)     - Build for custom rust target SDL3"
    echo "  target (target) static - staticl link sdl"
    echo "  target (target) run - Build and run"
    echo ""
    echo "  change dev enviroment"
    echo "  dev (target)      example: dev linux, dev web"
    echo "  or custom target  example: dev x86_64-pc-windows-gnu"

}


case "$1" in
    native)
        echo "Building native..."
        if [ "$3" = "static" ];then
            release="native-bin-release"
        fi
        case "$2" in
            run)
                eval echo $native_run
                #cargo run -r --target x86_64-unknown-linux-gnu --bin native
            ;;
            *)
                eval $native_dev_targetbuild
                #cargo build --release --bin native
                echo "Run with: cargo run --release --bin native"
            ;;
        esac
        ;;
    web)
        echo "Building for web (wasm32)..."
        
        # Ensure wasm-pack is installed
        if ! command -v wasm-pack &> /dev/null; then
            echo "Installing wasm-pack..."
            cargo install wasm-pack
        fi
        
        # Build WASM
        rustup run $rust_verr wasm-pack build . --target web --out-dir web/pkg -- -Z build-std=panic_abort,std
        
        echo ""
        case "$2" in
            run)
                echo "Build complete!"

                echo starting...
                python3 -m http.server 8080 -d web
            ;;
            *)
                echo "Build complete! To run:"
                echo "  cd web && python3 -m http.server 8080"
                echo "  Open http://localhost:8080 in a WebGPU-enabled browser"
            ;;
        esac
        ;;
    target)
        echo todo!  
        if [ "$2" = "run" ];then
            if [ -n "$3" ];then
                linux_target=$3
                eval $native_run
                exit 0
            fi
            exit 1
        elif [ -n "$2" ];then

            linux_target=$2
            echo $linux_target $2
            eval $native_build
        fi
        ;;
    
    dev)
        if [ -z "$2" ];then
            help_msg
            exit 1
        fi

        if [ "$2" = "linux" ];then
            dev_target=$(get_config "$linux_target")
        elif [ "$2" = "web" ];then
            dev_target=$(get_config "$wasm_target")
        elif [ "$2" = "windows" ];then
            dev_target=$(get_config "$windows_target")
        else
            dev_target=$(get_config "$2")
        fi
        printf "$dev_target" > .vscode/settings.json
        ;;
    *)
        help_msg
        exit 1
        ;;
esac