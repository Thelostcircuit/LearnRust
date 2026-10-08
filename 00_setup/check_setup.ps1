# Checks that this Windows machine is ready for the LearnRust course. Safe to run any time:
# it only looks, it never installs or changes anything.
#
#   powershell -ExecutionPolicy Bypass -File .\check_setup.ps1

$script:problems = 0

function Ok($msg)         { Write-Host "  [ok] " -ForegroundColor Green -NoNewline; Write-Host $msg }
function Bad($msg, $fix)  { Write-Host "  [X]  " -ForegroundColor Red -NoNewline; Write-Host $msg; Write-Host "       -> $fix"; $script:problems++ }
function Warn($msg, $fix) { Write-Host "  [!]  " -ForegroundColor Yellow -NoNewline; Write-Host $msg; Write-Host "       -> $fix" }
function Has($cmd)        { [bool](Get-Command $cmd -ErrorAction SilentlyContinue) }

# Runs a command and returns its first line of output, or $null if it failed.
function FirstLine($cmd, [string[]]$cmdArgs) {
    try {
        $out = & $cmd @cmdArgs 2>$null
        if ($LASTEXITCODE -eq 0 -and $out) { return ($out | Select-Object -First 1).ToString() }
    } catch {}
    return $null
}

$buildTools = 'winget install Microsoft.VisualStudio.2022.BuildTools --override "--wait --passive --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"   (Step 1)'

Write-Host "Required"

# The MSVC linker (link.exe) isn't on PATH outside a Developer Prompt, so ask vswhere where it is.
$vswhere = Join-Path ${env:ProgramFiles(x86)} "Microsoft Visual Studio\Installer\vswhere.exe"
$vcTools = $null
if (Test-Path $vswhere) {
    $vcTools = & $vswhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath 2>$null
}
if ($vcTools) {
    Ok "Visual Studio C++ build tools found"
} else {
    Bad "Visual Studio C++ build tools (the linker) not found" $buildTools
}

if (Has rustup) {
    $v = FirstLine rustup @("--version")
    Ok ($v -replace ' \(.*', '')
} else {
    Bad "rustup not installed" "winget install Rustlang.Rustup   (Step 1), then open a new terminal"
}

$v = FirstLine rustc @("--version")
if ($v) { Ok $v } else { Bad "No Rust toolchain" "rustup default stable   (Step 2)" }

$v = FirstLine cargo @("--version")
if ($v) { Ok $v } else { Bad "cargo not working" "rustup default stable   (Step 2)" }

$v = FirstLine cargo @("fmt", "--version")
if ($v) { Ok $v } else { Bad "rustfmt missing" "rustup component add rustfmt" }

$v = FirstLine cargo @("clippy", "--version")
if ($v) { Ok $v } else { Bad "clippy missing" "rustup component add clippy" }

$v = FirstLine rust-analyzer @("--version")
if ($v) {
    Ok $v
} elseif (FirstLine rustup @("which", "rust-analyzer")) {
    Bad "rust-analyzer installed but not on PATH" "open a new terminal; if that doesn't fix it, see Step 4"
} else {
    Bad "rust-analyzer missing" "rustup component add rust-analyzer   (Step 3)"
}

Write-Host ""
Write-Host "Recommended"

$cargoBin = Join-Path $env:USERPROFILE ".cargo\bin"
$onPath = ($env:PATH -split ';') | Where-Object { $_.TrimEnd('\') -ieq $cargoBin }
if ($onPath) {
    Ok "$cargoBin is on PATH"
} else {
    Warn "$cargoBin is not on PATH" "open a new terminal; the rustup installer adds it (Step 4)"
}

if (Has bacon) {
    Ok (FirstLine bacon @("--version"))
} else {
    Warn "bacon not installed (optional)" "cargo install --locked bacon   (Step 6)"
}

Write-Host ""
if ($script:problems -eq 0) {
    Write-Host "All required pieces are installed. Try: cargo run" -ForegroundColor Green
} else {
    Write-Host "$($script:problems) required piece(s) missing - follow the -> hints above, then run this again." -ForegroundColor Red
    exit 1
}
