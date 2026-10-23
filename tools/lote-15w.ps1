Set-Location D:\openmohaa
Remove-Item D:\openmohaa\xbe-out\default.xbe, D:\openmohaa\iso-stage\default.xbe -ErrorAction SilentlyContinue
& D:\LookingGlass-PTBR\tools\probe.ps1 -Lote 15w -Nome build -Nota "lg_mount_drives early" *>&1 |
  Tee-Object D:\openmohaa\build-15w.log | Select-Object -Last 30
$xbe = Get-Item D:\openmohaa\xbe-out\default.xbe -ErrorAction SilentlyContinue
if (-not $xbe -or $xbe.LastWriteTime -lt (Get-Date).AddMinutes(-10)) { Write-Host "SEM XBE NOVO - colar as 30 linhas acima" -ForegroundColor Red; return }
"XBE: $($xbe.Length) bytes $($xbe.LastWriteTime)"
Copy-Item $xbe.FullName D:\openmohaa\iso-stage\default.xbe -Force
(Get-Item D:\openmohaa\iso-stage\default.xbe).LastWriteTime = Get-Date
Remove-Item G:\xemu\alice-test.iso -ErrorAction SilentlyContinue
& E:\Xbox\extract-xiso.exe -c D:\openmohaa\iso-stage G:\xemu\alice-test.iso
$iso = Get-Item G:\xemu\alice-test.iso -ErrorAction SilentlyContinue
if (-not $iso) { Write-Host "ISO NAO GERADA" -ForegroundColor Red; return }
$iso | Select-Object Name, Length, LastWriteTime
"== push LookingGlass =="
git -C D:\LookingGlass-PTBR push origin xbox
git -C D:\LookingGlass-PTBR status -sb | Select-Object -First 1
