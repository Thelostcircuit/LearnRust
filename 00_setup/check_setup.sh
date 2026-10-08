#!/usr/bin/env bash
# Checks that this machine is ready for the LearnRust course. Safe to run any time:
# it only looks, it never installs or changes anything.

green=$'\e[32m'; red=$'\e[31m'; yellow=$'\e[33m'; reset=$'\e[0m'
problems=0

# Which install commands to suggest.
os=other
if [ "$(uname -s)" = Darwin ]; then
    os=macos
elif [ -r /etc/os-release ]; then
    . /etc/os-release
    case " $ID $ID_LIKE " in
        *" arch "*) os=arch ;;
        *" debian "*|*" ubuntu "*) os=debian ;;
    esac
fi
case $os in
    arch)   fix_linker="sudo pacman -S --needed base-devel"
            fix_rustup="sudo pacman -S rustup   (Step 1)"
            fix_bacon="sudo pacman -S bacon   (Step 6)" ;;
    debian) fix_linker="sudo apt install build-essential"
            fix_rustup="curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh   (Step 1), then open a new terminal"
            fix_bacon="cargo install --locked bacon   (Step 6)" ;;
    macos)  fix_linker="xcode-select --install"
            fix_rustup="curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh   (Step 1), then open a new terminal"
            fix_bacon="brew install bacon   (Step 6)" ;;
    *)      fix_linker="install your system's C compiler / linker (gcc or clang)"
            fix_rustup="curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh   (Step 1), then open a new terminal"
            fix_bacon="cargo install --locked bacon   (Step 6)" ;;
esac

ok()   { echo "  ${green}✔${reset} $1"; }
bad()  { echo "  ${red}✘${reset} $1"; echo "      → $2"; problems=$((problems + 1)); }
warn() { echo "  ${yellow}!${reset} $1"; echo "      → $2"; }

echo "Required"

if command -v cc >/dev/null; then
    ok "C linker (cc) found"
else
    bad "No C linker (cc)" "$fix_linker   (Step 1)"
fi

if command -v rustup >/dev/null; then
    ok "rustup $(rustup --version 2>/dev/null | awk '{print $2}')"
else
    bad "rustup not installed" "$fix_rustup"
fi

if rustc --version >/dev/null 2>&1; then
    ok "$(rustc --version)"
else
    bad "No Rust toolchain" "rustup default stable   (Step 2)"
fi

if cargo --version >/dev/null 2>&1; then
    ok "$(cargo --version)"
else
    bad "cargo not working" "rustup default stable   (Step 2)"
fi

if cargo fmt --version >/dev/null 2>&1; then
    ok "rustfmt $(rustfmt --version 2>/dev/null | awk '{print $2}')"
else
    bad "rustfmt missing" "rustup component add rustfmt"
fi

if cargo clippy --version >/dev/null 2>&1; then
    ok "$(cargo clippy --version 2>/dev/null)"
else
    bad "clippy missing" "rustup component add clippy"
fi

if rust-analyzer --version >/dev/null 2>&1; then
    ok "$(rust-analyzer --version 2>/dev/null)"
elif rustup which rust-analyzer >/dev/null 2>&1; then
    if [ "$os" = arch ]; then
        # Installed, but Arch's rustup doesn't put its shortcut on PATH.
        bad "rust-analyzer installed but not on PATH" \
            "mkdir -p ~/.cargo/bin; ln -sf /usr/lib/rustup/bin/rust-analyzer ~/.cargo/bin/rust-analyzer   (Step 3)"
    else
        bad "rust-analyzer installed but not on PATH" "put ~/.cargo/bin on your PATH (Step 4), then open a new terminal"
    fi
else
    bad "rust-analyzer missing" "rustup component add rust-analyzer   (Step 3)"
fi

echo
echo "Recommended"

case ":$PATH:" in
    *":$HOME/.cargo/bin:"*) ok "~/.cargo/bin is on PATH" ;;
    *) warn "~/.cargo/bin is not on PATH (only matters for 'cargo install' tools)" \
            "see Step 4 (fish: mkdir -p ~/.cargo/bin; fish_add_path ~/.cargo/bin), then open a new terminal" ;;
esac

lazy_json="$HOME/.config/nvim/lazyvim.json"
if [ -f "$lazy_json" ]; then
    if grep -q 'lang.rust' "$lazy_json"; then
        ok "LazyVim Rust extra enabled"
    else
        warn "LazyVim Rust extra not enabled" "in nvim: :LazyExtras → lang.rust → x   (Step 5)"
    fi
fi

if command -v bacon >/dev/null; then
    ok "bacon $(bacon --version 2>/dev/null | awk '{print $2}')"
else
    warn "bacon not installed (optional)" "$fix_bacon"
fi

echo
if [ "$problems" -eq 0 ]; then
    echo "${green}All required pieces are installed. Try: cargo run${reset}"
else
    echo "${red}$problems required piece(s) missing — follow the → hints above, then run this again.${reset}"
    exit 1
fi
