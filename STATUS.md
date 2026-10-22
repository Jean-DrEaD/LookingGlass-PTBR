<!-- FLUXO-INICIO -->
## >>> FLUXO ATUAL (manter no topo) <<<
- 15n: strtod OK; boot chega ao FS_Startup (search path D:\/main, 0 pk3, falta default.cfg)
- 15o: Sys_ListFiles real (FindFirstFileA); ISO a partir de D:\openmohaa\iso-stage (default.xbe + main\ hardlinks de base\)
- ISO: E:\Xbox\extract-xiso.exe -c D:\openmohaa\iso-stage G:\xemu\alice-test.iso
- Depois: Sys_Error sem lixo na tela, remover /FORCE:MULTIPLE, mbtowc/strtold (stubs assert do nxdk)
- Inicio de chat: & D:\LookingGlass-PTBR\tools\lg-status.ps1 -Show
- Regra: comandos sempre completos
<!-- FLUXO-FIM -->
# STATUS - Alice (LookingGlass-PTBR) -> Xbox Original
> Anexe no inicio de todo chat. Historico completo: docs/LOG.md. Auditoria: docs/AUDIT.md.
> Ultima atualizacao: 2026-10-21

## Regras fixas
- Comandos sempre completos, prontos para colar no PowerShell 7.
- Todo lote: `status-check` antes -> abrir `[..]` -> trabalho -> commit+push -> fechar `[ok] <hash>` -> push -> `status-check` depois.
- Erro: colar as ultimas 30 linhas do log/compilador.
- Assets nunca no repositorio; o usuario fornece os PK3.

## Hardware
- Xbox original, CerBios, **128 MB**, IP 192.168.1.100, FTP via FileZilla. Pentium III 733 MHz, NV2A.

## Decisao de arquitetura
- Toolchain: **C** - OpenMoHAA (C/C++) sobre nxdk (Docker xboxdev/nxdk); renderer NV2A reaproveitado do ioQuake3-Xbox; Rust = referencia dos formatos Alice/FAKK2.

## Pastas
| Local | O que |
|---|---|
| `D:\ioQuake3-Xbox` | fork Mayo1970/ioQuake3-Xbox; branch `alice` congelado (ioq3 incompativel com FAKK2) |
| `D:\LookingGlass-PTBR` (branch `xbox`) | motor Rust + docs |

## Validado
- nxdk/Docker compila; o triangulo do RXDK rodou; o OpenArena roda no console (128 MB detectados).
- lg-core no_std (i686) ok; lg_xbox.lib (i686, pentium3) gera, ~3,2 MB antes do link, exporta so `lg_version`.
- App desktop isolado do macroquad via `crate::platform` (Lotes 1-10, platform-guard ativo).

## Lotes
| Lote | Estado | Hash | Resumo |
|---|---|---|---|
| 1-8 | ok | (ver LOG) | math/platform/prelude/features |
| 9 | ok | 940bb15 | Rect proprio, src_rect() |
| 10 | ok | - | zero macroquad fora de src/platform |
| 11 | ok | 8814e2f | status-check.ps1, plano corrigido |
| 12 | ok | 4f94f87 | limpeza do STATUS, auditoria |
| 13 | ok | 89bf9eb | prova: toolchain nxdk ok; bloqueios SDL2/curl/qgl (docs/probes) |
| 14 | ok | 5686da0 | OpenMoHAA dedicado no nxdk: link OK (378 obj) + default.xbe via cxbe |
| 15 | wip | ~45% | link OK 4.6MB (/FORCE:MULTIPLE provisorio); ODR amplo: 7 headers corepp + code/script variam c/ GAME_DLL; proximo: isolar simbolos game; teste xemu exploratorio; Sys_ListFiles pendente |

## Problemas em aberto
- [ok] 14q/14r: link OK (0cb5ed6) + XBE via cxbe. Pendente: EH real, ListFiles.
- [build] 14q: lg_xboxsys.lib (Sys_*/CON_*/atof/EH stub) + libnxdk_net. RISCO: C++ EH ausente no nxdk.
- [build] 14o: link omohaaded.exe falha: lwip, CxxFrameHandler3/CxxThrowException, Sys_*/CON_*, atof. 14p: diagnostico de libs.
- [build] 14n: compila tudo; link falha (lwip, CxxFrameHandler3, Sys_*, atof). 14o: /errorlimit:0 p/ lista completa.
- [build] 14m: 9 falhos (zlib, FAR). 14n: FAR/NEAR vazios; script tools/probe.ps1.
- [build] 14l: 6 falhos; regressao libgame/libcgame (near/far do windows.h via lg_compat). 14m: LEAN_AND_MEAN/NOMINMAX + undef near/far; rmdir/errno; net_ip socklen_t (patch 0002) + stubs getnameinfo.
- [build] 14k: 17->4 falhos (io.h, wspiapi.h, fcntl.h, stat). 14l: shims io/fcntl/sys-stat/wspiapi.
- [build] 14j: configure falhou (nxdk repassa CFLAGS ao lld; '-include <arq>' virou input). 14k: -include junto; ssize_t=int (igual lwIP).
- [build] 14i: GameSpy ok (4 libs gcd); engine 17 falhos (sys/types.h, strcasecmp, *_r, ws2tcpip.h). 14j: shims POSIX + lg_compat.h via -include.
- [build] 14h: 4->2 falhos (gsAssert, gsPlatformUtil). 14i: DebugBreak/LPBYTE/RegCreateKeyExA/RegSetValueExA no shim; primeiro probe de link.
- [build] 14g: 23->4 falhos; libgame ok. nxdk tem CreateSemaphore (sem A), sem TerminateThread/registro. 14h: stubs Win32 no shim.
- [build] 14f: lwIP ok via nforceif/include; 217->23 falhos; libcgame ok. Restante: tipos Win32 no GameSpy, mkdir/rmdir, CreateSemaphoreA.
- [build] 14e: operator[] resolvido; Recast/Detour/DetourCrowd compilam. lwIP exige -I de lwipopts.h e arch/cc.h do port nxdk.
- [nxdk] Macros: NXDK=1, _WIN32=1, _MSC_VER=1933 -> guardar patches com #ifdef NXDK. -fexceptions OK (411 throws).
- [patch] 0001 str::operator[](int) (363 erros); shims winsock.h->lwIP (GameSpy) e sys/stat.h em tools/nxdk-shims.
- [build] Configure dedicado OK. Erros: winsock.h(23, _WIN32 no nxdk), sys/stat.h(2), str::operator[] ambiguo, throw sem -fexceptions.
- [docker] Usar imagem lg-nxdk (tools/docker/Dockerfile): nxdk + bison>=3 + flex.
- [cmake] Flags reais do OpenMoHAA: BUILD_CLIENT, BUILD_SERVER, BUILD_RENDERER_GL1, BUILD_GAME_QVMS, USE_HTTP, USE_OPENAL(_DLOPEN), USE_CODEC_*, USE_RENDERER_DLOPEN. sdl.cmake retorna se BUILD_CLIENT=OFF (SDL resolvido no dedicado).
- [git] docs/probes/*.log esta no .gitignore: usar git add -f.
- [port] qgl (1721 usos) -> camada NV2A do ioQuake3-Xbox; pthread(115)/dlopen(32)/fork(5) -> stubs; modulos de jogo estaticos.
- [risco] Formatos MOHAA (.skd/.skc, TIKI v2) != Alice (.skb/.ska, BSP FAKK): diferencas a mapear usando o Rust como referencia.
- `_aligned_malloc` do lg-xbox: existe no RXDK/MSVC; verificar no nxdk se for o caminho A ou C.
- `-sse2` no i686 (rust#116344): fixar a toolchain do build Xbox.
- Dependencias comportamentais do macroquad: render.rs:303, viewer.rs:1088, look.rs:21/67, character.rs:1406.

## Lote 15i (2026-10-21 20:52)
- lg_game_rename.h (368 linhas) via -include so no alvo game; condicao CMake = if(TARGET game)
- Build: EXIT=0, 378 obj ok / 0 falhos; omohaaded.exe linkado (ainda com /FORCE:MULTIPLE)
- probe.ps1 fica em D:\LookingGlass-PTBR\tools\probe.ps1 (rodar a partir de D:\openmohaa)
- xemu: abrir direto (RetroBat passava a ISO errada); FBO/DXGI warning e inofensivo
- Pendente: confirmar queda de dups (15i2), stubs Sys_Print/CON_Print -> debugPrint, remover /FORCE:MULTIPLE
- Regra: ENVIAR COMANDOS COMPLETOS SEMPRE

## Lote 15j (2026-10-21 20:57)
- 15i2: dups 1750 -> 560 (rename OK; maioria ??_C@ strings COMDAT, inofensivas)
- XBE novo: 4.759.552 bytes (23:52 UTC) -> recriar alice-test.iso com ele
- Tela preta: CON_Print/Sys_DebugPrint usam OutputDebugStringA -> trocar por debugPrint (hal/debug.h)
- Pendente: fonte sys_xbox.c, dups reais (15j), remover /FORCE:MULTIPLE
- Regra: ENVIAR COMANDOS COMPLETOS SEMPRE; consultar/atualizar STATUS.md todo lote

## Lote 15k (2026-10-21 21:01)
- STATUS.md unificado na raiz (docs\STATUS.md absorvido); tools\lg-status.ps1 criado (-Show / registrar+commit)
- 15j: 142 dups reais = __real@ (COMDAT ok) + _Vector*/_DotProduct + CLASS_Print/EV_Print/L_*/Error -> adicionados ao rename
- sys_xbox.c: CON_Print/Sys_DebugPrint/Sys_ErrorDialog -> debugPrint; Sys_PlatformInit -> XVideoSetMode 640x480
- Proximo: recriar alice-test.iso com xbe-out\default.xbe, testar xemu, remover /FORCE:MULTIPLE se dups=0
- REGRAS: ENVIAR COMANDOS COMPLETOS SEMPRE; inicio de todo chat: rodar lg-status.ps1 -Show e colar saida

## Lote 15l (2026-10-21 21:09)
- 15k: link falhou (G_acos/G_ceil/G_fabs/G_sqrt): rename pegou libc -> removidas do lg_game_rename.h
- 15k: dups reais 142 -> 7 (fmod/move/macro/__xmm = COMDAT ok)
- ISO testada no 15k usava XBE antigo (23:52 UTC) -> sem boot; agora a trava exige XBE com menos de 10 min
- Pendente: XBE novo -> alice-test.iso -> xemu (esperado texto debugPrint); remover /FORCE:MULTIPLE

## Lote 15m (2026-10-21 21:22)
- 15l: build OK, XBE 4.771.840; xemu mostra 'LG: OpenMoHAA Xbox boot' -> 1o boot real
- Trava apos Sys_PlatformInit e antes do Com_Init (sem print de versao); hipotese argv[0] NULL -> Sys_Dirname
- 15m: guard NULL em Sys_Dirname; __std_terminate/_CxxThrowException -> debugPrint + loop

## Lote 15n (2026-10-21 21:26)
- 15m xemu: boot -> Sys_Dirname(NULL) -> Common Initialization -> versao -> assert strtod (nxdk stdlib_ext_.c:91)
- 15n: lg_strtod/lg_strtof proprios; atof usa lg_strtod; define global via lg_compat.h
- git openmohaa: lg-status agora commita code/ e .gitignore (logs, xbe-out, build-xbox ignorados)

## Lote 15o (2026-10-21 21:33)
- 15n xemu: FS_Startup roda; search path D:\/main; 0 files in pk3; Couldn't load default.cfg; tela piscando = lixo pos-erro fatal
- 15o: Sys_ListFiles/Sys_FreeFileList reais (FindFirstFileA, malloc); iso-stage com main\ (pk3/cfg do Alice via hardlink)
- Stubs assert(0) restantes no nxdk: strtold, mbtowc (_byteswap_uint64 ok)

## Lote 15o2 (2026-10-21 21:39)
- 15o: LGSYS nao compilou (corpo do ListFiles antigo ficou orfao, linhas 38-40); ISO saiu com XBE velho (Copy-Item preserva data)
- 15o2: orfas removidas; trava da ISO agora = omohaaded.exe existe (apagado antes do build)

## Lote 15o3 (2026-10-21 21:42)
- CORRECAO 15o2: nada foi removido (checagem rejeitou a linha 'char **l = ...('); build falhou de novo
- 15o3: orfas removidas por conteudo exato (calloc / *num = 0 / chave)

## Lote 15o4 (2026-10-21 21:47)
- 15o3: build OK, XBE novo, mas sem print 'LG: ListFiles' -> outra definicao vence (FORCE:MULTIPLE) ou FindFirstFileA falha (D:\\main)
- 15o4: colapsa barras duplas; debugPrint sempre (pat/ok/err) para diagnostico

## Lote 15p (2026-10-21 21:51)
- 15o4 xemu: nossa Sys_ListFiles roda; FindFirstFileA('D:\main\*') err=3 (PATH_NOT_FOUND) -> suspeita D: nao montado
- 15p: lg_probe_d: nxIsDriveMounted('D'), mount D->\Device\CdRom0\ se preciso, lista raiz D:

## Lote 15q (2026-10-21 21:57)
- 15p xemu: D: NAO vinha montado; mount D->\Device\CdRom0\ OK; ListFiles acha 8 pk3, mas FS carrega 0 (fopen com D:\\main\\x.pk3)
- 15q: fopen -> lg_fopen global (lg_compat.h): / -> \ e colapsa barras; debugPrint das 12 primeiras aberturas
- TODO: mover mount do D: para Sys_PlatformInit
