<!-- FLUXO-INICIO -->
## >>> FLUXO ATUAL (manter no topo) <<<
- 16z4: MARCO - centipede1 (BSP FAKK v42 + 408 ents) RODANDO em SS_GAME a 20 fps estavel no xemu
- 15o: Sys_ListFiles real (FindFirstFileA); ISO a partir de D:\openmohaa\iso-stage (default.xbe + main\ hardlinks de base\)
- ISO: E:\Xbox\extract-xiso.exe -c D:\openmohaa\iso-stage G:\xemu\alice-test.iso
- 16z9g: MARCO - link LIMPO sem /FORCE:MULTIPLE (mbtowc/rand_s/byteswap/strto* proprios)
- 16z10: REDE FORA DO BINARIO - libnxdk_net (24 stubs lwip), GameSpy sv (6), gcd_*/q_gamespy (1); -I lwIP mantido so p/ headers (net_ip loopback no build cliente)
- Depois: Sys_Error sem lixo na tela; lote 17 = interpretador script Alice; build cliente
- Inicio de chat: & D:\LookingGlass-PTBR\tools\lg-status.ps1 -Show
- Regra: comandos sempre completos
- [ok 16z10g] SINGLE PLAYER SEM REDE: libnxdk_net/gcd_*/q_gamespy fora do link (31 stubs); -I lwIP so headers
- Repos: port = github.com/Jean-DrEaD/LookingGlass-PTBR (xbox); engine = github.com/Jean-DrEaD/openmohaa (xbox); upstream = openmoh/openmohaa (nunca push)
- Pipeline do lote (TODOS os passos): 0) Docker Desktop aberto (docker info) -> 1) probe.ps1 (build omohaaded.exe) -> 2) cxbe no container (exe -> xbe-out\default.xbe) -> 3) copiar p/ iso-stage + LastWriteTime -> 4) extract-xiso -> alice-test.iso -> 5) commit+push
- probe.ps1 NAO gera XBE: o cxbe tem que estar no script do lote
<!-- FLUXO-FIM -->
# STATUS - Alice (LookingGlass-PTBR) -> Xbox Original
> Anexe no inicio de todo chat. Historico completo: docs/LOG.md. Auditoria: docs/AUDIT.md.
> Ultima atualizacao: 2026-10-21

## Projeto
- Jogo: **American McGee's Alice** (single player) -> port para **Xbox Original**.
- Base: LookingGlass-PTBR = traducao PT-BR das builds Windows e Android da recompilacao em Rust do LookingGlass.
- Creditos: [skulitom/LookingGlass](https://github.com/skulitom/LookingGlass) (engine original); [Player124413](https://github.com/Player124413) (port Android).
- Build atual = OpenMoHAA DEDICADO apenas como prova de boot (cliente exige SDL/GL/AL ausentes no nxdk).
- Destino = build CLIENTE (renderer NV2A, audio/input Xbox) com servidor local por loopback (padrao Quake/FAKK2), SEM rede.
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

## Lote 15x2 (2026-10-23 12:01)
- 15x xemu: Com_Init 2984 ms; Localization 0; 'LG: frame 1' aparece -> loop principal alcancado
- Duvida: 600 frames a ~20fps sem mapa = ~30s; heartbeat agora por tempo (2s) com t=Sys_Milliseconds

## Lote 15x2-resultado (2026-10-23 12:07)
- xemu: 'LG: frame N t=...' a cada 2s, +40 frames/2s (20 fps) por 2+ min -> loop principal OK
- Esclarecido: dedicado = prova de boot; alvo = cliente single player com servidor local (loopback), sem rede
- Proximo 15y: remover lwIP/libnxdk_net/gcd_*/winsock do link; depois testar +map de um mapa do Alice (validar diferenca BSP FAKK x MOHAA)

## Lote 15y-a (2026-10-23 12:25)
- Inventario (sem build): refs de rede em probe.ps1/CMake/shims, uso GameSpy no codigo, argv do sys_xbox.c, mapas .bsp + ident/versao BSP
- Log: docs/probes/lote15y-inventario.log; 15y-b = edicoes exatas + build + teste +map

## Lote 15y-b (2026-10-23 12:33)
- 15y-a: BSP Alice = ident FAKK ver 42 (36 mapas em pak0.pk3); OpenMoHAA espera '2015' v17-21 -> loader FAKK necessario (lote 16)
- 15y-a: sys_xbox.c sem main/argv -> mapa disparado por Cbuf_AddText('map centipede1') no frame 40 (common.c)
- 15y-b: libnxdk_net removido do link: SEM_NET=False (se False: simbolos em docs/probes/lote15y-semnet-simbolos.log, probe restaurado)
- 15y-c (planejado): GameSpy fora (shared_sources.cmake/gamespy.cmake sob NXDK) + include lwIP fora

## Lote 15y-c (2026-10-23 12:46)
- 15y-b xemu: 'LG: auto map centipede1' -> 'Server Initialization' -> 'Server: centipede1' -> TRAVA (heartbeat para) dentro de SV_SpawnServer; sem erro de versao BSP
- 15y-b: sem libnxdk_net faltam so 18 lwip_* (socket/bind/select/getaddrinfo/inet_ntop...) -> stubs -1 ou #ifdef NXDK no net_ip.c (15z)
- 15y-c: breadcrumbs 'LG: sv <linha> t=' nas chamadas do SV_SpawnServer apos 'Server: %s'

## Lote 15y-d (2026-10-23 12:57)
- 15y-c xemu: ultimo breadcrumb 'LG: sv 624 t=7587' (600 e 624 no mesmo ms) -> chamada da linha 624 nao retorna (loop/espera, nao I/O lento); 5 min parado
- 15y-d: inspecao da linha 624 + definicao da funcao (docs/probes/lote15y-d-linha624.log), sem build

## Lote 15y-e (2026-10-23 14:09)
- CORRECAO 15y-c: script APAGOU 21 chamadas do SV_SpawnServer ($prev -match sobrescreveu $Matches) -> trava do 15y-c era artefato, diagnostico invalido
- 15y-e: sv_init.c restaurado de 58dc852; breadcrumbs em linha propria; trava = git diff sem remocoes
- Regra: nunca usar $Matches apos outro -match; preferir [regex]::Match e validar diff --numstat (-0)

## Lote 15y-f (2026-10-23 14:17)
- 15y-e xemu (breadcrumbs validos, +52 -0): sv 600/623/627/648 em 1 ms e para -> trava REAL na chamada apos linha 648 (antes do BSP)
- 15y-f: inspecao da chamada 'SV_Startup' (linha 649) + definicao (docs/probes/lote15y-f-linha648.log), sem build

## Lote 15y-g (2026-10-23 14:21)
- 15y-f: trava em SV_Startup (sv_init.c:454); suspeitos = SV_InitGamespy (sockets qr2 sem NET) e NET_JoinMulticast6
- 15y-g: ambos sob #ifndef NXDK (regra SEM REDE) + breadcrumbs 'LG: st <linha>' em cada chamada do SV_Startup

## Lote 15y-h (2026-10-23 14:32)
- 15y-g xemu: SV_Startup OK sem GameSpy/multicast6 (causa da trava); InitGame/CleanupGame do game rodam; pak0.pk3 aberto
- CM_LoadMap: 'centipede1.bsp has wrong version number (42 should be between 17 and 21)' -> previsto (15y-a); servidor encerra limpo
- models\player err=3 = esperado (asset MOHAA, nao existe no Alice)
- 15y-h: coleta p/ loader FAKK (checagens versao, qfiles.h lumps, Rust FAKK) em docs/probes/lote15y-h-bsp.log; lote 16 = loader BSP FAKK v42

## Lote 15y-i (2026-10-23 14:47)
- 15y-h: checagem versao em cm_load.c:894 e renderergl1/tr_bsp.c:2293; lumps via Q_GetLumpByVersion -> ponto de entrada do FAKK v42
- 15y-h: log poluido (typedef struct cm_local.h); 15y-i = coleta focada (LUMP_*/dheader_t, Q_GetLumpByVersion, cm_load 860-893, Rust)

## Lote 15y-j (2026-10-23 14:56)
- CORRECAO 15y-i: variaveis do PowerShell ignoram maiusculas/minusculas ($A sobrescreveu $a) -> arquivos inteiros no log; regra: nomes distintos, nunca $a/$A
- 15y-i (vazamento util): dheader_t = ident/version/checksum + 28 lumps; Q_GetLumpByVersion e static no qfiles.h (v<=18 desloca +1 apos BRUSHES)
- Plano lote 16: ramo version==42 no Q_GetLumpByVersion + conversao de dshader/dleaf/dbrushside/dsurface; checagens em cm_load.c:894 e tr_bsp.c:2293
- 15y-j: coleta do crates/lg-core/src/bsp.rs (ordem dos lumps e tamanhos FAKK)

## Lote 16a (2026-10-23 15:14)
- 15y-j: FAKK header 172 B = 12 + 20 lumps; ordem 0 shaders(76) 1 planes 2 lightmaps 3 surfaces(108=MOHAA) 4 verts(44=MOHAA) 5 indexes 6 leafbrushes 7 leafsurfaces 8 leafs 9 nodes 10 brushsides(8) 11 brushes(12) 12 fogs(72) 13 models(40) 14 entities 15 vis 16-19 lightgrid/entlights/lightdefs
- 16a: cm_load.c sob NXDK: LG_Lump (mapa MOHAA->FAKK; sideeq/terrain/lightgrid vazios), repack shaders 76->140, brushsides 8->12, leafs 48->64 (premissa 48 = layout Q3, nao confirmada); aceita versao 42
- Pendente: renderer tr_bsp.c (build cliente), lightgrid FAKK, fogs

## Lote 16b (2026-10-23 15:24)
- 16a xemu: 'Stack overflow caught' apos fopen pak0.pk3 (request 0x1002c=64KB, livre ~50KB) -> pilha padrao do nxdk pequena; loader FAKK ainda nao exercitado
- 16b 1a tentativa: probe.ps1 nao tem /FORCE:MULTIPLE (flags de link = CMAKE_EXE_LINKER_FLAGS '/errorlimit:0 ...'); nada aplicado
- 16b: /STACK:4194304,4194304 apos /errorlimit:0 (reserve+commit; cxbe herda do PE)

## Lote 16c (2026-10-23 15:49)
- 16b xemu: stack 4MB OK (PE 4194304); passou do overflow e do CM_LoadMap FAKK; breadcrumbs ate 'sv 829 t=9104', depois nada (nem heartbeat)
- 16c: coleta sv_init.c 795-880 (docs/probes/lote16c-sv829.log), sem build

## Lote 16d (2026-10-23 15:50)
- 16c: trava REAL em ge->SpawnEntities (sv_init.c:833; 829 sim, 835 nunca), reproduzida 2x
- Hipoteses: stub _CxxThrowException em loop (throw ScriptException) ou parse de entidades FAKK
- 16d: coleta stub EH + G_SpawnEntities/Level::SpawnEntities + lump14 do centipede1 (docs/probes/lote16d-spawn.log)

## Lote 16e (2026-10-23 15:54)
- 16d: lump14 OK (texto worldspawn/Item_*/func_spawn/info_pathnode) -> remapeamento FAKK de entidades correto
- 16d: print do stub EH nao apareceu -> provavelmente nao e throw; Com_Printf do game = gi.DPrintf (so com developer)
- 16e: breadcrumbs gi.Printf no Level::SpawnEntities (inicio/skill/parse/world/nodes + 'LG: le N classname' por entidade)

## Lote 16f (2026-10-23 15:58)
- 16e xemu: 'se inicio/skill/parse' ok, 'se world' nunca -> trava no args.SpawnInternal() do worldspawn (level.cpp:1159, construtor World)
- 16e: tela deu piscadas rapidas (possivel Com_Error/lixo pos-erro, ver 15o)
- 16f: coleta SpawnArgs::SpawnInternal + World::World (docs/probes/lote16f-world.log), sem build

## Lote 16g (2026-10-23 16:01)
- 16f: SpawnInternal (g_spawn.cpp:337) e World::World (worldspawn.cpp:545) coletados
- 16g: breadcrumbs 'LG: si' (inicio/getclass/cls ok) e 'LG: w' (ctor/move/cfgstr/soundtrack/fog/sky/soundman/target/bounds/fim)
- 16g: resto do SpawnInternal 396-480 em docs/probes/lote16g-spawninternal.log

## Lote 16h (2026-10-23 16:19)
- 16g xemu: si inicio/getclass/cls ok; w ctor..sky..soundman ok, 'w target' nunca -> trava em SoundMan.Load() (World::World)
- 16h: SoundMan.Load sob #ifndef NXDK (contorno); corpo em docs/probes/lote16h-soundman.log p/ correcao real

## Lote 16i (2026-10-23 16:25)
- 16h xemu: World OK; 406 entidades criadas; le 407 info_waypoint -> 'si cls ok' -> 'C++ throw sem runtime EH' (newInstance ou spawnargs)
- Hipotese: ScriptException (evento/spawnarg invalido); trava do SoundMan (Director.ExecuteThread) talvez fosse o mesmo throw
- 16i: coleta stub EH + ScriptException + entidades 403-409 do centipede1 (docs/probes/lote16i-throw.log), sem build
- Proximo: imprimir mensagem do ScriptException no stub; decidir EH real vs. try/catch-less (Com_Error)

## Lote 16j (2026-10-23 16:35)
- 16i: stub _CxxThrowException so imprime e trava; toda ScriptException passa por CreateException(data)
- 16i: ENTIDADES total=0 era bug do script (pipeline desmontou byte[]) -> corrigido com ,$ms.ToArray()
- 16j: debugPrint 'LG: SE <msg>' em ScriptException::CreateException; entidades 403-409 em docs/probes/lote16j-ents.log

## Lote 16k (2026-10-23 16:56)
- 16j xemu: ScriptException = "Couldn't convert string to vector - malformed string '-32 1492 -410'" (le 407 info_waypoint, idx 407)
- '96 1556 -410' e '-32.00 1668.00 -564.00' OK -> falha com inteiro negativo no inicio: bug lg_strtod ou sscanf %f do PDCLib
- 16k: coleta conversor de vetor + lg_strtod + lg_compat (docs/probes/lote16k-vector.log), sem build
- Regra PS: "$var:" dentro de string = escopo de unidade -> usar "${var}:"

## Lote 16l (2026-10-23 17:00)
- 16k: lg_strtod correto p/ '-32'; vectorValue (scriptvariable.cpp:1242) usa sscanf '%f %f %f' -> culpado = sscanf %f do PDCLib
- 1a tentativa: indice errado (stringValue fica 3 linhas acima do throw); nada aplicado. Busca agora a partir da assinatura da funcao
- 16l: LG_ParseVec3 (strtod->lg_strtod) no vectorValue sob NXDK; inventario sscanf %f em docs/probes/lote16l-sscanf.log (47 usos)
- Esperado xemu: passar de 'le 407' e chegar a 'se nodes'/fim do SpawnEntities

## Lote 16m (2026-10-23 17:15)
- 16l xemu: vetor OK (passa conversao); le 407 info_waypoint -> 'SE current thread is NULL' -> throw
- Hipotese: spawnarg do 407 chama Director.CurrentScriptThread() sem thread ativa (level script/SoundMan nao rodaram)
- 16m: coleta throw + chamadores + ent 407 + SpawnInternal (docs/probes/lote16m-thread.log), sem build

## Lote 16n (2026-10-23 17:21)
- 16m: throw em ScriptMaster::CurrentScriptThread (scriptmaster.cpp:1107); chamadores so scriptmaster.cpp:842 (CurrentScriptClass) e scriptslave.cpp:847 (Register)
- 16m: ent 403 (info_waypoint+targetname) passou; 407 e a ULTIMA (408 total) -> throw no fim do SpawnInternal ou apos o laco
- 16n: breadcrumbs 'si newinst ok', 'si fim', 'SM 842', 'SS 847' (linha -> bloco, diff +4 -4)

## Lote 16o (2026-10-23 17:35)
- 16n xemu: TODAS as 408 entidades OK (407 'si fim' t=14059); ~4s depois 'SM 842 CurrentScriptClass' t=18244 -> 'current thread is NULL'
- Throw vem de apos o laco de spawn (pos-spawn do Level), sem 'se nodes' antes
- 16o: coleta chamadores CurrentScriptClass + level.cpp pos-laco (docs/probes/lote16o-scriptclass.log), sem build

## Lote 16p (2026-10-23 17:38)
- 16o: CurrentScriptClass chamado so de gamescript.cpp:267 e :1145 (m_Script = ...->GetScript(), provavel label sem arquivo)
- 16o: pos-laco = UpdateConfigStrings -> PreSpawnSentient + L_ProcessPendingEvents (EV_Entity_Start x408) -> new Player -> ComputeDMWaypoints
- 16p: breadcrumbs 'pl ucfg/pending/player/dmwp' (+4 -4); coleta gamescript.cpp em docs/probes/lote16p-gamescript.log

## Lote 16q (2026-10-23 17:42)
- 16p xemu: 'pl ucfg' -> 'pl pending' -> ~4s -> SM 842 -> throw: dentro do L_ProcessPendingEvents (evento de entidade resolvendo label sem thread)
- 16p: FindLabel (gamescript.cpp:267) ja trata scriptClass NULL; CurrentScriptClass e que lancava
- 16q: CurrentScriptClass usa CurrentThread() e retorna NULL; SetThread (1145) tolera NULL e imprime label (+2 -2)

## Lote 16r (2026-10-23 17:51)
- 16q xemu: 'SM 842 sem thread' t=18072 e para; sem SetThread/SE/throw/'pl player' -> chamador = FindLabel privado; trava/crash silencioso no L_ProcessPendingEvents
- 16q: ~4s fixos entre 'pl pending' e SM 842 (evento pesado antes)
- 16r: FindLabel imprime label privado + 'FindLabel ret' (+2 -2); coleta L_ProcessPendingEvents em docs/probes/lote16r-pending.log

## Lote 16s (2026-10-23 18:03)
- 16r xemu: 'SM 842 sem thread' t=18473 e congela (5 min); NAO apareceu 'FindLabel privado' nem 'SetThread sem thread'
- Conclusao: chamador de CurrentScriptClass e outro (fora de fgame/script) OU copia duplicada de FindLabel/SetThread vence no /FORCE:MULTIPLE; NULL desreferenciado = congelamento
- 16s: coleta global (code\ inteiro + CMake + .obj) em docs/probes/lote16s-callers.log, sem build

## Lote 16t (2026-10-23 18:07)
- 16s: chamador real de CurrentScriptClass = corepp\listener.cpp:4726 (fora de fgame); sem duplicatas (1 obj gamescript/scriptmaster)
- Hipotese: Listener::CreateThreadInternal faz scriptClass->GetScript() sem checar NULL -> congelamento
- 16t: coleta da funcao + chamadores em docs/probes/lote16t-listener.log, sem build

## Lote 16u (2026-10-23 18:10)
- 16t: confirmado listener.cpp:4727 scriptClass->GetScript() com NULL (CreateThreadInternal, label string sem thread ativa)
- 16u: CTI retorna NULL + debugPrint 'LG: CTI sem thread label=X'; ExecuteThreadInternal/4668 saem se NULL (+4 -3)
- Pendente: scriptslave.cpp:865 Register(0, NULL) nao protegido; correcao real = thread do level script ativa no spawn

## Lote 16v (2026-10-23 18:18)
- 16u xemu: L_ProcessPendingEvents PASSOU; CTI sem thread: Centipede1_Talk1, Centipede1_Ambush_Cinema1; '0 teams with 0 entities'
- Em seguida: parse error 'unexpected TOKEN_RIGHT_BRACKET' -> SE "Script 'maps/centipede1.scr' was not properly loaded" -> throw
- Causa raiz do 'sem thread': level script (sintaxe FAKK2) nao compila no parser MOHAA -> sem thread do mapa
- 16v: coleta .scr (pk3) + tokens lexer/gramatica em docs/probes/lote16v-scr.log, sem build

## Lote 16w (2026-10-23 18:50)
- 16v: centipede1.scr = linguagem C-like Ritual (void main(), tipos string/entity/float, for(;;), #include) -> INCOMPATIVEL com Morpheus (MOHAA); '()' vazio = TOKEN_RIGHT_BRACKET
- Decisao: precisa interpretador proprio do script Alice (lote 17); 16x = pular level script sem throw
- 16w: coleta scriptmaster 870-935 + chamadores + VM no Rust + inventario .scr (docs/probes/lote16w-script.log), sem build

## Lote 16x (2026-10-23 19:03)
- 16w: throw em scriptmaster.cpp:903 (compile falhou) e :922 (cache falho); disparo em level.cpp:1496-1501 (GetScript + ExecuteThread m_mapscript)
- 16w: Rust NAO tem VM de script (so script_facts.rs + grafos por mapa, ex. levels_wchess1_graph.rs) -> lote 17 = interpretador proprio (73 .scr, 844 KB)
- 16x: sob NXDK os 2 throws -> 'LG: scr falhou X' + NULL; level.cpp so executa se script != NULL (+8 -0 / +1 -1)
- Risco: CreateThreadInternal (listener.cpp:4740) com scr NULL

## Lote 16y (2026-10-23 19:14)
- 16x xemu: MARCO - 'Server Initialization Complete' com centipede1 (BSP FAKK + 408 ents, level script pulado)
- ubersound err=2 esperado (pasta MOHAA); 'Complete ------ seconds' sem numero -> printf %f do PDCLib suspeito
- Apos 'No bots, skipping navigation' sem heartbeat no print (confirmar com 30 s de espera)
- 16y: coleta sv_init pos-1029 + Recast + Q_vsnprintf (docs/probes/lote16y-pos-init.log), sem build

## Lote 16z (2026-10-23 19:31)
- 16y: 'sv 1036' impresso -> g_gametype != GT_SINGLE_PLAYER -> SV_ServerLoaded -> ge->ServerSpawned -> Recast (sv_maxbots=0, retorna OK)
- 16y: trava esta APOS o Recast (fim do Level::ServerSpawned ou resto do SV_ServerLoaded)
- 16y: '%5.2f' vazio confirma printf %f do PDCLib quebrado (pendente)
- Pendente: g_gametype 0 (single player) sob NXDK, depois de achar a trava
- 16z: 5 breadcrumbs 'LG: sl' no SV_ServerLoaded (SetTime/Fixups/ServerSpawned antes+depois/errorMessage)

## Lote 16z2 (2026-10-23 19:39)
- 16z xemu: sl 1068/1070/1074/1076/1078 todos OK (ServerSpawned + Recast retornam); depois nada, sem heartbeat
- Trava entre fim do SV_ServerLoaded e o 1o Com_Frame/SV_Frame/ge->RunFrame
- 16z2: breadcrumbs sz (fim ServerLoaded), mp (pos SV_SpawnServer), cf (Com_Frame/SV_Frame), rf (ge->RunFrame), limitados a 3 frames

## Lote 16z3 (2026-10-23 19:50)
- 16z2 xemu: sz 1080 -> mp 269 -> rf 1135 -> rf 1137 e para (minutos); NENHUM cf
- 1o ge->RunFrame RETORNA; trava apos ele no mesmo frame (resto do SV_Frame/snapshots/volta ao Com_Frame)
- cf ausente: SV_Frame instrumentado no common.c nao e o caminho executado (outra chamada/ramo)
- 16z3: coleta SV_Frame/RunFrame + contexto cf/rf/mp (docs/probes/lote16z3-frame.log), sem build

## Lote 16z4 (2026-10-23 19:55)
- 16z3: cf NAO faltou - imprimiu nos 3 primeiros frames do boot (antes do map no frame 40), saiu da tela; SV_Frame de common.c:2387 e o caminho certo
- 16z3: trava apos ge->RunFrame (sv_main.c:1136): CheckTimeouts/SendClientMessages/MasterHeartbeat/ProcessGamespyQueries/HandleNonPVSSound
- 16z4: SV_MasterHeartbeat + SV_ProcessGamespyQueries sob #ifndef NXDK (SEM REDE); 6 breadcrumbs 'LG: fr' so em SS_GAME (+10 -0)
- Licao: breadcrumb limitado por contador gasta as impressoes antes do evento -> condicionar ao estado (sv.state)

## Lote 16z5 (2026-10-23 20:03)
- 16z4 xemu: MARCO - heartbeat 'LG: frame N' +40/2s apos o map (798..1638+, t=56s..98s); servidor em SS_GAME estavel
- Causa da trava pos-RunFrame: SV_MasterHeartbeat e/ou SV_ProcessGamespyQueries (rede ausente) -> fora sob NXDK (4ba0b42)
- Proximos: (a) g_gametype 0 sob NXDK; (b) printf/sscanf %f do PDCLib (47 usos sscanf); (c) remover breadcrumbs LG: sv/sl/sz/mp/cf/rf/fr; (d) remover /FORCE:MULTIPLE; (e) lote 17 = interpretador do script Alice; (f) build cliente (NV2A/loopback)

## Lote 16z6a (2026-10-23 20:09)
- Inicio do %f: PDCLib do nxdk imprime '%5.2f' vazio (16y) e sscanf '%f' falha (16k)
- 16z6a: coleta Q_vsnprintf/Com_sprintf/va, lg_compat.h, lg_strtod, PDCLib print/scan, contagem de usos (docs/probes/lote16z6a-printf.log), sem build
- Plano 16z6b: lg_vsnprintf (float proprio, resto delega ao PDCLib) + lg_vsscanf (%f via lg_strtod) ligados no Q_vsnprintf/lg_compat

## Lote 16z6b (2026-10-23 20:17)
- 1a tentativa: patch correto, mas validacao esperava +6 -1; git conta a linha original preservada no #else como mantida (+5 -0 / +4 -0) -> rollback
- Regra: bloco #ifdef que reinsere a linha original no #else = diff so de adicoes (-0)
- 16z6b: lg_vsnprintf proprio (sys_xbox.c; d i u x X o c s p f e g n %, ll/I64/*) ligado no Q_vsnprintf + macro do qcommon.h sob NXDK
- Teste: 'LG: fmt' antes do auto map; esperado [ 3.14] [0.5] [1.234568e+04] [-42] [ok] [00a]
- 16z6c (planejado): lg_sscanf (%f via lg_strtod) p/ os 47 usos

## Lote 16z6c (2026-10-23 20:23)
- 16z6b xemu: printf OK - 'LG: fmt [ 3.14] [0.5] [1.234568e+04] [-42] [ok] [00a]' identico ao esperado
- Efeito colateral: linha fmt repetia ~15x/2s e frame caiu (362 em t=35s) -> provavel if sem chaves no auto map (contexto em docs/probes/lote16z6c-automap.log)
- 16z6c: linha fmt removida (+0 -1); proximo 16z6d = lg_sscanf (%f via lg_strtod)

## Lote 16z6d (2026-10-23 20:42)
- CORRECAO 16z6c: nao era if sem chaves; linha fmt estava no inicio do Com_Frame (todo frame) -> repeticao + queda de fps por debugPrint
- 16z6c xemu: fmt sumiu; heartbeat +40/2s (82..202) -> 20 fps de volta
- 'Server Initialization Complete' nao capturavel (rola rapido); %f coberto pelo mesmo caminho Com_Printf validado em 16z6b
- 16z6d: sscanf/vsscanf definidos em sys_xbox.c (lg_vsscanf: d i u x o p f e g s c [ n %, *, largura, h/l/ll; float via lg_strtod) -> substitui PDCLib nos 47 usos
- Teste one-shot apos 'frame 1': esperado 'LG: scan n=3 [-32] [1492.5] [ab]'
- Proximo: se OK, remover teste + LG_ParseVec3 (16l) vira opcional; depois g_gametype 0 / limpeza de breadcrumbs

## Lote 16z6d2 (2026-10-23 21:11)
- 16z6d xemu: 'LG: scan n=3 [-32] [1492.5] [ab]' identico ao esperado -> lg_vsscanf vence o PDCLib; %f resolvido (printf + sscanf)
- 16z6d2: teste one-shot removido (+0 -1)
- Proximos: (a) g_gametype 0 sob NXDK; (b) remover breadcrumbs LG: sv/st/sl/sz/mp/cf/rf/fr/si/le/w/pl + LG_ParseVec3 (opcional); (c) remover /FORCE:MULTIPLE; (d) lote 17 = interpretador script Alice; (e) build cliente

## Lote 16z7a (2026-10-23 21:19)
- 16z6d2 xemu: build 378/0; heartbeat estavel 82..642 (+40/2s, t=47s); linha scan removida OK
- 16z7a: coleta g_gametype (Cvar_Get, GT_SINGLE_PLAYER, sv_init 1015-1060, SV_ServerLoaded, cfgs) em docs/probes/lote16z7a-gametype.log, sem build
- Risco: gametype 0 pode adiar SV_ServerLoaded ate conexao de cliente (inexistente no dedicado)

## Lote 16z8a (2026-10-23 21:21)
- 16z7a: dedicado cria g_gametype='1' (sv_init.c:1118); com 0, SV_ServerLoaded so via cliente (sv_client.c:938 / cl_ui.cpp:4012) -> sv.state nunca SS_GAME sem cliente
- Decisao: g_gametype 0 ADIADO p/ build cliente (loopback dispara ServerLoaded); dedicado fica gametype 1 como prova de mapa
- 16z8a: inventario de breadcrumbs 'LG:' + LG_ParseVec3 em docs/probes/lote16z8a-breadcrumbs.log, sem build

## Lote 16z8b (2026-10-23 21:25)
- 16z8a: 89 linhas 'LG:' em 19 arquivos (docs/probes/lote16z8a-breadcrumbs.log)
- 16z8b: removidos breadcrumbs sv/st/sl/sz/mp/cf/rf/fr/w/se/le/si + blocos 16n/16p/FindLabel ret restaurados ao comando original (9 arquivos, -101 +8)
- Mantidos: CTI/SM 842/SetThread/FindLabel privado/scr falhou/level script pulado/SE (lote 17), heartbeat+auto map, fs_homepath, rede desativada, FAKK repack, LG_ParseVec3
- Proximos: (a) remover /FORCE:MULTIPLE; (b) lote 17 = interpretador script Alice; (c) build cliente (g_gametype 0 via loopback)

## Lote 16z9a (2026-10-23 21:35)
- 16z8b xemu: breadcrumbs sumiram; 'Server Initialization Complete 18.86 seconds' (%f real OK); heartbeat 242..522 +40/2s; Hitch 11049 ms = frame do load (unico)
- 16z8b: 'Restantes 1' = falso positivo (Select-String case-insensitive casou 'LG: SE' mantido)
- 16z9a: coleta origem do /FORCE:MULTIPLE + dups do link (docs/probes/lote16z9a-force.log), sem build

## Lote 16z9b (2026-10-23 21:39)
- 16z9a: /FORCE:MULTIPLE vem de CMakeLists.txt:103 (target_link_options); libnxdk_net.lib ainda em CMAKE_EXE_LINKER_FLAGS (lote rede)
- 16z9b: link SEM FORCE falhou -> flag restaurada; duplicatas em docs/probes/lote16z9b-dups.log

## Lote 16z9d (2026-10-23 22:10)
- 16z9c: rename G_Error -> dup passou a ser lg_game_G_Error -> as 2 definicoes recebem o rename (game+cgame ou 2 TUs do game); FORCE e rename restaurados (openmohaa em da77ea6)
- 16z9c xemu = ISO antiga (build falhou), boot igual ao 16z8b
- 16z9d: coleta definicoes/alvos/llvm-nm de G_Error (docs/probes/lote16z9d-gerror.log), sem build

## Lote 16z9f (2026-10-24 06:24)
- 16z9e: G_Error RESOLVIDO (Error() do scriptexception.cpp virava G_Error via rename:376); sem FORCE restam _strtod/_strtof/_strtold (sys_xbox.c x PDCLib)
- 16z9f: patch scriptexception reaplicado e commitado (FORCE ainda ativo); coleta membro libc que define strtod + refs (docs/probes/lote16z9f-strtod.log), sem build
- Risco: Error() do script agora = G_Error fatal do g_main (antes throw ScriptException)

## Lote 16z9g (2026-10-24 06:31)
- 16z9f: stdlib_ext_.obj (libxboxrt) define strtod/strtof/strtold + mbtowc/rand_s/_byteswap_* -> outro simbolo puxava o obj
- 16z9g: sys_xbox.c define mbtowc/rand_s/_byteswap_* (asm label); /FORCE:MULTIPLE removido; LINK LIMPO; XBE 4784128 bytes -> testar xemu
- mbtowc deixa de ser stub assert (pendencia do FLUXO)

## Lote 16z10a (2026-10-24 06:45)
- 16z9g xemu: boot identico ao 16z8b sem FORCE; Complete 17.14 s; frame 41 + Hitch 10044 ms (load)
- 16z10a: FLUXO atualizado; coleta refs de rede no link (docs/probes/lote16z10a-rede.log), sem build

## Lote 16z10b (2026-10-24 06:51)
- 16z10a: rede no link = libnxdk_net (probe.ps1) + gcd_key/qr2/natneg/common (sv_gamespy.c/sv_gqueryreporting.c em shared_sources.cmake:101-102); SV_ShutdownGamespy (sv_init.c:1212) sem guard
- 16z10b: libnxdk_net removida do link; stubs de rede gerados: 24 (asm label, -1/NULL); XBE 4620288 bytes -> testar xemu
- 16z10c (planejado): GameSpy fora (gamespy.cmake/shared_sources sob NXDK, SV_ShutdownGamespy guard, include lwIP fora)

## Lote 16z10c (2026-10-24 07:00)
- 16z10b xemu: boot identico; Complete 17.84 s; heartbeat 41/82/122/162 (+40/2s) -> G_Error do script sem efeito colateral
- 16z10c: sv_gamespy.c + sv_gqueryreporting.c comentados (#LG16z10c) no shared_sources.cmake; libs gcd_* nao puxadas; stubs: 6; XBE 4554752 bytes -> testar xemu
- Proximo: include lwIP/shim winsock fora (net_ip.c sob NXDK), depois lote 17 (script Alice) / build cliente

## Lote 16z10d (2026-10-24 07:11)
- 16z10c xemu: boot identico; Complete 18.29 s; heartbeat 41..202 (+40/2s); 376 obj
- 16z10d: build sem -I lwIP/nforceif FALHOU -> probe restaurado; erros em docs/probes/lote16z10d-openmohaa-build.log

## Lote 16z10e (2026-10-24 07:25)
- 16z10d: sem -I lwIP -> 23 falhos 'lwip/sockets.h' (gcd_common gsAssert/gsAvailable/gsCore/gsCrypt/gsDebug...): gamespy.cmake ainda compila gcd_*; probe restaurado
- 16z10e: coleta objetos falhos + includes de rede (code/shims) + refs gcd_/gamespy no CMake + includes q_gamespy.c (docs/probes/lote16z10e-lwip.log), sem build

## Lote 16z10g (2026-10-24 07:40)
- 16z10e: lwIP via shim winsock em gsPlatform.h (gcd_*), net_ip.c, common.c; -I lwIP fica (headers nao entram no XBE)
- 16z10f-a: sem gcd_*/q_gamespy so falta _Com_InitGameSpy; 16z10f-b falhou: comentario C com 'gcd_*/' fechou o /* (regra: nunca '*/' dentro de comentario gerado)
- 16z10g: include(gamespy) -> include_directories; q_gamespy.c comentado; stub Com_InitGameSpy vazio; XBE 4554752 bytes -> testar xemu

## Lote 17a (2026-10-24 08:07)
- 16z10g xemu: boot identico; Complete 16.93 s; heartbeat 41/82/122 (+40/2s); 343 obj -> REDE FORA DO BINARIO concluida
- 17a: inventario dos .scr do Alice (75 arquivos, 888122 bytes): includes, keywords, funcoes (1425), metodos (169), $vars, amostra global (docs/probes/lote17a-scr.log), sem build

## Lote 17b (2026-10-24 08:09)
- 17a: 75 scr/888 KB; keywords wait 2233, void 2073, thread 1192, waitfor 934; 169 metodos (anim/time/show/remove/bind...); 1425 funcoes = maioria labels proprios
- DECISAO lote 17: TRADUTOR offline Alice->Morpheus (PowerShell no PC) gerando iso-stage\main\zz_lg_scr.pk3 (fora do repo); VM MOHAA reaproveitada; comandos ausentes = Events stub (17d)
- 17b: amostras completas (centipede1/utils/cinematics/sound/centipede1_cinematics/garden3) + tokens Morpheus (docs/probes/lote17b-amostras.log), sem build

## Lote 17c (2026-10-24 08:20)
- 17b: amostras confirmam sintaxe Ritual C-like (tipos, params, globais, for/while/if, #include, getentity, vetores '...')
- 17c: tools/scr2morph.cs (C# via Add-Type) traduz maps/*.scr -> Morpheus (includes inline, globais level., locais local., chamadas -> waitthread/cmd, metodos -> $e m args, hoist de chamadas em expressao)
- 17c: 19 ok / 19 falhos de 38; saida iso-stage\main\zz_lg_scr.pk3 (fora do repo); ISO recriada sem build (docs/probes/lote17c-scr2morph.log)
- Esperado xemu: sem parse error; provavel 'LG: SE' + trava (EH stub) no 1o comando desconhecido -> 17d = ScriptError sem throw

## Lote 17d (2026-10-24 08:59)
- 17c xemu: parse error SUMIU (Morpheus aceitou a traducao); 'SE unknown command: movetoent' -> throw: o compilador valida TODOS os comandos (ate em labels nao chamados)
- 17c: 17 falhas '{' ~linha 411 = provavel prototipo 'void X();'; 2 falhas '=' (centipede2/garden4)
- 17d: scr2morph v2 (prototipos, contexto da linha na falha): 0 ok / 38 falhos; coleta 'unknown command' no compilador (docs/probes/lote17d-scr2morph.log); ISO recriada, sem build
- 17e (planejado): unknown command -> aviso 'LG: cmd? X' + comando ignorado sob NXDK

## Lote 17d2 (2026-10-24 09:05)
- CORRECAO 17d: Add-Type do v2 falhou (LgScrError ja definido pelo v1 na mesma sessao) -> 0/38; pk3 saiu VAZIO
- Regra PS: Add-Type nao redefine tipos na sessao -> nomes de classe versionados + -PassThru (sem type literal); abortar pk3/ISO se 0 ok
- 17d: 'unknown command' = scriptcompiler.cpp:1126 (cmd_event_statement) e :1149 (cmd_event_expr), CompileError quando eventnum==0 -> alvo do 17e
- 17d2: v2 rodado: 32 ok / 6 falhos (docs/probes/lote17d2-scr2morph.log); ISO recriada, sem build

## Lote 17e (2026-10-24 09:18)
- 17d2: v2 = 32 ok / 6 falhos; xemu igual (movetoent)
- Tentativas: -f do PS quebrou com '{ }' (usar marcador + .Replace()); probe.ps1 exige -Lote/-Nome; cxbe com Substring(15) errado (D:\openmohaa\ = 13) -> usar Substring('D:\openmohaa\'.Length)
- 17e: scriptcompiler.cpp cmd/method_event_statement: eventnum==0 -> 'LG: cmd? X' (40 max) + break (+9 -0); build 343/0; XBE 4554752
- Pendente 17f: cmd/method em expressao (cmd_event_expr:1148, method_event_expr:1191 -> precisa valor na pilha); scr2morph v3 p/ os 6 falhos

## Lote 17f (2026-10-24 09:24)
- 17e xemu: statements desconhecidos viram aviso (cam_fadeout/fadein, hideweapon, fakeplayer, dt_fadetime/fadeoutspeed/fadeinspeed/position/clear); para em 'SE unknown command: allow_cheshire' (uso em expressao) -> throw
- 17f: cmd_event_expr + method_event_expr: eventnum==0 -> 'LG: cmd?e X' (40 max) + OP_STORE_NIL na pilha + break (+6 -0); XBE 4554752
- Lista de comandos Alice ausentes = base p/ Events stub (17g); scr2morph v3 p/ os 6 falhos

## Lote 17g (2026-10-24 12:07)
- 17f xemu: tela so com 'C++ throw sem runtime EH' -> avisos (ate 40+40) encheram a tela e o debugPrint apagou o resto (inclusive o 'LG: SE')
- Regra: total de debugPrint antes de um erro tem que caber na tela (~20 linhas)
- 17g: lg_n++ < 40 -> < 5 (4 ocorrencias, +4 -4); coleta stub EH/SE em docs/probes/lote17g-limite.log; XBE 4554752
- Proximo: stub EH reimprime a ultima mensagem SE antes de travar

## Lote 17h (2026-10-24 15:53)
- 17g xemu: 'cmd?e allow_cheshire' -> 'SE Failed execution of command alias' -> throw; COMPILACAO DO SCRIPT PASSOU, erro agora em execucao
- 17h: coleta origem 'Failed execution of command' + evento alias + linhas alias no zz_lg_scr.pk3/pak0.pk3 (docs/probes/lote17h-alias.log), sem build

## Lote 17i (2026-10-24 15:59)
- 17h: 'alias' = EV_RegisterAlias da classe ScriptMaster (scriptmaster.cpp:103), nao da thread -> checkValidEvent (scriptvm.cpp:607) lanca; formato 'alias "a" "b"' correto
- 17i: zz_lg_scr.pk3 pos-processado: 'local.lg_master = spawn ScriptMaster' + 'local.lg_master alias ...' (alias=527, spawns=31); ISO recriada, sem build
- Pendente: levar a regra pro scr2morph v3; checkValidEvent sob NXDK -> aviso em vez de throw

## Lote 17j (2026-10-24 16:07)
- 17i xemu: alias OK via ScriptMaster (527 alias / 31 spawns); novo 'SE command nottriggerable applied to NULL listener' -> throw (entidade $nome inexistente)
- 17j: coleta origem 'NULL listener' + nottriggerable (traduzido x original) + targetnames do centipede1.bsp (docs/probes/lote17j-nulllistener.log), sem build
- Plano 17k: NULL listener sob NXDK -> aviso 'LG: nul X' + pula comando (comportamento do jogo original = so warning)

## Lote 17k (2026-10-24 16:11)
- 17j: throw em scriptvm.cpp:723 (cmd) e :742 (method); traducao correta, mas c1_changelevel NAO existe nos 71 targetnames do centipede1.bsp
- 17k: macro LG_NUL_THROW sob NXDK -> 'LG: nul X' (5 max) + return (args ja desempilhados; method retorna NIL) (+9 -2); XBE 4554752
- Pendente: 'Field applied to NULL listener' (1436/1588/1616) e 'function applied to NULL listener' (761/798) ainda lancam

## Lote 17l (2026-10-24 16:17)
- 17k xemu: 'nul nottriggerable' + 'nul load' OK; novo 'SE Failed execution of command maxmouthangle for class Object Targetname centipede_actor1' -> throw (checkValidEvent)
- ACHADO: centipede_actor1 spawnado como Object (classnames Alice sem mapeamento p/ Actor) -> lote futuro = tabela classname Alice -> classe OpenMoHAA
- 17l: checkValidEvent sob NXDK -> 'LG: cev X classe' (5 max) + return (+3 -0); executeCommandInternal no log p/ validar; XBE 4554752

## Lote 17m (2026-10-24 16:47)
- 17l xemu: MARCO - level script traduzido EXECUTA sem throw; cev maxmouthangle/ai_off Object (5); Complete 16.39 s; heartbeat 41..202 (+40/2s)
- 17l: secao executeCommandInternal vazia no log (regex nao casou) - irrelevante, skip validado no xemu
- 17m: coleta classnames centipede1 x CLASS_DECLARATION OpenMoHAA + getClassDef + refs Rust (docs/probes/lote17m-classes.log), sem build
- Plano 17n: tabela classname Alice -> classe OpenMoHAA (alias no getClassDef sob NXDK)

## Lote 17n (2026-10-24 17:17)
- 17m: centipede1 = 35 classnames; 22 SEM classe (Enemies_*, Item_*, ambient_*, emitter_*, trigger_catmessage) -> Object; centipede_actor1 = Enemies_Centipede (c_centipede.tik)
- 17m: getClassDef (g_spawn.cpp:204) cai em Object::ClassInfo e tenta classname via TIKI (formato Alice diferente)
- 17n: tabela lg_alice_cls + LG_AliceClass sob NXDK; teste com 1 entrada Enemies_Centipede -> Actor; 'LG: cls X -> Y ok/FALHOU' (+24 -0); XBE 4554752
- Plano 17o: se Actor sobreviver -> Enemies_* -> Actor, ambient_*/emitter_* -> script_model, trigger_catmessage -> trigger_multiple

## Lote 17o (2026-10-24 18:15)
- 17n xemu: 'cls Enemies_Centipede -> Actor ok' -> 'SE Can't find anim/pain.scr' -> throw: Actor MOHAA exige scripts de IA (global/anim) inexistentes no Alice
- Decisao: Actor/IA FAKK2 adiado (lote proprio); inimigos como script_model (modelo + anim, sem IA)
- 17o: 8 Enemies_* -> script_model; lookup getClassForID -> getClass; print cls limitado a 3 (+10 -3); XBE 4554752

## Lote 17p (2026-10-24 18:22)
- 17o xemu: inimigos como ScriptModel OK (cev ... ScriptModel); Complete 13.68 s; heartbeat 41..242 (+40/2s)
- 17p: +14 entradas (8 ambient_*, emitter_SmallFire1, 4 Item_* -> script_model; trigger_catmessage -> trigger_multiple) -> 0 Object no centipede1; XBE 4554752
- Pendente: Item_* com logica real (pickup), Actor/IA FAKK2, build cliente

## Lote 18a (2026-10-24 18:24)
- 17p xemu: MARCO - centipede1 com 0 entidades Object; Complete 12.70 s; Hitch 5908 ms; heartbeat 41..283 (+40/2s); trigger_multiple OK
- Dedicado encerrado como prova de boot; inicio do BUILD CLIENTE
- 18a: inventario (options CMake, sdl.cmake, pastas cliente/renderer, usos SDL/qgl/OpenAL, GetRefAPI/dlopen, NV2A/input do ioQuake3-Xbox) em docs/probes/lote18a-cliente.log, sem build

## Lote 18b (2026-10-24 18:28)
- 18a: cliente = renderergl1 (32 arq, 968 qgl) + code\sdl (5 arq, 354 SDL_) + sys (39) + renderercommon (4); GetRefAPI estatico com USE_RENDERER_DLOPEN=OFF (cl_main.cpp:3340)
- 18a: opcoes Xbox = CLIENT ON, SERVER OFF, GL2 OFF, RENDERER_DLOPEN OFF, OPENAL OFF, HTTP OFF, OPUS OFF; sdl.cmake precisa retornar sob NXDK
- 18a: contagem OpenAL = falso positivo (regex al[A-Z]); filtro ioq3 casou 'Xbox' do nome da pasta -> invalido
- 18a: ioq3 tem sys\xbox_input.c (usbh_lib), xbox_input_engine.c, xbox_snd.c (XAudioProvideSamples)
- 18b: coleta focada ioq3 (sys\xbox*, GLimp/pbkit, build) + exports code\sdl + qgl.h (docs/probes/lote18b-nv2a.log), sem build

## Lote 18c (2026-10-24 19:06)
- 18b: ioq3-Xbox tem RENDERER NV2A PROPRIO (code\renderernv2a, 7 arq ~260 KB: backend/DXT/shader/sky/world Q3/MD3) sobre xgu/pbkit; ligado via sys\xbox_ref.c; input usbh (xbox_input.c), audio XAudio (xbox_snd.c)
- DECISAO: NAO portar qgl do GL1 (968 usos); adaptar renderernv2a ao refexport_t do OpenMoHAA (world -> BSP FAKK v42; MD3 -> TIKI/skeletor em lote proprio); xbox_input/xbox_snd substituem code\sdl
- 18c: coleta tr_public.h (ioq3 x OpenMoHAA), xbox_ref.c, xbox_nv2a.h, input/snd, chamadas IN_/SNDDMA_/GLimp_ (docs/probes/lote18c-refapi.log), sem build

## Lote 18d (2026-10-24 19:08)
- 18c: refexport_t OpenMoHAA = 85 entradas (REF_API 14 x ioq3 8); xbox_ref.c cobre ~30; assinaturas mudam: AddRefEntityToScene(+parent), AddPolyToScene(qboolean), AddLightToScene(+type), DrawStretchRaw(components), MarkFragments(+fRadiusSquared); ~55 non-ioq3 = stubs
- 18c: xbox_input_engine.c cobre IN_Init/Frame/Shutdown/Restart; faltam IN_MouseOn/Off/GetMousePosition/IsCursorActive/FreeCursor/SetCursorFromImage/ToggleMouse; xbox_snd_engine.c cobre SNDDMA_Init/GetDMAPos/Shutdown, faltam BeginPainting/Submit/Activate/MasterGain/captura
- PLANO: 18e cliente com renderer NULO (85 stubs) + stubs IN_/SNDDMA_/GLimp_ -> validar client/cgame/uilib no nxdk + loopback g_gametype 0; 18f input/audio reais; 18g+ renderer NV2A (2D -> mundo FAKK)
- 18d: coleta probe.ps1, client/renderer cmake, CMakeLists, code\sys, tipos (docs/probes/lote18d-clientbuild.log), sem build

## Lote 18e (2026-10-24 19:14)
- 18d: probe.ps1 com flags fixas; client.cmake inclui sdl_input/sdl_mouse + cl_gamespy; renderer_common exige 1 renderer estatico; main em sys_main.c:782
- 18e: sob NXDK sdl.cmake retorna; renderer_gl1.cmake -> code/renderernull/tr_null.c (85 entradas, stub generico + float/struct/nomes proprios); client.cmake sem sdl_input/sdl_mouse/cl_gamespy
- 18e: probe.ps1 -Cliente (CLIENT+GL1 ON); dedicado inalterado sem -Cliente; build diagnostico (link do cliente deve falhar) em docs/probes/lote18e-openmohaa-cliente.log; prototipos em lote18e-proto.log; sem XBE

## Lote 18f (2026-10-25 06:24)
- 18e: cliente CONFIGUROU; 510 obj ok / 16 falhos; link nao alcancado; erros = launcher (spawn.h/cout), SDL.h em sys_local/sys_main/sys_loadlib, sys/mman.h (mumble), al.h (qal), str[uintptr_t] em cl_uimpmappicker.cpp:126
- 18e xemu (XBE dedicado anterior): estavel, Complete 12.77 s, heartbeat 41..202
- 18f: launcher.cmake retorna sob NXDK; includes SDL sob #ifndef NXDK; libmumblelink.c + qal.c fora; contexto em docs/probes/lote18f-ctx.log; build em lote18f-openmohaa-cliente.log; sem XBE

## Lote 18g (2026-10-25 06:40)
- 18f: 502 ok / 9 falhos; restam SDL_VERSION_ATLEAST (sys_local.h), SDL_* em sys_main/sys_main_new/sys_curl/con_log, putenv (cl_main.cpp:1247), str[uintptr_t] (mappicker 126/131), al.h via snd_openal_new.h (snd_dma_new/snd_openal_new)
- 18g: sys_local.h bloco SDL sob !NXDK; sys/* do cliente com COMPILE_DEFINITIONS DEDICATED (sem SDL, igual dedicado); lg_putenv no lg_compat.h; currentDirectory[(int)i]; -I do al.h interno se existir (docs/probes/lote18g-patches.log)
- Risco: sys_main.c com DEDICATED pode omitir Sys_GLimp*/IN_* do cliente -> aparecem no link (stubs no proximo lote)

## Lote 18h (2026-10-25 07:55)
- 18g: MARCO - cliente COMPILA inteiro (502 obj, 0 falhos); so o link do openmohaa.exe falha: al*/alc* (dllimport), QAL_*, ServerList*/ServerGet*/gcd_/GS_* (GameSpy UI), IN_*, Sys_GLimp*/PumpMessageLoop/LowPhysicalMemory, GetGameAPI
- 18h: code/renderernull/lg_client_stubs.c (74 stubs, retorno 0/NULL/""); AL_LIBTYPE_STATIC no cliente; game estatico no openmohaa (CMakeLists.txt)
- 18h: se linkar -> xbe-out\cliente.xbe (ISO do dedicado intacta); duplicatas/cxbe em docs/probes/lote18h-extras.log

## Lote 18i (2026-10-25 10:26)
- 18h: MARCO - CLIENTE LINKADO: 503 obj / 0 falhos, 0 duplicatas; openmohaa.exe 5415936; cliente.xbe 5427200 (cxbe = $NXDK_DIR/tools/cxbe/cxbe OK)
- 18i: ISO separada G:\xemu\alice-cliente.iso (iso-cliente = default.xbe do cliente + hardlinks de iso-stage\main); ISO do dedicado intacta
- Esperado xemu: boot LG -> 'LG: renderer nulo (18e)' -> Com_Init -> auto map centipede1 -> heartbeat (tela de debugPrint, sem grafico)

## Lote 18j (2026-10-25 11:28)
- 18i xemu: MARCO - cliente.xbe BOOTA; centipede1 carrega; Complete 12.48 s; Hitch 5783 ms; heartbeat 41..242 (+40/2s)
- 18i: tela igual a do dedicado; 'renderer nulo' nao visivel; 'No bots' = caminho SV_ServerLoaded do dedicado -> suspeita com_dedicated=1 (sys_main.c com DEDICATED, 18g)
- 18j 1a tentativa: Select-String abortou (sys_xbox.c fica em tools\, nao em code\sys); nada aplicado
- 18j: heartbeat imprime 'LG: ded=X cl=Y' (+1 -0); coleta em docs/probes/lote18j-ded.log
- Regra PS: com ErrorActionPreference Stop, filtrar -Path com Test-Path antes do Select-String

## Lote 18l (2026-10-25 11:44)
- 18j xemu (ISO certa): MARCO - CLIENTE REAL: Client Initialization -> 'LG: renderer nulo (18e)' -> Sound Init sem OpenAL (0 ms) -> QKEY found
- 18j: 'ERROR: Couldn't load font Verdana' -> Unloading ClientGame x3 -> lixo na tela -> REBOOT (volta ao 'OpenMoHAA Xbox boot')
- 18i 'igual ao dedicado' = xemu com ISO errada; 18k descartado (nao rodado)
- 18l: coleta origem do erro de fonte, fonts/ nos pk3, 'verdana' em urc/cfg, caminho Sys_Error->reboot (docs/probes/lote18l-font.log), sem build

## Lote 18m (2026-10-25 17:04)
- 18l: verdana-*.ritualfont EXISTEM nos pk3; erro = stub LoadFont do renderer nulo (NULL) -> uifont.cpp:270 ERR_DROP -> reboot
- 18m 1a: regex '.LoadFont =' nao casou; 2a: fontheader_s sem height/aspectRatio (so no fontheader_sgl_s); 3a: atribuicao inserida no return de um stub int (IndexOf GetRefAPI pegou prototipo)
- 18m 4a: lg_null_LoadFont (fontheader_t estatico, so name, cache 16) ligado na variavel refexport_t real (designated ou antes do return dela)
- Pendente: Sys_PlatformExit vazio / ERR_DROP->reboot apaga a mensagem

## Lote 18n (2026-10-26 19:32)
- 18m xemu: fonte Verdana OK (erro sumiu); novo: 'Called FadeSound with: 0.000000' -> 'ERRO: Error during initialization' (Com_Error dentro do setjmp do Com_Init, msg perdida)
- 18m: rerun do 4a quebrou o ';' da linha 76 -> restaurado via git checkout (regra: scripts de patch idempotentes)
- 18n: Sys_Error de init imprime com_errorMessage (+1 -1)

## Lote 18p (2026-10-27 10:33)
- 18n xemu: erro real = '::LoadFile : Couldn't load global/inventory.txt' (FadeSound 0.0 = so a linha anterior)
- 18o: unico chamador cl_ui.cpp:4163 (ui_inventoryfile, default MOHAA); Alice so tem global/inventory.scr vazio
- 18p 1a: patch OK, mas trava esperava +4 -0; bloco #ifdef/if/else/#else/#endif = +5 -0 (linha original mantida no #else)
- 18p: sob NXDK CL_LoadInventory so se FS_ReadFile > 0, senao 'LG: sem X (inventario vazio)' (+5 -0)
- Pendente: inventario/HUD do Alice (formato proprio) em lote futuro

## Lote 18r (2026-10-27 10:48)
- 18p xemu: 'LG: sem global/inventory.txt (inventario vazio)' OK; depois TRAVA (minutos, sem heartbeat)
- 18q/18q2: UI_LoadInventory_f termina limpo; chamador = CL_InitializeUI (cl_ui.cpp:5315, chamado de cl_main.cpp:2946)
- Suspeitos pos-5315: UI_CreateScoreboard, View3D/RegisterShader (renderer nulo), parse urc (FAKK), CreateMenus, CL_TryStartIntro
- 18r: 5 breadcrumbs 'LG: ui 1..5' com t= (+5 -0)

## Lote 18s (2026-10-27 10:53)
- 18r xemu: nenhum 'LG: ui' visivel; ui 1 (3938) e ui 4 (3562) cairam FORA do CL_InitializeUI (regex pegou 1a ocorrencia do arquivo)
- 18r: ui 2 (5396, pos-View3D) nao apareceu -> trava entre 5315 e 5396 (scoreboard/game_version/Cmd_AddCommand/devcon/View3D)
- Regra: ancoras de patch buscadas a partir do inicio da funcao alvo (Regex.Match(txt, ini))
- 18s: ui 1/4 reposicionados + ui 1b (game_version) + ui 1c (cmds, dev=) (+4 -2)

## Lote 18t (2026-10-27 11:06)
- 18s xemu: ui 1 / 1b / 1c (dev=0) t=2752-2753, ui 2 nunca -> trava no bloco View3D (new/bottom/InitFrame facfont-20/setName/InitSubtitle)
- Hipotese: fontheader dummy do renderer nulo (so name, metricas 0) -> laco de layout infinito
- 18t: breadcrumbs 'LG: v3 new/bottom/initframe/name' (+4 -0); coleta View3D/InitFrame/UIFont/lg_null_LoadFont em lote18t-view3d.log

## Lote 18u (2026-10-27 11:12)
- 18t xemu: v3 new/bottom/initframe/name OK (t=2721-2724), ui 2 nunca -> trava em View3D::InitSubtitle (cl_uiview3d.cpp:806, laco getCharWidth A..Z)
- Hipotese: fontheader dummy (so name) -> getCharWidth le tabela/ponteiro NULL -> congelamento
- 18u: sob NXDK totalWidth=260 (avWidth 10), laco pulado (+4 -0); coleta getCharWidth/fontheader_t em lote18u-subtitle.log p/ fonte dummy real

## Lote 18v (2026-10-27 11:59)
- 18u xemu: 'ui 2 view3d ok t=2862' -> InitSubtitle era a trava; ui 3 nunca -> nova trava apos View3D
- 18u: getCharWidth/getHeight leem m_font->sgl[0] (NULL na fonte dummy) -> toda medicao de texto congela
- 18v: lg_null_sgl[16]; cada fonte dummy com sgl[0] (indirection=c, largura 8px, height 16, aspect 1); contorno 18u mantido; trecho ui2..ui3 em lote18v-fonte.log

## Lote 18w (2026-10-27 12:09)
- 18v xemu: MARCO - UI do cliente INICIALIZA: ui 3 (13 urc) / ui 4 menus / ui 5 intro; Client Init Complete; Common Init Complete 4147 ms; frame 1; ded=0 cl=1; auto map centipede1
- 18v: 'register sound ... soundsystem hasn't started' nao fatal (sem OpenAL); EAlogo ausente (intro)
- 18v: 'ded=0 cl=1' impresso TODO frame -> esconde o pos-map e derruba fps
- 18w: linha ded/cl one-shot em code/qcommon/common.c (+1 -1)

## Lote 18x (2026-10-27 12:18)
- 18w xemu: ded/cl one-shot OK; cliente ocioso ~90 fps (renderer nulo); auto map -> Server Init -> InitGame -> FAKK repack -> cls Item_* -> script (cmd?e/nul/cev) OK
- 18w: FATAL '::LoadFile : Couldn't load global/mike_Legs.st' -> Server crashed: Player MOHAA exige statefile mike_*; Alice tem statefiles proprios
- 18x: coleta .st nos pk3 + g_statefile/GetStatemap (docs/probes/lote18x-statefile.log), sem build

## Lote 18y (2026-10-27 12:33)
- 18x: pak0 tem global/alice_legs.st (8028) + alice_torso.st (20364) (+ jay/joe/old); g_statefile default 'global/mike' (gamecvars.cpp:449) -> player.cpp:5069-5072
- 18x: .st do Alice usam condicoes FAKK/Alice (CHESSPUZZLE, ENV_SLIDING, IS_WEAPON_ACTIVE, LOOKINGGLASS, TEA, RAGE...) -> risco de erro no GetStatemap
- 18y: g_statefile = global/alice sob NXDK (+4 -0); erros do statemap.cpp no log (alvo 18z)

## Lote 18z (2026-10-27 13:40)
- 18y xemu: alice_Legs.st CARREGADO (mike sumiu); fatal 'global/alice_Legs.st: Unknown condition CHESSPUZZLE on line 13' -> Server crashed
- 18y: coleta statemap.cpp vazia (arquivo fora de code\fgame)
- 18z: origem Unknown condition + tabela m_conditions x condicoes dos .st Alice (faltam=55) em docs/probes/lote18z-condicoes.log, sem build
- Plano 18z2: condicao desconhecida -> aviso 'LG: st? X' + condicao sempre falsa sob NXDK

## Lote 18z2 (2026-10-27 13:44)
- 18z: Unknown condition = characterstate.cpp:136-145 (addCondition==0 -> gi.Error ERR_DROP)
- 18z: tabela m_conditions nao achada (offset -1) -> 'faltam 55' = todas as usadas, lista invalida
- 18z2: sob NXDK aviso 'LG: st? X' (5 max) + condition_index 0 mantido; getResult retorna false se cond index 0 (+7 -0)

## Lote 18z3 (2026-10-27 16:23)
- 18z2 xemu: alice_Legs.st PARSEADO (st? CHESSPUZZLE/ENV_SLIDING/ENV_THERMAL/ALTITUDE/31 - '31' = parametro de ALTITUDE, inofensivo)
- 18z2: fatal 'alice_Torso.st: Unknown command torso on line 6' -> FAKK usa torso{}, MOHAA usa action{}
- 18z3: characterstate.cpp:672 icmp("action") || icmp("torso") sob NXDK (+4 -0); comandos do State: nextstate/movetype/camera/legs/action/behavior/time/states/entry/exitcommands
- 18z3 1a: trava esperava 11 0, mas diff e contra HEAD (18z2 ja commitado) -> real 4 0; regra: numstat conta so o lote atual

## Lote 18z4 (2026-10-28 07:48)
- 18z3 xemu: torso OK (alice_Torso.st ate linha 93); fatal 'Unknown camera type frontright on line 93'
- 18z4: gi.Error de camera/movetype desconhecidos -> 'LG: cam?/mv? linha N' (5 max) sob NXDK, tipo fica DEFAULT (2 sites, +8 -0)
- readCamera/readMoveType completos em docs/probes/lote18z4-camera.log (mapear tipos Alice depois)

## Lote 18z5 (2026-10-28 09:04)
- 18z4 xemu: cam? OK (passou linha 93); fatal 'Unknown command -70 on line 506' -> argumento extra de comando FAKK (camera/movetype com parametros) lido como comando
- 18z5: Unknown command do State -> 'LG: stc? X linha N' (5 max) + descarta resto da linha; readCamera/readMoveType desconhecidos tambem descartam a linha (4 insercoes, 10	0	code/fgame/characterstate.cpp)
- Linhas 90-95 e 500-510 dos alice_*.st em docs/probes/lote18z5-stcmd.log

## Lote 18z6a (2026-10-28 09:59)
- 18z5 xemu: MARCO - alice_Legs.st + alice_Torso.st PARSEADOS (linha 506 = 'camera bulletcam -70 0 20 ...', descarte de linha OK)
- 18z5: fatal 'Sys_GetCGameAPI(cgame.dll) failed: Extension not allowed' -> cgame ainda via DLL (18h so tornou o game estatico)
- 18z6a 1a: lg_game_rename.h nao esta em tools\ (Get-Content com Path nulo) -> busca nos 2 repos
- 18z6a: coleta Sys_Get*API + GetCGameAPI + CMake cgame + local do rename (docs/probes/lote18z6a-cgame.log), sem build
- Plano 18z6b: Sys_GetCGameAPI -> GetCGameAPI estatico sob NXDK + libcgame no link (risco: dups game x cgame)

## Lote 18z6b (2026-10-28 10:13)
- 18z6a: Sys_GetGameAPI ja estatico (sys_main_new.c:240); GetCGameAPI(void) SEM parms (cg_main.c:814); cgame fora do link
- 18z6b 1a-3a: rename usa prefixo G_ (nao lg_game_); bloco colado isolado = variaveis nulas (regra: rodar script inteiro)
- 18z6b: Sys_GetCGameAPI -> GetCGameAPI() sob NXDK (+3 -0); lg_cgame_rename.h (397 G_->LGC_, guard CGAME_DLL); cgame no link do openmohaa (+5 -0)
- 18z6b: link=False (se False: dups em docs/probes/lote18z6b-openmohaa-cliente.log)

## Lote 18z6c (2026-10-28 10:20)
- 18z6b: link falhou: dups BG_SOURCES (game+cgame: VoteOptions/VoteOptionListItem/SingleVoteOption/CacheResource/pmove_msec/pmove_fixed/paused) + _GetCGameAPI indefinido no omohaaded (sys_main_new.c compartilhado)
- 18z6c: +7 nomes LGC_ no lg_cgame_rename.h; stub GetCGameAPI (NULL) so no omohaaded (lg_cgame_stub_ded.c); link=True

## Lote 18z7a (2026-10-28 10:30)
- 18z6c xemu: MARCO - CLIENTE NO JOGO: 'Client 0 (IP: localhost) is connecting' (loopback) -> FS restart (9 pk3/45604) -> CL_InitCGame 0.06 s -> autosave centipede10001 'Game Saved' -> heartbeat 1451..1547
- cgame estatico OK (XBE 6139904); Allied/Axis model invalid = cvars MP do MOHAA, inofensivo
- Proximos: (a) confirmar heartbeat estavel pos-save; (b) renderer NV2A (2D: fonte/shader/StretchPic) no lugar do nulo; (c) input xbox_input (pad); (d) audio xbox_snd; (e) limpeza dos avisos LG:

## Lote 18z8a (2026-10-28 10:39)
- 18z7a xemu: heartbeat ESTAVEL pos-save 1726..5366 (t=34-75 s, +182/2s ~91 fps, renderer nulo)
- Decisao: input DEPOIS do grafico; ordem = NV2A 2D -> input -> audio -> limpeza LG:
- 18z8a 1a: '-f' dentro de Add(a -f x, y) = virgula vira argumento do metodo -> regra: Add((fmt -f x, y))
- 18z8a 2a: Get-ChildItem -Include sem -Recurse = vazio -> Path nulo; regra: -Include sempre com -Recurse + helper que tolera lista vazia
- 18z8a: coleta refexport_t, uso re.* 2D, fonte GL1, renderernv2a/xbox_ref.c/init pbkit, build ioQ3, tr_null.c (docs/probes/lote18z8a-nv2a2d.log), sem build

## Lote 18z8b (2026-10-28 12:37)
- 18z8a: 2D usado = SetColor/DrawStretchPic/DrawTrianglePic/DrawString/RegisterShader(NoMip)/GetShaderWidth/Height/DrawBox/DrawTilePic/Scissor
- 18z8a: xbox_ref.c = casca; init pb_init/pb_show_front_screen em xbox_nv2a.c:2334-2339; swap vbl/back_buffer/reset/target 2519-2539
- 18z8a: secao 4 vazia = regex \bNV2A_ nao casa dentro de XboxNV2A_ (sem fronteira de palavra)
- DECISAO: 2D minimo proprio no tr_null.c (pb_init, clear, DrawStretchPic = pb_fill solido com SetColor, swap); renderernv2a fica como referencia
- 18z8b: coleta xbox_nv2a.h + corpos Init/BeginFrame/EndFrame/SetColor/DrawStretchPic + link probe.ps1 + pbkit no nxdk + tr_null.c (docs/probes/lote18z8b-nv2a-corpo.log), sem build

## Lote 18z8c (2026-10-28 12:42)
- 18z8b: pbkit.h SEM pb_fill -> retangulos por CPU no pb_back_buffer(); libpbkit.lib existe no nxdk (adicionada ao link do probe.ps1)
- 18z8b: sequencia ioQ3 = pb_init/show_front_screen; Begin: wait_vbl/reset/target_back_buffer; End: laco pb_finished com timeout
- 18z8c: tr_null.c sob NXDK: SetColor/BeginFrame/EndFrame/DrawStretchPic(2)/DrawTilePic/DrawBox reais; fundo 0x101020; cor = hash do shader x SetColor; alpha<0.5 pulado
- 18z8c: debugPrint some apos pb_init -> overlay pbkit 'LG 18z8c frame N' + barra verde no rodape = heartbeat visual
- 18z8c: exe=True; ISO G:\xemu\alice-cliente.iso (docs/probes/lote18z8c-nv2a2d.log)

## Lote 18z8d (2026-10-28 12:59)
- 18z8c xemu: MARCO - pbkit OWNS SCREEN: fundo 0x101020 + overlay 'LG 18z8c frame 840' + barra verde varrendo a tela sem travar (init/flip/loop OK)
- 18z8c: NENHUM retangulo de UI/HUD visivel -> suspeitos: RegisterShader stub=0, alpha<0.5, ou 2D por outra entrada
- 18z8d: overlay com fps + fill/draw/alpha/sh0/setcolor/shaders por frame; RegisterShader(NoMip) = tabela de nomes (512, handle i+1)
- 18z8d: exe=True (docs/probes/lote18z8d-contadores.log)

## Lote 18z8e (2026-10-28 15:21)
- 18z8d xemu: fps 2 (era ~91 no nulo); fill 7 draw 1 alpha 6 sh0 7 setcolor 11 shaders 3
- 18z8d: 1 retangulo tela cheia sh0 = fade do jogo (tela preta, esmaece rosa->laranja = SetColor); nenhum StretchPic com shader real; so 3 shaders (UI registra por outra via?)
- Suspeita fps: laco pb_finished sem comandos na GPU sai so por timeout, ou wait_vbl
- 18z8e: overlay linha 4 'vbl/fin/game ms + to (timeouts)'; timeout pb_finished 2000 -> 50 ms
- 18z8e: exe=True (docs/probes/lote18z8e-tempos.log)

## Lote 18z8f (2026-10-28 16:23)
- 18z8e xemu: fade fps 4 (draw 2) / ocioso fps 22 (draw 0); vbl 1-17, fin 0, game 1-4, to 0 -> pb_finished/vbl/jogo INOCENTES
- Causa: preenchimento por CPU no framebuffer (~100 ms por tela cheia no xemu); clear do fundo ja limita a 22 fps
- Plano 18z8g: clear de fundo + retangulos solidos via GPU (SET_COLOR_CLEAR_VALUE + CLEAR_RECT + CLEAR_SURFACE); alpha depois via triangulos
- 18z8f: coleta lg2d_fill/Begin/EndFrame + macros nv_regs/pbkit (docs/probes/lote18z8f-gpuclear.log), sem build

## Lote 18z8g (2026-10-28 16:28)
- 18z8f: grep NV097_* vazio (padrao exigia 1 espaco); pb_begin/pb_push nao achados no pbkit.h; clear D/S = trigger 0x03
- 18z8g: lg2d_gpufill = push manual (n<<18|subch 0<<13|mtd): 0x1D98 rect (x2-1<<16|x1, y2-1<<16|y1) + 0x1D8C zs/cor/trigger 0xF0 (RGBA)
- 18z8g: lg2d_fill e clear do BeginFrame sem CPU; overlay 'LG 18z8g'; exe=True (docs/probes/lote18z8g-gpufill.log)
- Esperado xemu: ocioso ~60 fps (vbl), fade sem queda; cores iguais ao 18z8d

## Lote 18z8h (2026-10-28 17:02)
- 18z8g xemu: MARCO - fill/clear GPU OK: ocioso 60-61 fps (era 22), fade 60 fps (era 4); cores corretas
- 18z8g: queda pontual fps 8 com game=86 ms (vbl 11, fin 0) = hitch do jogo (load/shaders 3->19), NAO do renderer
- 18z8h: coleta p/ StretchPic texturizado (tr_null registro, loaders GL1, upload/quad/blend do renderernv2a, imagens ui nos pk3) em docs/probes/lote18z8h-textura.log, sem build

## Lote 18z8i (2026-10-28 17:49)
- 18z8h: GL1 carrega so .tga/.jpg (tr_image.c:2399 R_LoadImage); .ftx so em textures/menu; ioQ3 upload = swizzle + MmAllocateContiguousMemoryEx + xgu_set_texture_*
- 18z8i: lg_ri capturado no GetRefAPI; RegisterShader le <nome>[.tga] via FS_ReadFile e decodifica cabecalho TGA (tipo/w/h/bpp); overlay linhas 4-5; exe=True
- Proximo 18z8j: upload RGBA swizzled + quad texturizado com blend

## Lote 18z8j (2026-10-28 18:08)
- 18z8i xemu: fps 60 ocioso; tga ok 1 / fail 18 de 19; nomes = UI MOHAA (mohaa_title, legal, textures/menu/loading, textures/hud/radar_axis) -> urc/cgame do OpenMoHAA pedem assets inexistentes no Alice
- 18z8i: overlay t2/b24 com -1x-1 = bug (fail so zerava w/h); o unico ok e TGA tipo 2 24 bpp
- 18z8j: status por shader (T/J/-) nas linhas 5+ do overlay; fallback .jpg; refs dos nomes nos .shader/.urc dos pk3 em docs/probes/lote18z8j-shaders.log; exe=True

## Lote 18z8k (2026-10-28 18:28)
- 18z8j xemu: so gfx/2d/backtile = T (TGA 2, 64x64, 24 bpp); 0 REF nos .shader/.urc -> nomes MOHAA vem do codigo C cliente/cgame, nao de script (parser .shader adiado)
- 18z8j: 'radar_axis.tgagarop' = sobra de texto do pbkit (linha nao limpa) -> padding %-34.34s
- 18z8k: lg2d_decode (TGA 2/10, 24/32 bpp, ate 256x256, ARGB, flip por origem) no RegisterShader; lg2d_dbgblit CPU antes do overlay (canto inferior direito); overlay 'dec N'; exe=True
- Proximo: upload swizzled NV2A + quad texturizado no DrawStretchPic

## Lote 18z8l (2026-10-28 18:47)
- 18z8k xemu: MARCO - decode TGA OK: dec 1, backtile 64x64 no canto, orientacao/cores BGR/pitch 640 corretos; fps 62; padding do overlay OK
- 18z8l: coleta upload swizzled/combiner/quad do renderernv2a + API xgu do nxdk + corpos atuais do tr_null.c (docs/probes/lote18z8l-xgu.log), sem build
- Proximo 18z8m: textura NV2A + quad texturizado no DrawStretchPic

## Lote 18z8m-a (2026-10-28 18:49)
- 18z8l: upload ioQ3 = MmAllocateContiguousMemoryEx(WRITECOMBINE) + swizzle por tabela X|Y + A8R8G8B8_SWIZZLED (ARGB = mesmo do lg2d_decode); offset GPU = ptr & 0x03ffffff
- 18z8l: bind = xgu_set_texture_offset/format/address/control0/filter; blend = xgu_set_blend_func_s/dfactor + blend_enable; draw = xgux_draw_arrays
- 18z8l: xgu.h NAO esta em NXDK_DIR/lib/xgu -> 18z8m-a localiza xgu (container/ioQ3) + linhas 500-660/1100-1150/2220-2400 do xbox_nv2a.c (docs/probes/lote18z8m-a-xgu.log), sem build

## Lote 18z8m-b (2026-10-28 18:52)
- 18z8m-a: xgu = copia propria do ioQ3 (code/thirdparty/xgu: xgu.h, xgux.h, nv2a_regs.h), nao vem do nxdk -> copiar p/ code/renderernull/xgu
- 18z8m-a: ioQ3 usa vertex program (xbox_nv2a_vp.inl) + fragment program (xbox_nv2a_fp.inl gerado de .ps via fp20compiler); setup de frame 2276-2309; bind/blend 532-620
- 18z8m-b: coleta .inl/.ps, regras do Makefile, compiladores no container, assinaturas xgu e defs ioQ3 (docs/probes/lote18z8m-b-shaders.log), sem build

## Lote 18z8m-c (2026-10-28 19:05)
- 18z8m-b: vp.inl (vp20, 10 instr, m_viewport c[0]=slot 96) e fp.inl (tex0*col0*tex1) JA GERADOS no ioQ3 -> copiados, sem fp20compiler
- 18z8m-c: code/renderernull/xgu (xgu.h/xgux.h/nv2a_regs.h) + .inl; lg_nv2a.c (lgx_init/lgx_upload swizzle/lgx_frame/lgx_quad, branco 1x1 na unidade 1) incluido no TU do tr_null.c (+3 -0)
- 18z8m-c: SEM chamadas (comportamento igual); exe=True; sem XBE/ISO (docs/probes/lote18z8m-c-xgu.log)
- Proximo 18z8m-d: lgx_init no Init, lgx_upload no RegisterShader (lg2d_decode), lgx_frame no BeginFrame, lgx_quad no DrawStretchPic

## Lote 18z8m-d-a (2026-10-28 19:41)
- 18z8m-c: build 504/0, exe=True (42dea8b); pb_begin/xgu/enums resolvem (grep vazio do pbkit.h = declarado em outro header)
- 18z8m-d-a: coleta tr_null.c 1-240 (buffer do lg2d_decode, SetColor, Begin/EndFrame) + pb_begin nos headers pbkit (docs/probes/lote18z8m-d-a-trnull.log), sem build
- Proximo 18z8m-d: ligar lgx_init/upload/frame/quad com nomes reais

## Lote 18z8m-d (2026-10-28 19:46)
- 18z8m-d-a: pb_begin/pb_end em pbkit_pushbuffer.h; buffer do decode = lg_img/lg_iw/lg_ih[handle-1]
- 18z8m-d: prototipos lgx_* apos lg_img; lgx_init no lg2d_init (lgx_st 1/-1); lgx_upload no RegisterShader (pot. de 2 -> lg_tex[]); lgx_frame apos clear; DrawStretchPic = lgx_quad c/ fallback fill
- 18z8m-d: overlay 'LG 18z8m' + linha 2 'q N up N xS'; exe=True iso=True (docs/probes/lote18z8m-d-quad.log)
- Esperado xemu: x1 up 1; backtile texturizado onde o jogo o desenha (q>0); se x-1 -> lgx_init falhou

## Lote 18z8m-e (2026-10-28 19:57)
- 18z8m-d xemu: x1 up 1 q 0 draw 0 fps 62 -> lgx_init/upload OK; jogo nunca chama StretchPic c/ backtile (so fade sh0); canto = blit CPU
- 18z8m-e: quad forcado do 1o lg_tex (128x128 em 200,150, branco) no EndFrame apos barra verde; overlay 'LG 18z8m-e'; exe=True iso=True
- Esperado xemu: backtile 128x128 (escalado 2x) em 200,150 + q 1; senao vp/matriz slot 96/estado xgu

## Lote 18z8n-a (2026-10-28 20:09)
- 18z8m-e xemu: MARCO - QUAD TEXTURIZADO NV2A OK: backtile 128x128 em 200,150, cores certas, q 1, fps 63-64 (upload swizzle/vp slot 96/combiner/bind validados)
- 18z8m-e: jogo segue draw 0 (so fade sh0) -> 18z8n-a coleta entradas refexport reais x stub + uso re.* cliente/cgame/uilib + shaders do cgame (docs/probes/lote18z8n-a-entradas.log), sem build

## Lote 18z8n-b (2026-10-28 20:12)
- 18z8n-a: atribuicoes refexport nao casaram (GetRefAPI usa outra forma); DrawTrianglePic (22) e DrawString (21) = stub; shaders cgame todos MOHAA
- 18z8n-a: draw 0 = DrawStretchPic nunca chamado -> suspeita SCR/CG_Draw2D pulando 2D por estado
- 18z8n-b: coleta GetRefAPI/LG_NullStub/LG2D_DrawStretchPic + SCR_DrawScreenField/UpdateScreen + CG_DrawActiveFrame/CG_Draw2D (docs/probes/lote18z8n-b-refapi.log), sem build

## Lote 18z8n-c (2026-10-28 20:18)
- 18z8n-b: GetRefAPI preenche 85 entradas com LG_NullStub (ret 0) e troca ~22 via LG_SET; GetShaderWidth/Height = 1 (pendente)
- 18z8n-b: CG_Draw2D so via cl_cgame.cpp:1023 e cl_uiview3d.cpp:584; CG_DrawActiveFrame sai sem snap
- 18z8n-c 1a: ancora 'LG 18z8m-e' casou o comentario do quad forcado -> nada gravado; ancora agora = literal '"LG 18z8m-e frame'
- 18z8n-c: lg_dbg[8]; linha vbl do overlay -> 's a r d c v' (clc.state/DrawActiveFrame/sai sem snap/CG_Draw2D/via cl_cgame/via View3D, %10000); exe=False iso=False

## Lote 18z8n-c2 (2026-10-28 20:27)
- 18z8n-c: build falhou so no tr_null.c:187 (lg_dbg nao declarado: 'ultimo #include' = lg_nv2a.c, abaixo do overlay; 5 erros 'unsigned int *' = correcao para lg_img do clang)
- Regra: declaracao global nova em tr_null.c vai no TOPO do arquivo (nao apos o ultimo #include)
- 18z8n-c2: lg_dbg[8] movido p/ linha 1 do tr_null.c; demais 5 arquivos do 18z8n-c mantidos; exe=True iso=True

## Lote 18z8n-d (2026-10-28 20:39)
- 18z8n-c2 xemu: carregando s9 (CA_CINEMATIC) a0 d0; em jogo s8 (CA_ACTIVE) a1681 r0 d1680 c0 v1680 -> CG_Draw2D roda todo frame (so via View3D), snapshot sempre valido
- Conclusao: 2D NAO e pulado por estado; draw 0 vem de dentro do CG_Draw2D (cvar, shader 0, ou DrawString/DrawTrianglePic stub)
- 18z8n-d: coleta corpo CG_Draw2D + funcoes chamadas + chamadas R_Draw* do cgame + cvars 2D + contexto View3D (docs/probes/lote18z8n-d-draw2d.log), sem build

## Lote 18z8n-e (2026-10-28 20:51)
- 18z8n-d: causa do draw 0 = cg_hud default '0' (cg_main.c:170): Icons/PlayerTeam/EntInfo/Attacker/Crosshair/Stopwatch/HudDrawElements saem cedo; lagometer off; Objectives/Vote/InstaMsg/Zoom dependem de estado
- 18z8n-d: HUD real do MOHAA = UI (urc via 'ui_hud 1', cg_main.c:770), nao cgame
- 18z8n-e: sob NXDK cg_hud=1 e cg_lagometer=1 (+8 -0); overlay 'LG 18z8n-e'; exe=True iso=True
- Esperado xemu: draw>=1 (lagometer 272,432 96x48) + fill maior (DrawBox)

## Lote 18z8o (2026-10-28 20:57)
- 18z8n-e xemu: MARCO - CGAME 2D NO NV2A: draw 73 fill 113 sh0 112 fps 63; lagometer visivel no rodape (fundo fallback + DrawBox azul/verde)
- 18z8n-e: shaders cgame MOHAA inexistentes -> sh0 dominante; so backtile = T
- 18z8o: coleta fonts/ nos pk3 + verdana-14.ritualfont + R_LoadFont/R_DrawString GL1 + fontheader_t (docs/probes/lote18z8o-fonte.log), sem build
- Plano 18z8p: LoadFont real (ritualfont + tga) + DrawString via lgx_quad por caractere

## Lote 18z8p-a (2026-10-28 21:03)
- 18z8o: 16 .ritualfont em pak0 fonts/; formato RitFont = height/aspect/indirections[256]/locations {x y w h} em PIXELS (GL1 usa size*256 -> loader divide por 256, atlas 256 px provavel)
- 18z8o: .ritualfont nao nomeia textura; so fonts/backup/asrafel.tga e courier-14.tga no fonts/ -> textura resolvida em R_LoadFontShader
- 18z8p-a: coleta R_LoadFontShader + parser sgl + struct letterloc/sgl + arquivos de fonte qualquer extensao + topo tr_null.c (docs/probes/lote18z8p-a-fontshader.log), sem build

## Lote 18z8p (2026-10-28 21:13)
- 18z8p 1a: trava '-<=2' rejeitou diff legitimo (83 3 = hook LoadFont + overlay + ultima linha/return) -> trava por conteudo das linhas removidas
- 18z8p-a: locations ja normalizadas (x,w/256; y,h*aspect/256); textura gfx/fonts/<nome>.tga; sem tga (verdana/-10/-14/-20, hapsburg) -> fallback verdana-12
- 18z8p: lgf_load no lg_null_LoadFont; lgf_DrawString = DrawStretchPic por caractere; LG_SET(DrawString); overlay 'LG 18z8p'; numstat 83	3	code/renderernull/tr_null.c; exe=False iso=False

## Lote 18z8p2 (2026-10-28 21:22)
- 18z8p: build falhou (tr_null.c:1 fontheader_t/fontheader_sgl_t desconhecidos): prototipos na linha 1, antes dos includes; script commitou mesmo assim (950db65)
- Regra: global sem tipo do engine -> topo; prototipo com tipo do engine -> logo antes da funcao que usa (apos includes)
- 18z8p2: prototipos movidos p/ antes de lg_null_LoadFont; numstat 2	2	code/renderernull/tr_null.c; exe=True iso=False

## Lote 18z8p2-b (2026-10-28 21:30)
- 18z8p2: build 504/0 OK, mas cxbe falhou ('bash: not found' no lg-nxdk) -> xemu ainda rodava XBE 18z8n-e
- Regra: cxbe via 'sh -c' (sem bash no container); NXDK_DIR default /usr/src/nxdk
- 18z8p2-b: cxbe sem rebuild; xbe=6189056 iso=True

## Lote 18z8q (2026-10-28 21:50)
- 18z8p xemu: MARCO - DrawString REAL: 'Game Saved' em RitualFont; tga ok 4 / dec 4 / up 4; q 10; fps 63 -> 36-38 (suspeita dbgblit CPU 256 px)
- 18z8q 1a/2a: quad forcado = 'if (lgx_quad(...)) lg2d_nq++;' com comentario NO FIM da linha 183 -> regex 'lgx_quad(...);' e busca apos o comentario falharam; nada gravado
- 18z8q: linha inteira do quad + dbgblit -> (void)0; numstat 3	3	code/renderernull/tr_null.c; exe=True iso=True

## Lote 18z9a (2026-10-28 22:00)
- 18z8q xemu: MARCO - 2D NV2A LIMPO: fps 59-60 (era 36-38), atlas forcados sumiram, 'Game Saved' RitualFont visivel (q 9 -> q 0 apos sumir); draw 66-73 fill 103-112
- Confirmado: queda do 18z8p = dbgblit CPU; caminho GPU (fill/quad/DrawString) sem custo de fps
- Pendentes 2D: alpha<0.5 pulado (6/frame), shaders MOHAA inexistentes (sh0), GetShaderWidth/Height=1
- 18z9a: inicio INPUT - coleta xbox_input* do ioQ3 + stubs IN_* + K_JOY/SE_* + usbh/xid do nxdk (docs/probes/lote18z9a-input.log), sem build

## Lote 18z9a2 (2026-10-28 22:02)
- 18z9a: ioQ3 input = xbox_input_engine.c (IN_* -> Sys_XboxInput*) + xbox_input.c (XID 4 portas + HID teclado/mouse, Com_QueueEvent SE_KEY/SE_CHAR/SE_MOUSE/SE_JOYSTICK_AXIS); mapa D-pad->setas, Start->ESC, A/B/X/Y/black/white/LT/RT->K_JOY1..8, menu A=ENTER B=ESC
- 18z9a: OpenMoHAA tem IN_*, Com_QueueEvent, K_JOY1..32, SE_* compativeis; IN_* hoje = LG_Z em lg_client_stubs.c:65-71; nxdk tem nxdk_usb.lib + xid_driver (libusbohci_xbox)
- 18z9a2: coleta xbox_input.c 700-fim + Makefile.xbox USB + probe.ps1 libs + Key_* OpenMoHAA + xid_driver.h/usbh_hid.h + simbolos nxdk_usb.lib (docs/probes/lote18z9a2-input.log), sem build
- Plano 18z9b: copiar xbox_input*.c/h -> code/renderernull (adaptar Key_*), remover IN_Init/Frame/Shutdown/Restart dos stubs, nxdk_usb.lib no link

## Lote 18z9b (2026-10-28 22:16)
- 18z9a2: nxdk_usb.lib SEM usbh_hid_* -> HID fora; probe.ps1 nao linkava nxdk_usb; IN_Init nunca chamado (so IN_Frame em common.c:2351)
- 18z9b 1a: Select-String -SimpleMatch com 'lg_client_stubs\.c' = barra literal -> 0 cmake; regra: -SimpleMatch nunca com escape regex
- 18z9b: code/renderernull/lg_input.c (XID 4 portas; IN_Init lazy, enum 500 ms; botoes->K_JOY1..11/setas/ESC, menu A=ENTER B=ESC, stick esq = cursor; binds MOHAA; j_* do ioQ3)
- 18z9b: IN_* fora dos stubs; include USB + USBH_USE_EXTERNAL_CONFIG em client.cmake; nxdk_usb.lib no probe.ps1
- 18z9b: overlay campo 'r' = pads*1000 + botoes; exe=True iso=True (docs/probes/lote18z9b-input.log)

## Lote 18z9b-resultado (2026-10-28 22:23)
- 18z9b xemu: MARCO - INPUT XID OK: overlay r1062 (1 pad, 62 botoes) no frame 2626; fps 60 estavel
- 18z9b: r0 logo apos 'Connected XInput Controller #1' -> hot-plug via usbh_pooling_hubs OK (pad conectado apos o IN_Init lazy)
- Pendente: provar efeito no jogo (Start->menu, stick->movimento) -> 18z9c overlay com ultima tecla + origem/angulos do player
- Overlay ainda rotulado 'LG 18z8q' (cosmetico)

## Lote 18z9c (2026-10-28 22:29)
- 18z9c: overlay 'LG 18z9c'; linha 4 = s r k(ultima tecla, CL_KeyEvent em cl_keys.cpp) c(Key_GetCatcher) o(cl.snap.ps.origin) y/p(cl.viewangles)
- 18z9c: lg_dbg2[8] no topo do tr_null.c; preenchido em cl_scrn.cpp; exe=False iso=False (docs/probes/lote18z9c-overlay.log)
- Teste xemu: Start -> k27 + c muda (menu); stick esq -> o muda; stick dir -> y/p mudam

## Lote 18z9c2 (2026-10-28 22:38)
- 18z9c: link falhou ('undefined symbol: int *lg_dbg' = nome mangled C++ x definicao C); xemu rodou XBE 18z9b (overlay 'LG 18z8q')
- 18z9b xemu: Start abriu o MENU do Alice (input chega a UI); CONGELOU ao girar os analogicos (suspeita: eixos/cursor no menu)
- 18z9c2: extern "C" nos 4 .cpp + bloco extern "C" (#ifdef __cplusplus) no tr_null.c; exe=True iso=True
- Regra: global compartilhado C/C++ = extern "C" nos .cpp

## Lote 18z9d-a (2026-10-28 22:47)
- 18z9c2 xemu: overlay 'LG 18z9c' OK (link C resolvido); Start -> k27 -> MENU Alice (fundo em blocos 256 + ui/main/glowA..F = T); k183 navega newgame/loadgame/settings/quit
- 18z9c2: origem fixa 1136 -1712 -439, y0 p0 (sem movimento); stick ESQ fora do menu CONGELA (suspeita: caminho mouse SE_MOUSE/CL_MouseEvent/stubs IN_Mouse*)
- 18z9d-a: coleta lg_input.c + CL_MouseEvent/JoystickEvent + SE_* + stubs IN_ + estado do mouse (docs/probes/lote18z9d-a-stick.log), sem build

## Lote 18z9d-b (2026-10-28 23:01)
- 18z9d-a: SE_MOUSE so com KEYCATCH_UI -> fora do menu stick esq = SE_JOYSTICK_AXIS 0/1 -> CL_JoystickMove (forward/side); hipotese mouse descartada
- 18z9c2 xemu: stick DIR OK (y-116 p-105, sem travar); D-pad cima k132 e A k185 chegam mas origem fixa (setas sem bind de movimento); stick ESQ congela fora do menu, sem efeito no menu
- Suspeita: 1o movimento real -> Pmove/statemap Alice (condicoes falsas/nextstate em laco) OU caminho do eixo
- 18z9d-b: setas -> +forward/+back/+moveleft/+moveright; overlay 'LG 18z9d'; exe=True iso=True
- Teste: D-pad cima fora do menu congela = movimento/servidor; anda = problema so no eixo analogico

## Lote 18z9e-a (2026-10-28 23:07)
- 18z9d-b xemu: overlay 'LG 18z9d'; D-pad cima (+forward, sem eixo) CONGELA igual stick esq; origem nao muda -> trava no 1o movimento real no servidor, input inocente
- Suspeitos: laco msec do Pmove (pmove_msec/fixed renomeados LGC_ 18z6c), nextstate em laco no statemap Alice, PM_SlideMove x BSP FAKK
- 18z9e-a: coleta Pmove/PmoveSingle/SlideMove + pmove_* + Player::EvaluateState/ClientMove + State::Evaluate (docs/probes/lote18z9e-a-move.log), sem build

## Lote 18z9e (2026-10-28 23:22)
- 18z9e-a: laco do Pmove seguro (pml.msec>=1); State::Evaluate faz sent_conditionals->ObjectAt(index) com index 0 (condicao Alice desconhecida, 18z2) -> Container 1-based fora do limite; STAND sem condicao desconhecida, RUN/WALK com -> congela no 1o movimento
- 18z9e-a: EvaluateState count>20 -> assert(0)+gi.Error (invisivel com pbkit = congelamento)
- 18z9e: sob NXDK 'if (index <= 0) continue;' nos lacos clearCheck + laco de estado so break; overlay 'LG 18z9e'; exe=True iso=True

## Lote 18z9f (2026-10-28 23:30)
- 18z9e xemu: overlay 'LG 18z9e'; D-pad cima AINDA congela -> index 0 / assert do laco de estado nao eram (so) a causa
- 18z9f: thread watchdog em lg_input.c (IN_Frame pulsa lg_wd; parado 1,5 s -> pb_show_debug_screen + debugPrint 'LG WD' cm/ev/ps/pl)
- Marcas: cm 1/2 = antes/depois Pmove (ClientMove); ev 1/2 = EvaluateState; ps = PmoveSingle; pl = laco Pmove; exe=True iso=True
- Leitura: cm1 = trava no Pmove; cm2 ev1 = statemap; cm2 ev2 = depois (anim/TIKI/outro); sem texto WD = crash/excecao (CPU parada)

## Lote 18z9g (2026-10-28 23:38)
- 18z9f xemu: tela presa no debug screen apos 'Saving autosave' -> watchdog disparou em hitch legitimo (load/save > 1,5 s) e nunca voltava ao pbkit
- 18z9f: 'omohaaded Error 1' = lg_st so definido em lg_input.c (cliente); game no dedicado ficou sem simbolo; cliente linkou (exe/iso validos)
- 18z9g: lg_st definido em bg_pmove.cpp (selectany, game+cgame); watchdog 3 s; ao voltar pulso -> pb_show_front_screen; exe=True ded=True iso=True
- Nota: ps conta Pmove do game E da predicao do cgame; cm/ev so do game

## Lote 18z9h-a (2026-10-29 07:50)
- 18z9g xemu: overlay OK (s9, 60 fps); WD 939 = hitch do load (zeros, voltou); WD 941 'cm 0 ev 2 ps 0' + 'Saving to centipede10028 (autosave)...' e para
- Conclusao: Pmove NUNCA rodou (cm0/ps0) -> trava no AUTOSAVE, nao no movimento; 1 linha WD so = CPU parada ou deadlock debugPrint (watchdog x main)
- Saves acumulam (10028) no E:; 18z7a salvava OK ('Game Saved')
- 18z9h-a: coleta caminho do save (Saving to/autosave/ArchiveLevel/SaveShot/FS write) em docs/probes/lote18z9h-a-autosave.log, sem build

## Lote 18z9h (2026-10-29 07:57)
- 18z9g xemu (2o boot): botao durante load (s9, k13 ENTER, fps 6) -> trava SEM tela WD -> CPU inteira morta (crash/excecao), nao laco; trava separada do autosave
- 18z9h-a: contexto saiu do gamespy (1o hit); caminho = SV_SaveGame (sv_ccmds 2573) -> SV_ArchiveLevelFile -> ge->WriteLevel -> G_ArchiveLevel (g_main 1367)
- 18z9h: s5 (1 print,2 pos-print,3 ArchiveLevelFile,4 WriteLevel,5 pos-WriteLevel) e s6 (1/2 gi.ArchiveLevel, 3 Game Saved); exe=True ded=True iso=True

## Lote 18z9i (2026-10-29 08:07)
- 18z9h xemu: WD 943 (s5 0 s6 0) = hitch do load, frame 944 rodou; apos 'Saving to centipede10028' NENHUM WD -> CPU inteira morta no save (crash), s5/s6 invisiveis
- 18z9i: marcadores s5/s6 imprimem direto (debugPrint 'LG sX N'); SV_SaveGame/SV_ArchiveLevelFile no log; exe=True ded=True iso=True

## Lote 18z9j (2026-10-29 08:35)
- 18z9i xemu: trava SEM input no LOAD (s9 r1000 k0 fps 4, frame 989), sem tela WD; nao chegou ao autosave
- Hipotese: watchdog (18z9f) troca tela pbkit + debugPrint de OUTRA thread durante hitch >3 s -> corrida com main no pushbuffer -> crash; 18z8q salvava OK ('Game Saved') antes do watchdog
- 18z9j: pb_show_*/debugPrint anulados em lg_input.c (thread e contadores mantidos); exe=True ded=True iso=True
