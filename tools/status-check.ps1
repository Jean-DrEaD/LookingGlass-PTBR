param([switch]$NoBuild)
$ErrorActionPreference = 'Continue'
Set-Location (Split-Path $PSScriptRoot -Parent)
$r = New-Object System.Collections.Generic.List[string]; $script:fail = 0
function Add($s){ $r.Add($s); Write-Host $s }
function Check($n,$ok,$i=''){ if($ok){Add "- [ok] $n $i"} else {Add "- [FALHA] $n $i"; $script:fail++} }

Add "# Auditoria $(Get-Date -Format 'yyyy-MM-dd HH:mm')"; Add ""
Add "## Git"
git fetch -q
$branch = git rev-parse --abbrev-ref HEAD; $head = git rev-parse --short HEAD
$ahead = git rev-list --count '@{u}..HEAD'; $behind = git rev-list --count 'HEAD..@{u}'
$dirty = git status --porcelain | ? { $_ -notmatch 'docs/AUDIT.md' }
Check "branch xbox" ($branch -eq 'xbox') "($branch @ $head)"
Check "sincronizado com remoto" ($ahead -eq '0' -and $behind -eq '0') "(ahead $ahead / behind $behind)"
Check "arvore limpa" (-not $dirty) "($(@($dirty).Count) alteracoes)"

Add ""; Add "## STATUS.md"
$st = Get-Content docs\STATUS.md; $lotes = [ordered]@{}
foreach($l in $st){ if($l -match '^\|\s*(\d+)\s*\|\s*(ok|\.\.)\s*\|'){ $lotes[$matches[1]] = $matches[2] } }
$abertos = @($lotes.Keys | ? { $lotes[$_] -eq '..' })
Check "no maximo 1 lote aberto" ($abertos.Count -le 1) "(abertos: $($abertos -join ', '))"
$dups = $st | Select-String '^\s*-\s*\[ok\]\s*Lote\s*(\d+\w?)' | % { $_.Matches[0].Groups[1].Value } | group | ? Count -gt 1 | % Name
if($dups){ Add "- [aviso] [ok] repetido: Lote $($dups -join ', ')" }
$semHash = @($st | Select-String '^\s*-\s*\[ok\]\s*Lote' | ? { $_.Line -notmatch '\b[0-9a-f]{7,}\b' })
Add "- [info] lotes [ok] sem hash: $($semHash.Count)"
Add "- [info] ultimo lote: $(@($lotes.Keys)[-1]) = $(@($lotes.Values)[-1])"

Add ""; Add "## Plano"
$plano = Get-Content lookingglass-xbox.md -Raw
Check "plano aponta Xbox original" ($plano -match 'Xbox original')


Add ""; Add "## Coerencia"
$dec = @($st | Select-String '^- Toolchain:')
Check "decisao de toolchain unica" ($dec.Count -eq 1) "($($dec.Line))"
Check "decisao definida" (-not ($dec.Line -match 'A DEFINIR')) "(bloqueia backend-xbox)"
Check "RAM 128 MB no plano" ((Get-Content lookingglass-xbox.md -Raw) -notmatch '64 MB RAM')
Add ""; Add "## Isolamento e builds"
if(Test-Path tools\platform-guard.ps1){ & .\tools\platform-guard.ps1 *> $null; Check "platform-guard" ($LASTEXITCODE -eq 0 -or $null -eq $LASTEXITCODE) }
if(-not $NoBuild){
  cargo check -q -p lg-core --no-default-features --target i686-pc-windows-msvc 2>$null; Check "lg-core no_std (i686)" ($LASTEXITCODE -eq 0)
  cargo build -q -p lg-xbox --release --target i686-pc-windows-msvc 2>$null;            Check "lg-xbox staticlib (i686)" ($LASTEXITCODE -eq 0)
  cargo check -q 2>$null;                                                                  Check "desktop (macroquad)" ($LASTEXITCODE -eq 0)
}
$lib = 'target\i686-pc-windows-msvc\release\lg_xbox.lib'
if(Test-Path $lib){ Add "- [info] lg_xbox.lib: $([math]::Round((Get-Item $lib).Length/1MB,2)) MB" }

Add ""; Add "**Resultado: $(if($script:fail){"$($script:fail) FALHA(S)"}else{'TUDO OK'})**"
$r | Set-Content docs\AUDIT.md -Encoding utf8
exit $script:fail

