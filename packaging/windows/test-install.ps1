<#
.SYNOPSIS
  Install a test MSI, verify native binaries and document conversion, then uninstall.
.DESCRIPTION
  For disposable CI runners only. Verifies independent Not Word registration, Start Menu
  shortcut, both PE machine types, CLI execution and a sample DOCX conversion. Always
  attempts uninstall after a successful install. Does not choose document defaults.
#>
param(
  [Parameter(Mandatory = $true)] [string] $Msi,
  [ValidateSet('x64', 'x86', 'arm64')] [string] $Arch = 'arm64'
)
$ErrorActionPreference = 'Stop'
$Msi = (Resolve-Path $Msi).Path
$Log = Join-Path ([IO.Path]::GetTempPath()) "not-word-install-$PID.log"
$Work = Join-Path ([IO.Path]::GetTempPath()) "not-word-smoke-$PID"
$Machine = switch ($Arch) { 'x64' { 0x8664 } 'x86' { 0x14C } 'arm64' { 0xAA64 } }
$ProgramRoot = if ($Arch -eq 'x86' -and ${env:ProgramFiles(x86)}) { ${env:ProgramFiles(x86)} } else { $env:ProgramFiles }
$Directory = Join-Path $ProgramRoot 'Not Word'
$Shortcut = Join-Path ([Environment]::GetFolderPath('CommonPrograms')) 'Not Word.lnk'

$process = Start-Process msiexec.exe -ArgumentList '/i', "`"$Msi`"", '/qn', '/l*v', "`"$Log`"" -Wait -PassThru
if ($process.ExitCode -ne 0) {
  if (Test-Path $Log) { Get-Content $Log -Tail 60 }
  throw "MSI installation exited $($process.ExitCode)"
}
try {
  foreach ($name in 'not-word.exe', 'not-word-cli.exe') {
    $path = Join-Path $Directory $name
    if (-not (Test-Path $path)) { throw "$name was not installed in $Directory" }
    $bytes = [IO.File]::ReadAllBytes($path)
    $actual = [BitConverter]::ToUInt16($bytes, [BitConverter]::ToInt32($bytes, 0x3C) + 4)
    if ($actual -ne $Machine) { throw "$name has PE machine $actual, expected $Machine for $Arch" }
  }
  if (-not (Test-Path $Shortcut)) { throw 'Not Word Start Menu shortcut was not installed' }
  $registration = Get-ItemProperty 'HKLM:\Software\RegisteredApplications'
  if ($registration.'Not Word' -ne 'Software\NotWord\Capabilities') { throw 'Independent Not Word capabilities registration is missing' }
  $cli = Join-Path $Directory 'not-word-cli.exe'
  & $cli --version
  if ($LASTEXITCODE -ne 0) { throw 'Installed CLI could not run natively' }
  New-Item -ItemType Directory -Force $Work | Out-Null
  $docx = Join-Path $Work 'sample.docx'
  $text = Join-Path $Work 'sample.txt'
  & $cli run --template sample --save $docx
  if ($LASTEXITCODE -ne 0) { throw 'Installed CLI could not write a sample DOCX' }
  & $cli convert $docx $text
  if ($LASTEXITCODE -ne 0) { throw 'Installed CLI could not reopen and convert its DOCX' }
  if (-not (Test-Path $text) -or (Get-Item $text).Length -eq 0) { throw 'Installed CLI conversion did not preserve sample text' }
  Write-Output "ok: $Arch installer, shortcut, registration, native CLI and DOCX conversion"
}
finally {
  $process = Start-Process msiexec.exe -ArgumentList '/x', "`"$Msi`"", '/qn' -Wait -PassThru
  if ($process.ExitCode -ne 0) { throw "MSI uninstall exited $($process.ExitCode)" }
  foreach ($path in (Join-Path $Directory 'not-word.exe'), (Join-Path $Directory 'not-word-cli.exe'), $Shortcut) {
    if (Test-Path $path) { throw "Uninstall left $path behind" }
  }
  Remove-Item -Recurse -Force $Work -ErrorAction SilentlyContinue
  Write-Output 'ok: test package uninstalled'
}
