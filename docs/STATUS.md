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
| 13 | .. | | decisao C + prova OpenMoHAA no nxdk |

## Problemas em aberto
- [risco] Formatos MOHAA (.skd/.skc, TIKI v2) != Alice (.skb/.ska, BSP FAKK): diferencas a mapear usando o Rust como referencia.
- `_aligned_malloc` do lg-xbox: existe no RXDK/MSVC; verificar no nxdk se for o caminho A ou C.
- `-sse2` no i686 (rust#116344): fixar a toolchain do build Xbox.
- Dependencias comportamentais do macroquad: render.rs:303, viewer.rs:1088, look.rs:21/67, character.rs:1406.

