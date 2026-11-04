#requires -Version 7
<#
  lg-build.ps1 - build INCREMENTAL + cxbe + ISO em um comando (substitui probe.ps1 + lote-*.ps1 no dia a dia).
  Salvar em D:\LookingGlass-PTBR\tools\lg-build.ps1

  Uso (rodar a partir de qualquer pasta):
    .\tools\lg-build.ps1                          # cliente  -> G:\xemu\alice-cliente.iso
    .\tools\lg-build.ps1 -Dedicado                # dedicado -> G:\xemu\alice-test.iso
    .\tools\lg-build.ps1 -Iso G:\xemu\alice-x.iso # ISO de experimento (nao sobrescreve a principal)
    .\tools\lg-build.ps1 -Testar 10               # apos a ISO, roda tools\lg-xemu-test.ps1 N vezes
    .\tools\lg-build.ps1 -Limpo                   # apaga o build deste alvo (rebuild total)
    .\tools\lg-build.ps1 -SemMapa                 # nao gera o .map do linker

  O que muda em relacao ao probe.ps1:
    - NAO faz rm -rf: so recompila o que mudou. 1a execucao = build completo (como hoje).
    - Build dir num volume Docker (lgbuild), bem mais rapido que bind mount do NTFS.
      cliente e dedicado tem build dirs separados (trocar de alvo nao reconfigura).
    - Reconfigura sozinho quando as flags deste script mudam (hash).
    - cxbe roda no mesmo container; XBE/ISO so existem se o build deste run deu certo.
    - Gera xbe-out\<alvo>.map (alvo = cli | ded) para mapear EIP -> funcao.
#>
param(
  [switch]$Dedicado,
  [string]$Iso,
  [switch]$Limpo,
  [switch]$SemMapa,
  [int]$Testar = 0,
  [int]$Jobs = 12
)
$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $false

# ---------------- CONFIG (ajuste so aqui) ----------------
$omo   = 'D:\openmohaa'
$shims = 'D:\LookingGlass-PTBR\tools\nxdk-shims'
$xiso  = 'E:\Xbox\extract-xiso.exe'
$image = 'lg-nxdk'
$alvo    = $Dedicado ? 'ded' : 'cli'
$exeName = $Dedicado ? 'omohaaded.exe' : 'openmohaa.exe'
$stage   = $Dedicado ? "$omo\iso-stage" : "$omo\iso-cliente"   # default.xbe + main\ (hardlinks)
if (-not $Iso) { $Iso = $Dedicado ? 'G:\xemu\alice-test.iso' : 'G:\xemu\alice-cliente.iso' }
# ----------------------------------------------------------

# flags: iguais as do probe.ps1; lgsys agora em /build/lgsys; +/MAP
$mapFlag = $SemMapa ? '' : " /MAP:/src/xbe-out/$alvo.map"
$flags = '-DCMAKE_TOOLCHAIN_FILE=$NXDK_DIR/share/toolchain-nxdk.cmake -DCMAKE_BUILD_TYPE=Release -DBUILD_CLIENT=OFF -DBUILD_SERVER=ON -DBUILD_RENDERER_GL1=OFF -DBUILD_RENDERER_GL2=OFF -DBUILD_GAME_QVMS=OFF -DBUILD_GAME_LIBRARIES=ON -DUSE_RENDERER_DLOPEN=OFF -DUSE_OPENAL=OFF -DUSE_OPENAL_DLOPEN=OFF -DUSE_HTTP=OFF -DUSE_CODEC_VORBIS=OFF -DUSE_CODEC_OPUS=OFF -DUSE_CODEC_MAD=OFF -DUSE_INTERNAL_LIBS=ON -Wno-dev -DCMAKE_EXE_LINKER_FLAGS="/errorlimit:0 /STACK:4194304,4194304 /build/lgsys/lg_xboxsys.lib $NXDK_DIR/lib/libpbkit.lib $NXDK_DIR/lib/nxdk_usb.lib@@MAPFLAG@@"'
$flags = $flags.Replace('@@MAPFLAG@@', $mapFlag)
if (-not $Dedicado) {
  $flags = $flags.Replace('-DBUILD_CLIENT=OFF', '-DBUILD_CLIENT=ON').Replace('-DBUILD_RENDERER_GL1=OFF', '-DBUILD_RENDERER_GL1=ON')
}
$inc = '-I/shims -include/shims/lg_compat.h -I$NXDK_DIR/lib/xboxrt/libc_extensions -I$NXDK_DIR/lib/net/lwip/src/include -I$NXDK_DIR/lib/net/nforceif/include'
$hash = [BitConverter]::ToString([Security.Cryptography.SHA1]::HashData([Text.Encoding]::UTF8.GetBytes($flags + $inc))).Replace('-', '')

# script que roda DENTRO do container (here-string literal: sem interpolacao do PowerShell)
$tpl = @'
cd /src
INC="@@INC@@"
B=/build/@@ALVO@@
mkdir -p /build /src/xbe-out
@@LIMPO@@
echo '## lgsys (shims)'
if ! { cmake -S /shims/xboxsys -B /build/lgsys -DCMAKE_TOOLCHAIN_FILE=$NXDK_DIR/share/toolchain-nxdk.cmake -DCMAKE_BUILD_TYPE=Release && cmake --build /build/lgsys; } > /build/lgsys.txt 2>&1; then
  echo '## LGSYS falhou'; grep -E 'error' /build/lgsys.txt | head -n 20; exit 1
fi
if [ "$(cat $B/.lgflags 2>/dev/null)" != "@@HASH@@" ]; then
  echo '## configurando'
  cmake -S . -B $B @@FLAGS@@ -DCMAKE_C_FLAGS="$INC -ferror-limit=20" -DCMAKE_CXX_FLAGS="$INC -fexceptions -ferror-limit=20" > $B.cfg.txt 2>&1 || { grep -E 'error' $B.cfg.txt | head -n 20; exit 1; }
  echo "@@HASH@@" > $B/.lgflags
fi
echo '## build'
cmake --build $B -j @@JOBS@@ -- -k -Otarget > $B.build.txt 2>&1; RC=$?
echo "BUILD_EXIT=$RC  objetos_recompilados=$(grep -c 'Building' $B.build.txt)  falhos=$(grep -c 'Error 1' $B.build.txt)"
cp $B.build.txt /src/xbe-out/build-@@ALVO@@.txt
if [ $RC -ne 0 ]; then
  echo '## Erros por arquivo:'
  grep -E '^/src/[^:]+:[0-9]+:[0-9]+: (fatal )?error:' $B.build.txt | sed 's|^/src/||' | awk -F: '!s[$1]++' | head -n 30
  echo '## Linker (simbolos):'
  grep -oE 'undefined symbol: .*' $B.build.txt | sort | uniq -c | sort -rn | head -n 60
  exit 1
fi
EXE=$(find $B -name @@EXE@@ | head -n 1)
[ -n "$EXE" ] || { echo 'exe @@EXE@@ nao encontrado em '$B; exit 1; }
rm -f /src/xbe-out/@@ALVO@@.xbe
$NXDK_DIR/tools/cxbe/cxbe -OUT:/src/xbe-out/@@ALVO@@.xbe "$EXE" > /src/xbe-out/cxbe-@@ALVO@@.txt 2>&1 || { cat /src/xbe-out/cxbe-@@ALVO@@.txt; exit 1; }
echo "XBE $(stat -c %s /src/xbe-out/@@ALVO@@.xbe) bytes  (exe $(stat -c %s "$EXE") bytes)"
'@
$sh = $tpl.Replace('@@INC@@', $inc).Replace('@@ALVO@@', $alvo).Replace('@@HASH@@', $hash).Replace('@@FLAGS@@', $flags).Replace('@@JOBS@@', "$Jobs").Replace('@@EXE@@', $exeName)
$sh = $sh.Replace('@@LIMPO@@', $(if ($Limpo) { 'rm -rf $B' } else { '' }))
$sh = $sh -replace "`r", ''

# ---------------- pre-checks ----------------
docker info *> $null
if ($LASTEXITCODE -ne 0) { throw 'Docker Desktop fechado (abra e rode de novo)' }
foreach ($p in @($omo, $shims, $stage, "$stage\main", $xiso)) {
  if (-not (Test-Path $p)) { throw "nao existe: $p  (ajuste o bloco CONFIG)" }
}
$t0 = Get-Date
New-Item -ItemType Directory -Force "$omo\xbe-out" | Out-Null
Remove-Item "$omo\xbe-out\$alvo.xbe" -ErrorAction SilentlyContinue      # nunca reaproveitar XBE velho
[IO.File]::WriteAllText("$omo\xbe-out\lg-build-$alvo.sh", $sh, (New-Object Text.UTF8Encoding $false))
docker volume create lgbuild | Out-Null

# ---------------- build + cxbe (container) ----------------
docker run --rm -v "${omo}:/src" -v "${shims}:/shims:ro" -v lgbuild:/build -w /src $image sh /src/xbe-out/lg-build-$alvo.sh 2>&1 |
  Tee-Object "$omo\xbe-out\last-$alvo.log"
if ($LASTEXITCODE -ne 0) { throw "build/cxbe falhou - nada foi gerado (log: $omo\xbe-out\last-$alvo.log)" }
$xbe = "$omo\xbe-out\$alvo.xbe"
if (-not (Test-Path $xbe)) { throw 'cxbe nao gerou o XBE' }

# ---------------- stage + ISO ----------------
Copy-Item $xbe "$stage\default.xbe" -Force
(Get-Item "$stage\default.xbe").LastWriteTime = Get-Date
if (Test-Path $Iso) { Remove-Item $Iso -Force }     # falha aqui se o xemu estiver com a ISO aberta
& $xiso -c $stage $Iso | Out-Null
if (-not (Test-Path $Iso)) { throw 'extract-xiso nao gerou a ISO' }

$sec = [int]((Get-Date) - $t0).TotalSeconds
"OK [$alvo]  xbe=$((Get-Item $xbe).Length)  iso=$Iso  ($sec s)  commit=$(git -C $omo rev-parse --short HEAD)"
if (-not $SemMapa) { "map: $omo\xbe-out\$alvo.map" }

if ($Testar -gt 0) {
  & "$PSScriptRoot\lg-xemu-test.ps1" -Iso $Iso -N $Testar
}
