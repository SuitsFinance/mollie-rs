Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$root = Resolve-Path (Join-Path $PSScriptRoot "..")
$script = Join-Path $PSScriptRoot "generate_openapi_client.py"
$python = Get-Command python -ErrorAction SilentlyContinue

if (-not $python) {
    $python = Get-Command python3 -ErrorAction SilentlyContinue
}

if (-not $python) {
    $python = Get-Command py -ErrorAction SilentlyContinue
}

if (-not $python) {
    throw "python, python3, or py is required to regenerate the OpenAPI client."
}

if ($python.Name -eq "py.exe" -or $python.Name -eq "py") {
    & $python.Source -3 $script --root $root @args
}
else {
    & $python.Source $script --root $root @args
}

exit $LASTEXITCODE
