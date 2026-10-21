# STATUS — Alice (LookingGlass-PTBR) → Xbox Original

> Anexe este arquivo no início de todo chat. Atualize e faça commit no fim de cada sessão.
> Última atualização: 2026-10-21

## Decisão de arquitetura (FIXA — não rediscutir)
- Alvo: **Xbox original** (CerBios, 128 MB, IP 192.168.1.100, FTP via FileZilla).
- **Caminho 2:** base = fork do `ioQuake3-Xbox` (nxdk + renderer NV2A pbkit/xgu, Docker).
- O código Rust (`LookingGlass-PTBR`) serve como **manual dos formatos** do Übertools/FAKK2
  (BSP FAKK, TIKI, .skb/.ska, shaders, scripts). Não roda no Xbox.
- Descartado: Xbox Series/UWP/GDK, RXDK/D3D8, frontend Rust no_std, XDK 5849.
- Assets nunca vão no repositório; o usuário fornece os PK3.

## Pastas
| Local | O quê |
|---|---|
| `D:\ioQuake3-Xbox` | fork/clone do Mayo1970/ioQuake3-Xbox (base do port) ❓ repo próprio? |
| `D:\LookingGlass-PTBR` (branch `xbox`) | motor Rust = referência de formatos + estes docs |

## Build e deploy
```powershell
cd D:\ioQuake3-Xbox
docker run --rm -v "${PWD}:/src" -w /src xboxdev/nxdk:latest sh -c "make -f Makefile.xbox oa -j8 DEBUG=y"
# saída: build_xbox_oa\default.xbe → FileZilla → E:\Apps\<pasta>\
# log (DEBUG=y): D:\ioquake3_oa.log no Xbox
