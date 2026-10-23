<!-- FLUXO-INICIO -->
## >>> FLUXO ATUAL (manter no topo) <<<
- 15w: boot passa Com_Init completo (E: homepath OK, .pid gravado); para apos 'rede desativada (NXDK)'
- 15o: Sys_ListFiles real (FindFirstFileA); ISO a partir de D:\openmohaa\iso-stage (default.xbe + main\ hardlinks de base\)
- ISO: E:\Xbox\extract-xiso.exe -c D:\openmohaa\iso-stage G:\xemu\alice-test.iso
- Depois: Sys_Error sem lixo na tela, remover /FORCE:MULTIPLE, mbtowc/strtold (stubs assert do nxdk)
- Inicio de chat: & D:\LookingGlass-PTBR\tools\lg-status.ps1 -Show
- Regra: comandos sempre completos
- SINGLE PLAYER SEM REDE: remover lwIP/libnxdk_net/gcd_*/winsock do link (15y)
- Repos: port = github.com/Jean-DrEaD/LookingGlass-PTBR (xbox); engine = github.com/Jean-DrEaD/openmohaa (xbox); upstream = openmoh/openmohaa (nunca push)
- Pipeline do lote (TODOS os passos): 0) Docker Desktop aberto (docker info) -> 1) probe.ps1 (build omohaaded.exe) -> 2) cxbe no container (exe -> xbe-out\default.xbe) -> 3) copiar p/ iso-stage + LastWriteTime -> 4) extract-xiso -> alice-test.iso -> 5) commit+push
- probe.ps1 NAO gera XBE: o cxbe tem que estar no script do lote
<!-- FLUXO-FIM -->
# STATUS - Alice (LookingGlass-PTBR) -> Xbox Original
> Anexe no inicio de todo chat. Historico completo: docs/LOG.md. Auditoria: docs/AUDIT.md.
> Ultima atualizacao: 2026-10-21

## Regras fixas
- Comandos sempre completos, prontos para colar no PowerShell 7.
- Todo lote: `status-check` antes -> abrir `[..]` -> trabalho -> commit+push -> fechar `[ok] <hash>` -> push -> `status-check` depois.
- Erro: colar as ultimas 30 linhas do log/compilador.
- Assets nunca no repositorio; o usuario fornece os PK3.
- SINGLE PLAYER SEM REDE: eliminar toda a pilha de rede (lwIP, libnxdk_net, gcd_*/GameSpy, shim winsock, NET_* no-op) p/ economizar RAM/CPU e evitar travas

## Hardware
- Xbox original, CerBios, **128 MB**, IP 192.168.1.100, FTP via FileZilla. Pentium III 733 MHz, NV2A.

## Decisao de arquitetura
- Toolchain: **C** - OpenMoHAA (C/C++) sobre nxdk (Docker xboxdev/nxdk); renderer NV2A reaproveitado do ioQuake3-Xbox; Rust = referencia dos formatos Alice/FAKK2.

## Pastas
| Local | O que |
|---|---|
| `D:\ioQuake3-Xbox` | fork Mayo1970/ioQuake3-Xbox; branch `alice` congelado (ioq3 incompativel com FAKK2) |
| `D:\LookingGlass-PTBR` (branch `xbox`) | PORT: motor Rust + docs + tools/shims (github Jean-DrEaD/LookingGlass-PTBR) |
| `D:\openmohaa` (branch `xbox`) | engine: fork github Jean-DrEaD/openmohaa; upstream = oficial (nunca push) |

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

## Lote 15r (2026-10-21 22:01)
- 15q xemu: FS OK! 8 pk3 / 22738 arquivos; default.cfg executado; omconfig_server.cfg lido
- 15q: novo assert strtod (nxdk stdlib_ext_.c:91) -> chamada fora do macro (libc interna ou TU sem lg_compat)
- 15r: strtod/strtof/strtold definidos como simbolo real em sys_xbox.c -> lg_strtod

## Lote 15s (2026-10-21 22:06)
- 15r xemu: strtod OK (q_shared + libcxx); cfgs executados; fatal FS_CreatePath 'D:\main' (DVD read-only, .pid)
- 15s: mount E:->Harddisk0\Partition1; homepath E:\LG\omohaa; Sys_Mkdir aceita pasta existente

## Lote 15t (2026-10-21 22:11)
- 15s xemu: homepath E: OK; passa .pid; Localization 0 (sem global\); Winsock Initialized; trava apos 'Hostname: xbox'
- Hipotese: gethostbyname sem nxNetInit (lwIP sem netif/DHCP) bloqueia
- 15t: homepath E:\TDATA\4c470001 (+UDATA), cxbe -TITLEID:4C470001; nxNetInit(NULL) no probe com debugPrint

## Lote 15u (2026-10-21 22:15)
- 15t: cxbe rejeitou -TITLEID (saida oculta) -> sem XBE; ISO saiu com XBE velho (trava checava .exe)
- 15u: jogo single player -> NET_Init no-op sob NXDK (net_ip.c), nxNetInit removido; trava ISO = default.xbe existe
- 15v (planejado): remover lwIP/libnxdk_net/gcd_* /winsock shim do link

## Lote 15v (2026-10-21 22:22)
- 15u xemu: 'LG: rede desativada (NXDK)' OK; fatal FS_CreatePath 'D:\main\configs' -> homepath ainda = basepath
- Causa: Sys_DefaultHomePath nosso perde no /FORCE:MULTIPLE (.pid so passou pq Sys_Mkdir aceita D:\main existente)
- 15v: files.c forca fs_homepath=E:\TDATA\4c470001 sob NXDK; remocao lwIP/gcd do link adiada p/ 15w

## Lote 15v2 (2026-10-21 22:26)
- CORRECAO 15v: files.c nao existe (OpenMoHAA = files.cpp); nada aplicado, sem build
- 15v2: Cvar_Get fs_homepath substituido sob NXDK (Cvar_Set seria recusado: CVAR_INIT)

## Lote 15v3 (2026-10-21 22:37)
- CORRECAO 15v2: regex nao casou ('cvar_t *' antes de fs_homepath); XBE igual, nada commitado
- Causa raiz: engine usa Sys_DefaultHome{Data,Config,State}Path do sys_win32.c (static Sys_DefaultHomePath); o nosso nunca e chamado
- 15v3: sys_win32.c retorna E:\TDATA\4c470001 sob NXDK + files.cpp Cvar_Get com default E:; build so se os 2 patches entrarem

## Lote 15w (2026-10-21 22:48)
- 15v3 xemu: 'LG: fs_homepath = E:\TDATA\4c470001' OK; fatal FS_CreatePath 'E:\TDATA' -> E: montado so depois (no probe)
- 15w: lg_mount_drives() (E:->Harddisk0\Partition1, D:->CdRom0) no inicio do Sys_PlatformInit
- 15x (planejado): remover lwIP/libnxdk_net/gcd_*/winsock shim do link

## Lote 15w2 (2026-10-23 11:35)
- 15w: 1a tentativa falhou (Docker Desktop fechado); 2a: build OK (378/0), omohaaded.exe 4.763.648, mas sem XBE
- Causa: lote-15w.ps1 sem o passo cxbe (probe.ps1 so compila) -> pipeline completo registrado no FLUXO

## Lote 15w3 (2026-10-23 11:40)
- 15w xemu: early mount OK; attr E:\TDATA\4c470001=10; .pid gravado em E:; Common Initialization Complete 3770 ms
- Localization 0 (sem main\global\ - esperado); para apos 'LG: rede desativada (NXDK)'
- Hipoteses: (a) dedicado ocioso sem +map no Com_Frame; (b) NET_Sleep/select bloqueando sob NXDK
- 15x: heartbeat no Com_Frame + NET_Sleep sob NXDK -> Sleep(1); remover lwIP/gcd do link

## Lote 15w4 (2026-10-23 11:43)
- Regra fixa: single player sem rede -> pilha de rede sera eliminada (15x)
- openmohaa: fork Jean-DrEaD/openmohaa criado; origin=fork, upstream=oficial; commits LG na branch xbox

## Lote 15x (2026-10-23 11:54)
- 1a tentativa: Select-String posicional + Set-Content sem -NoNewline -> nada aplicado; regra SEM REDE pulada (-notmatch case-insensitive)
- Regra: editar arquivos com [IO.File]::ReadAllText/WriteAllText + caminho absoluto; Select-String sempre com -Path/-Pattern
- 2a: net_ip.c:1671 'extern __stdcall Sleep' conflita com windows.h do nxdk (sem calling convention) -> usar Sleep do windows.h
- 15x: NET_Sleep -> Sleep(msec) sob NXDK (sem select); heartbeat 'LG: frame N' a cada 600 frames no Com_Frame
- 15y (planejado): tirar libnxdk_net/lwIP do probe.ps1 + GameSpy do link
