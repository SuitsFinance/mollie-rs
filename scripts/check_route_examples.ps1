Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$root = Resolve-Path (Join-Path $PSScriptRoot "..")
$script = Join-Path $PSScriptRoot "route_examples.py"
$python = Get-Command python -ErrorAction SilentlyContinue

if (-not $python) {
    $python = Get-Command python3 -ErrorAction SilentlyContinue
}

if (-not $python) {
    $python = Get-Command py -ErrorAction SilentlyContinue
}

if (-not $python) {
    throw "python, python3, or py is required to check route examples."
}

if ($python.Name -eq "py.exe" -or $python.Name -eq "py") {
    & $python.Source -3 $script check --root $root
}
else {
    & $python.Source $script check --root $root
}

exit $LASTEXITCODE
