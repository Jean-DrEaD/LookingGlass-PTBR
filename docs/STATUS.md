## Regras fixas
- Sempre enviar tudo em codigos completos prontos para colar no PowerShell 7 (sem passos manuais, sem trechos soltos).
# STATUS — Alice (LookingGlass-PTBR) → Xbox Original

> Anexe este arquivo no início de todo chat. Atualize e faça commit no fim de cada sessão.
> Última atualização: 2026-10-21

## Decisão de arquitetura (FIXA — não rediscutir)
- Alvo: **Xbox original** (CerBios, 128 MB, IP 192.168.1.100, FTP via FileZilla).
- **Caminho 2:** base = fork do `ioQuake3-Xbox` (nxdk + renderer NV2A pbkit/xgu, Docker).
- O código Rust (`LookingGlass-PTBR`) serve como **manual dos formatos** do Übertools/FAKK2
  (BSP FAKK, TIKI, .skb/.ska, shaders, scripts). Não roda no Xbox.
- Descartado: Xbox Series/UWP/GDK, RXDK/D3D8, frontend Rust no_std, XDK 5849.
  (commits `lg-core` no_std no branch `xbox` são legado desse caminho; manter como referência)
- Assets nunca vão no repositório; o usuário fornece os PK3.

## Pastas
| Local | O quê |
|---|---|
| `D:\ioQuake3-Xbox` | clone do Mayo1970/ioQuake3-Xbox (base do port) ❓ remote próprio? (`git remote -v`) |
| `D:\LookingGlass-PTBR` (branch `xbox`) | motor Rust = referência de formatos + estes docs |

## Build e deploy
```powershell
cd D:\ioQuake3-Xbox
# ícone (opcional, se mudar icon.png → redimensionar p/ icon128.png antes)
docker run --rm -v "${PWD}:/src" -w /src xboxdev/nxdk:latest sh -c "python3 tools/png2xbx.py icon128.png icons/oa/TitleImage.xbx"
# build
docker run --rm -v "${PWD}:/src" -w /src xboxdev/nxdk:latest sh -c "make -f Makefile.xbox oa -j8 DEBUG=y"
# saída: build_xbox_oa\default.xbe → FileZilla → E:\Apps\<pasta>\
# log (DEBUG=y): D:\ioquake3_oa.log no Xbox
```

## Validado ✅
- Toolchain nxdk/Docker compila; o triângulo do RXDK rodou (ambiente ok).
- OpenArena roda no console; 128 MB detectados no boot.
- Desempenho ok (tearing → ativar VSync no port).
- Ícone 128×128 embutido no XBE (`$$XTIMAGE`), aparece na UnleashX (XBMC4Gamers: usar `_resources`).
- Title ID do XBE = `0x4F410001`; init flags `0x5 → 0x1`.
- OA com 4 bots: erro de modelo sarge; com 3 bots ok → problema do OA, ignorar.

## Etapa atual
Base OA estável no console. Ainda não existe o sabor `alice` nem código FAKK.

## Próximos passos (em ordem)
1. Criar o sabor `alice` no `Makefile.xbox` (game dir base, nome do xbe, ícone `icons/alice/`, title ID próprio)
2. Escalar o orçamento para 128 MB em `code/sys/xbox_boot.c` (hunk/zone/textura)
3. Loader de BSP FAKK (referência: `bsp` no Rust)
4. Modelos TIKI/skeletal (referência: `skeletal`)
5. Shaders/scripts FAKK2
6. Áudio, cutscenes, VSync

## Problemas em aberto
- (nenhum registrado)

## Regras do fluxo
- 1 sessão = 1 objetivo da lista acima.
- Erro → colar as últimas 30 linhas do log/compilador.
- Fim da sessão → "atualiza o STATUS" → commit `docs: status`.
- Histórico longo em `docs/LOG.md` (só acréscimos no final; não precisa anexar).
- Comandos sempre completos em blocos PowerShell prontos para colar (nada de trechos de arquivo soltos).
- Icone alice: fonte em icons\alice\icon128.png -> icons\alice\TitleImage.xbx
- [ok] Build alice: build_xbox_alice\default.xbe com icone e TitleID 414C0001 (modulos ainda baseq3, provisorio)
- Proximo: blocos STANDALONEALICE (BASEGAME base, sem CD key) em q_shared.h, qcommon.h, common.c, files.c, xbox_launch.c
- [ok] STANDALONEALICE: BASEGAME base, com_standalone=1, sem FS_CheckPak0, sem launcher TA. Icone commitado com git add -f (icons/ esta no .gitignore)
- Pendente: FS_XboxStockVM sem checksums Alice; modulos ainda q3 (cgame/ui/qagame)
- [ok] Removido LEGACY_PROTOCOL do bloco STANDALONEALICE
- Proximo: teste no xemu/console com pasta base\ ao lado do default.xbe
- [teste] alice no ioq3: pk3 montam e default.cfg carrega, mas UI q3 sem assets (tela azul). Engine do Alice e Ritual/FAKK2 (TIKI, BSP FAKK, .urc), incompativel com ioq3. Branch alice congelado.
- Proximo: decidir entre portar o engine Rust para nxdk ou avaliar um engine C/C++ da linha FAKK2 (ex. OpenMoHAA)
- [decisao] Toolchain Xbox: RXDK (Team-Resurgent, GPLv3, D3D8/DirectSound/MSVC). nxdk descartado para este port.
- [ok] lg-core compila no_std (x86_64-unknown-none, --no-default-features).
- [medicao] app usa macroquad 329x, std::fs 288x, image 21x, rodio 16x, zip 6x, std::thread 5x -> vira frontend C++ (D3D8 + DirectSound + XAPI arquivos).
- Proximo: crate lg-xbox (staticlib no_std, i686, target-cpu=pentium3) + FFI C para o frontend RXDK.
- [wip] crate lg-xbox (staticlib no_std, i686, pentium3) criado.
- [ok] lg_xbox.lib gerada (i686-pc-windows-msvc, pentium3, ~3,2 MB antes do link).
- [medicao] lg-core 172 KB vs app 7.492 KB (~2% portavel hoje). Mapa em docs/xbox-port-map.csv.
- [risco] -sse2 no i686 gera warning que vira hard error futuro (rust#116344): fixar toolchain do build Xbox.
- [medicao] PURO 190 arq/1.821 KB, MISTO 195/3.982 KB, PLATAFORMA 78/1.689 KB.
- [auditoria] MISTO facil: 94 arq | MISTO com API: 101 arq | PURO contaminado: 76 arq.
- Plano: lg-core exporta Vec2/Color/Rect proprios (glam + struct Color) para substituir macroquad::prelude nos MISTO faceis.
- [fix] KB dos CSVs auditados estavam 10x (parse pt-BR). Real: MISTO facil ~1.297 KB, MISTO API ~2.361 KB.
- [nota] regex de API pegou funcoes proprias (draw_pose, load_levels...). API real: Camera3D, set_camera, clear_background, Texture2D, is_key_*, screen_*, draw_text/line/rectangle, load_texture.
- [ok] lg_core::math criado (re-export glam). Lote 1: 24 arquivos migrados de macroquad::prelude.
- [ok] math ampliado (Quat/Vec4/Mat3/EulerRot). Lote 1b: 23 arquivos. [nota] pais com filhos 'use super::*' precisam manter macroquad ate migrar os filhos.
