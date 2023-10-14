if [[ ! -e "./Cargo.toml" ]]; then
    echo "This script must be run under the root path of the repo."
    exit
fi

if [[ ! -e ./src/main.rs ]]; then
    cat > ./src/main.rsnnv <<_EOF
    fn main() {
        println!("Hello, world!");
    }
_EOF
fi