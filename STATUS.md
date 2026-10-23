<!-- FLUXO-INICIO -->
## >>> FLUXO ATUAL (manter no topo) <<<
- 16z4: MARCO - centipede1 (BSP FAKK v42 + 408 ents) RODANDO em SS_GAME a 20 fps estavel no xemu
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
