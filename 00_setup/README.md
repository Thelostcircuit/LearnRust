# Lesson 0 — Setting Up Your Computer for Rust

This lesson gets your computer ready to write Rust. It takes about 10–20 minutes. Instructions
are given for **Arch Linux** (and Arch-based distros like CachyOS, EndeavourOS, Manjaro),
**Debian / Ubuntu** (and derivatives like Mint and Pop!_OS), **macOS**, and **Windows**. Pick
your system in each step. At the end, the setup checker should report all green.

## What you'll install

| Thing | What it is |
| --- | --- |
| A **C linker** | Rust uses the system linker to produce the final program. On Linux it comes with the C compiler (`gcc`), on macOS with the Xcode Command Line Tools, and on Windows with the Visual Studio C++ Build Tools. |
| **rustup** | The official Rust installer and version manager. It installs the compiler (`rustc`), `cargo`, the standard library docs, `rustfmt` (formatter) and `clippy` (linter), and keeps them updated. |
| **rust-analyzer** | The "language server" that gives your editor autocomplete, inline types, go-to-definition, and red squiggles *as you type*. When you're learning, it's the single most useful tool: it shows you what type everything is. |
| An **editor** set up for Rust | VS Code, Neovim, Zed, or anything with LSP support. |

> Throughout the course, **"the course folder"** means the `LearnRust` folder you unzipped or
> cloned (the one containing this `00_setup` folder). Commands like `cd 00_setup` are run from
> there.

## Step 1 — Install a C linker and rustup

<details open>
<summary><b>Arch Linux / CachyOS</b></summary>

Install rustup from the package repos. That's better than the `curl | sh` script from
rustup.rs, because pacman then keeps rustup itself updated:

```sh
sudo pacman -S --needed base-devel rustup
```

> Don't install the `rust` pacman package as well. It's a separate, fixed copy of Rust that
> conflicts with rustup. rustup is what you want for learning: it can switch versions and add
> components.

</details>

<details>
<summary><b>Debian / Ubuntu</b></summary>

Install the C toolchain (`build-essential`) and `curl`, then use the official rustup
installer. Debian and Ubuntu's own `rustc`/`cargo` packages are often several versions behind,
so skip them.

```sh
sudo apt update
sudo apt install build-essential curl pkg-config libssl-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

When the installer asks, press **Enter** for the default installation. `pkg-config` and
`libssl-dev` aren't needed by this course, but popular crates (like HTTP clients) build
against them, so it saves you a confusing error later.

</details>

<details>
<summary><b>macOS</b></summary>

Install Apple's Command Line Tools (they include the linker), then the official rustup
installer:

```sh
xcode-select --install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

`xcode-select` opens a dialog. Click **Install** and wait for it to finish before running the
second command. If it says the tools are already installed, that's fine. When the rustup
installer asks, press **Enter** for the default installation.

> Homebrew also has a `rust` formula. Skip it. Like the Linux distro packages, it's a fixed
> copy of Rust that doesn't come with rustup.

</details>

<details>
<summary><b>Windows</b></summary>

Rust on Windows uses Microsoft's C++ linker, so you need the **Visual Studio Build Tools**
first. Open **PowerShell** and run:

```powershell
winget install Microsoft.VisualStudio.2022.BuildTools --override "--wait --passive --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
winget install Rustlang.Rustup
```

The first command is a large download (a few GB) and takes a while. It installs the
"Desktop development with C++" workload, which includes the linker and the Windows SDK.

No `winget`? Download **rustup-init.exe** from <https://rustup.rs> and run it. If the Build
Tools are missing, it offers to install them for you. Choose that option.

**Close PowerShell and open a new window** afterwards so it picks up the new PATH.

> You can also do the whole course inside **WSL** (Windows Subsystem for Linux). Run
> `wsl --install` in an admin PowerShell, restart, open Ubuntu, and follow the Debian / Ubuntu
> instructions instead.

</details>

## Step 2 — Install the stable toolchain

Tell rustup to download the current stable Rust and make it the default:

```sh
rustup default stable
```

On **Arch**, rustup starts empty, so this step is required. On the other systems, the
installer already did it, and the command just confirms it's up to date.

This installs `rustc`, `cargo`, `rustfmt`, `clippy`, and offline docs. Check it:

```sh
rustc --version
cargo --version
```

`command not found`? Open a **new terminal** first. The installer changed your PATH, but only
new terminals see the change. (See Step 4 if it still isn't found.)

## Step 3 — Install rust-analyzer

Install it through rustup, so it always matches your compiler version. This works the same on
every system:

```sh
rustup component add rust-analyzer
rustup which rust-analyzer
```

**Arch only:** run these two lines as well:

```sh
mkdir -p ~/.cargo/bin
ln -sf /usr/lib/rustup/bin/rust-analyzer ~/.cargo/bin/rust-analyzer
```

Arch's `rustup` package only puts shortcuts for the common tools (`rustc`, `cargo`, …) in
`/usr/bin`, and leaves the `rust-analyzer` shortcut in `/usr/lib/rustup/bin`, which isn't on
your PATH. Without the link, `rustup component add` says it's installed but the `rust-analyzer`
command isn't found. The link still goes through rustup, so it follows your toolchain when it
updates. (There's also a `rust-analyzer` pacman package. Skip it and use the rustup one.)

On the other systems, the rustup installer already put a `rust-analyzer` shortcut in
`~/.cargo/bin` (`%USERPROFILE%\.cargo\bin` on Windows), so there's nothing extra to do.

`rust-analyzer --version` works once `~/.cargo/bin` is on your PATH (Step 4).

## Step 4 — Make sure `~/.cargo/bin` is on your PATH

`~/.cargo/bin` is where rustup's shortcuts live on Debian, Ubuntu, macOS and Windows. It's also
where tools you install later with `cargo install <tool>` go, on every system.

- **Debian / Ubuntu / macOS with bash or zsh, and Windows:** the rustup installer already added
  it. Open a new terminal and it's there.
- **Arch:** `cargo` lives in `/usr/bin`, so nothing added `~/.cargo/bin`. Add it yourself
  (below).
- **fish (any system):** add it yourself (below). Newer rustup versions do this for fish too,
  but it's harmless to repeat.

For **fish**, run this once:

```fish
mkdir -p ~/.cargo/bin
fish_add_path ~/.cargo/bin
```

The `mkdir` matters: `fish_add_path` silently skips folders that don't exist. fish remembers
the path permanently, so there's no config file to edit.

For **bash** or **zsh** on Arch, add this line to the end of `~/.bashrc` or `~/.zshrc`, then
open a new terminal:

```sh
export PATH="$HOME/.cargo/bin:$PATH"
```

## Step 5 — Set up your editor

Pick one. **VS Code** is the easiest place to start if you don't already have a favourite.

<details open>
<summary><b>VS Code</b> (all systems)</summary>

1. Install it:
   - Arch: `sudo pacman -S code`
   - Debian / Ubuntu: download the `.deb` from <https://code.visualstudio.com>, then
     `sudo apt install ./code_*.deb`
   - macOS: `brew install --cask visual-studio-code`, or download it from the website
   - Windows: `winget install Microsoft.VisualStudioCode`
2. Open the Extensions panel (`Ctrl+Shift+X`, or `Cmd+Shift+X` on macOS), search for
   **rust-analyzer**, and install the one published by *The Rust Programming Language*.
3. Open the course folder with *File → Open Folder…*, then open `00_setup/src/main.rs`. After a
   few seconds (rust-analyzer indexes the project on first open), you should see types appear
   inline.

Useful keys: `F12` goes to definition, hovering shows types and docs, `Ctrl+.` (`Cmd+.`) shows
quick fixes, and `Shift+Alt+F` (`Shift+Option+F`) formats the file.

</details>

<details>
<summary><b>Neovim with LazyVim</b></summary>

LazyVim has a ready-made Rust bundle ("extra"):

1. Open Neovim: `nvim`
2. Type `:LazyExtras` and press Enter.
3. Move the cursor to **`lang.rust`** and press `x` to enable it.
4. Quit and reopen Neovim. It will download the Rust plugins (`rustaceanvim`, `crates.nvim`).
5. Open a Rust file, e.g. `nvim 00_setup/src/main.rs` from the course folder. After a few
   seconds (rust-analyzer indexes the project on first open), you should see types appear
   inline.

Useful keys in LazyVim with the Rust extra (`<leader>` is Space):

| Keys | Does |
| --- | --- |
| `K` | Hover: show the type & docs of the thing under the cursor |
| `gd` | Go to definition (works on standard library items too) |
| `<leader>ca` | Code actions: quick fixes the compiler suggests |
| `<leader>cr` | Rename a symbol everywhere |
| `]d` / `[d` | Jump to next / previous error |
| `<leader>cf` | Format the file (runs rustfmt) |

Files also format automatically when you save.

</details>

<details>
<summary><b>Other editors</b></summary>

- **Zed**: Rust support is built in. Install it from <https://zed.dev> (Arch:
  `sudo pacman -S zed`; macOS: `brew install --cask zed`).
- **RustRover** (JetBrains): a full Rust IDE, free for non-commercial use.
- **Kate** (Linux): has a built-in LSP client. Once rust-analyzer is installed it works
  automatically for `.rs` files. Enable *Settings → Configure Kate → Plugins → LSP Client* if
  it isn't already.

</details>

## Step 6 — Optional nice-to-haves

**bacon** re-runs `cargo check` / `cargo test` every time you save a file:

```sh
sudo pacman -S bacon          # Arch
brew install bacon            # macOS
cargo install --locked bacon  # Debian / Ubuntu / Windows (or anywhere)
```

Run `bacon` in a lesson folder and leave it open in a second terminal. It shows errors the moment
you save. (Press `t` inside bacon to switch to running tests.)

## Step 7 — Check everything

From the course folder, run the checker. It only looks, and never installs or changes anything.

**Linux / macOS:**

```sh
cd 00_setup
./check_setup.sh
```

**Windows (PowerShell):**

```powershell
cd 00_setup
powershell -ExecutionPolicy Bypass -File .\check_setup.ps1
```

It checks each piece and tells you exactly what's missing. Then:

```sh
cargo run
```

You should see a hello message. **Your first compile may take a moment. After that it's fast.**

## Exercise — meet `rustfmt` and `clippy`

`src/messy.rs` has code that *works* (its tests pass) but is badly formatted and unidiomatic.
Rust's tools can tell you exactly what's wrong:

1. `cargo test` — it passes. The code is correct.
2. `cargo fmt` — reformats all the code in this lesson to standard Rust style. Open
   `src/messy.rs` again and compare. No arguments about brace placement, ever.
3. `cargo clippy` — the linter. It prints several warnings, each with an explanation and usually
   a suggested fix. Fix each one by hand (reading the suggestion is the point!).
4. You're done when `cargo clippy` shows no warnings and `cargo test` still passes.

Compare with `solution.rs` afterwards.

## Troubleshooting

| Problem | Fix |
| --- | --- |
| `rustc` / `cargo`: `command not found` | Open a new terminal. Still missing? See Step 4 (PATH) |
| `error: rustup could not choose a version of cargo to run` | You skipped Step 2: `rustup default stable` |
| `rust-analyzer: command not found` or "Unknown binary" | `rustup component add rust-analyzer`. On Arch, also do the `ln` line in Step 3 |
| `linker 'cc' not found` (Linux) | Arch: `sudo pacman -S base-devel` · Debian/Ubuntu: `sudo apt install build-essential` |
| `xcrun: error: invalid active developer path` (macOS) | `xcode-select --install` |
| `linker 'link.exe' not found` (Windows) | Install the Visual Studio Build Tools with the C++ workload (Step 1, Windows), then open a new terminal |
| Editor shows no types / errors | Wait for indexing on first open. Open the **course folder** (not a single file) so rust-analyzer finds `Cargo.toml`. In Neovim, `:LspInfo` shows whether rust-analyzer started |
| `cargo install`-ed tool not found | Step 4 (PATH), then open a new terminal |
| `./check_setup.sh: Permission denied` | `bash check_setup.sh` |

## Keeping Rust up to date

A new stable Rust comes out every 6 weeks. Update with:

```sh
rustup update
```

## Key takeaways

- rustup manages Rust. `rustup default stable` installs it, and `rustup update` updates it.
- Rust needs a C linker from your system: `base-devel` / `build-essential` on Linux, the Xcode
  Command Line Tools on macOS, the Visual Studio Build Tools on Windows.
- rust-analyzer powers editor features. Install it with `rustup component add rust-analyzer`.
- `cargo fmt` formats, `cargo clippy` lints. Run both often.
