param([string]$Lote, [string]$Texto, [switch]$Show)
$repo = "D:\LookingGlass-PTBR"; $f = "$repo\STATUS.md"
if ($Show) {
  git -C $repo pull --rebase -q 2>$null
  "===== STATUS.md ====="; Get-Content $f
  "===== git log LookingGlass-PTBR ====="; git -C $repo log --oneline -10
  "===== git log openmohaa ====="; git -C D:\openmohaa log --oneline -5; git -C D:\openmohaa status -s | Select-Object -First 20
  return
}
Add-Content $f "`n## Lote $Lote ($(Get-Date -Format 'yyyy-MM-dd HH:mm'))`n$Texto"
git -C $repo add -A STATUS.md docs tools
git -C $repo commit -q -m "lote ${Lote}: status + probes"
git -C $repo push -q 2>$null
git -C D:\openmohaa add -A CMakeLists.txt tools code .gitignore
git -C D:\openmohaa commit -q -m "LG lote ${Lote}" 2>$null
"STATUS lote $Lote registrado e commitado."
