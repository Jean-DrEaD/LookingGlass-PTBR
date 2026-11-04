#requires -Version 7
<#
  lg-xemu-test.ps1 - abre a ISO no xemu varias vezes, olha a tela e diz se o jogo TRAVOU.
  Salvar em D:\LookingGlass-PTBR\tools\lg-xemu-test.ps1
  Normalmente voce nao roda este arquivo direto: use o painel  .\tools\lg.ps1

  Como funciona (sem programar):
    1. abre o xemu com a ISO
    2. tira "fotos" da janela do xemu a cada 5 segundos
    3. se a imagem fica IDENTICA 3 vezes seguidas (depois dos primeiros 30 s), considera TRAVADO
  Durante o teste NAO mexa no mouse/teclado e nao coloque outra janela por cima do xemu.
#>
param(
  [Parameter(Mandatory)][string]$Iso,
  [int]$N = 5,
  [int]$MaxSeg = 120
)
$ErrorActionPreference = 'Stop'
$pasta  = 'D:\openmohaa\xbe-out'
$cfgArq = "$pasta\lg-config.json"
New-Item -ItemType Directory -Force "$pasta\travas" | Out-Null
Add-Type -AssemblyName System.Drawing

if (-not ('LgWin' -as [type])) {
  Add-Type @"
using System;
using System.Runtime.InteropServices;
public class LgWin {
  [StructLayout(LayoutKind.Sequential)] public struct RECT { public int L, T, R, B; }
  [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
  [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr h);
  [DllImport("user32.dll")] public static extern bool SetProcessDPIAware();
  [DllImport("user32.dll")] public static extern bool ShowWindow(IntPtr h, int c);
}
"@
}
[LgWin]::SetProcessDPIAware() | Out-Null

if (-not (Test-Path $Iso)) { throw "A ISO nao existe: $Iso  (rode a opcao 1 do painel para criar)" }

# ---- onde esta o xemu.exe (pergunta so uma vez e lembra) ----
$cfg = if (Test-Path $cfgArq) { Get-Content $cfgArq -Raw | ConvertFrom-Json } else { [pscustomobject]@{} }
$xemu = $cfg.xemu
$abertos = @(Get-Process -Name xemu -ErrorAction SilentlyContinue)
if ((-not $xemu -or -not (Test-Path $xemu)) -and $abertos.Count -gt 0) { $xemu = $abertos[0].Path }
if (-not $xemu -or -not (Test-Path $xemu)) { $xemu = (Read-Host 'Digite o caminho completo do xemu.exe (ex.: G:\xemu\xemu.exe)').Trim('"') }
if (-not (Test-Path $xemu)) { throw "xemu.exe nao encontrado: $xemu" }
$cfg | Add-Member -NotePropertyName xemu -NotePropertyValue $xemu -Force
$cfg | ConvertTo-Json | Set-Content $cfgArq

if ($abertos.Count -gt 0) {
  $r = Read-Host 'O xemu esta aberto. Posso fechar para iniciar os testes? (s/n)'
  if ($r -ne 's') { throw 'Teste cancelado: feche o xemu e rode de novo.' }
  $abertos | Stop-Process -Force
  Start-Sleep 3
}

function Tirar-Foto($proc) {
  $proc.Refresh()
  $h = $proc.MainWindowHandle
  [LgWin]::ShowWindow($h, 9) | Out-Null
  [LgWin]::SetForegroundWindow($h) | Out-Null
  Start-Sleep -Milliseconds 300
  $r = New-Object LgWin+RECT
  [LgWin]::GetWindowRect($h, [ref]$r) | Out-Null
  $larg = $r.R - $r.L
  $alt  = $r.B - $r.T
  $bmp = New-Object System.Drawing.Bitmap $larg, $alt
  $g = [System.Drawing.Graphics]::FromImage($bmp)
  $g.CopyFromScreen($r.L, $r.T, 0, 0, $bmp.Size)
  $g.Dispose()
  return $bmp
}
function Impressao($bmp) {
  $pq = New-Object System.Drawing.Bitmap $bmp, 96, 54
  $ms = New-Object System.IO.MemoryStream
  $pq.Save($ms, [System.Drawing.Imaging.ImageFormat]::Bmp)
  $pq.Dispose()
  return [BitConverter]::ToString([System.Security.Cryptography.SHA1]::HashData($ms.ToArray()))
}

$nomeIso = Split-Path $Iso -Leaf
$res = @()
for ($i = 1; $i -le $N; $i++) {
  Write-Host "Execucao $i de ${N}: abrindo o xemu..." -ForegroundColor Cyan
  $p = Start-Process $xemu -ArgumentList @('-dvd_path', "`"$Iso`"") -PassThru
  $esp = 0
  while (($p.MainWindowHandle -eq [IntPtr]::Zero) -and ($esp -lt 30)) { Start-Sleep 1; $esp++; $p.Refresh() }
  if ($p.MainWindowHandle -eq [IntPtr]::Zero) {
    Stop-Process -Id $p.Id -Force -ErrorAction SilentlyContinue
    throw 'A janela do xemu nao apareceu em 30 s. Abra o xemu manualmente uma vez para conferir a configuracao.'
  }

  $tempo = 0; $igual = 0; $antes = $null; $status = 'OK'
  while ($tempo -lt $MaxSeg) {
    Start-Sleep 5; $tempo += 5
    if ($p.HasExited) { $status = 'FECHOU'; break }
    $foto = Tirar-Foto $p
    $agora = Impressao $foto
    if (($tempo -gt 30) -and ($agora -eq $antes)) { $igual++ } else { $igual = 0 }
    if ($igual -ge 3) {
      $status = 'TRAVOU'
      $foto.Save("$pasta\travas\trava-$nomeIso-$i.png", [System.Drawing.Imaging.ImageFormat]::Png)
    }
    $foto.Dispose()
    $antes = $agora
    if ($status -eq 'TRAVOU') { break }
  }
  if (-not $p.HasExited) { Stop-Process -Id $p.Id -Force }
  Start-Sleep 3
  $cor = if ($status -eq 'TRAVOU') { 'Red' } else { 'Green' }
  Write-Host "   -> $status (aos $tempo s)" -ForegroundColor $cor
  $res += [pscustomobject]@{ Iso = $nomeIso; Run = $i; Status = $status; Seg = $tempo; Data = (Get-Date -Format s) }
}
$res | Export-Csv "$pasta\testes-xemu.csv" -Append -NoTypeInformation

$trav = @($res | Where-Object { $_.Status -eq 'TRAVOU' }).Count
$det  = ($res | ForEach-Object { "$($_.Run)=$($_.Status)@$($_.Seg)s" }) -join ' '
Write-Host "Resultado: $trav de $N execucoes TRAVARAM." -ForegroundColor Yellow
return [pscustomobject]@{ Total = $N; Travou = $trav; Detalhe = $det }
