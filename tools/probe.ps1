param(
  [Parameter(Mandatory)][string]$Lote,
  [Parameter(Mandatory)][string]$Nome,
  [string]$Nota = ''
)
$flags = '-DCMAKE_TOOLCHAIN_FILE=$NXDK_DIR/share/toolchain-nxdk.cmake -DCMAKE_BUILD_TYPE=Release -DBUILD_CLIENT=OFF -DBUILD_SERVER=ON -DBUILD_RENDERER_GL1=OFF -DBUILD_RENDERER_GL2=OFF -DBUILD_GAME_QVMS=OFF -DBUILD_GAME_LIBRARIES=ON -DUSE_RENDERER_DLOPEN=OFF -DUSE_OPENAL=OFF -DUSE_OPENAL_DLOPEN=OFF -DUSE_HTTP=OFF -DUSE_CODEC_VORBIS=OFF -DUSE_CODEC_OPUS=OFF -DUSE_CODEC_MAD=OFF -DUSE_INTERNAL_LIBS=ON -Wno-dev'
$cmd = @"
INC="-I/shims -include/shims/lg_compat.h -I`$NXDK_DIR/lib/xboxrt/libc_extensions -I`$NXDK_DIR/lib/net/lwip/src/include -I`$NXDK_DIR/lib/net/nforceif/include"
rm -rf build-xbox
cmake -S . -B build-xbox $flags -DCMAKE_C_FLAGS="`$INC -ferror-limit=20" -DCMAKE_CXX_FLAGS="`$INC -fexceptions -ferror-limit=20" > cfg.txt 2>&1 || { grep -E 'error' cfg.txt | head -n 20; exit 0; }
cmake --build build-xbox -j 12 -- -k -Otarget > build.txt 2>&1; echo "BUILD_EXIT=`$?"
echo '## Objetos ok / falhos:'; grep -c 'Building' build.txt; grep -c 'Error 1' build.txt
echo '## Erros por arquivo:'; grep -E '^/src/[^:]+:[0-9]+:[0-9]+: (fatal )?error:' build.txt | sed 's|^/src/||' | awk -F: '!s[`$1]++' | sort | head -n 40
echo '## Top mensagens:'; grep -oE '(fatal )?error: .*' build.txt | sed -E 's/[0-9]+//g' | sort | uniq -c | sort -rn | head -n 25
echo '## Linker (simbolos):'; grep -oE 'undefined symbol: .*' build.txt | sort | uniq -c | sort -rn | head -n 40
echo '## Alvos:'; find build-xbox \( -name '*.lib' -o -name '*.exe' -o -name '*.xbe' \) | head -n 25
"@
$log = "D:\openmohaa\xbox-probe$Lote.log"
"# Probe $Lote $(Get-Date -Format 'yyyy-MM-dd HH:mm') @ $(git -C D:\openmohaa rev-parse --short HEAD) $Nota" | Set-Content $log
docker run --rm -v "D:\openmohaa:/src" -v "D:\LookingGlass-PTBR\tools\nxdk-shims:/shims:ro" -w /src lg-nxdk sh -c ($cmd -replace "`r","") 2>&1 | Add-Content $log
Copy-Item $log "D:\LookingGlass-PTBR\docs\probes\lote$Lote-openmohaa-$Nome.log" -Force
Get-Content $log
