> **ALVO ATUAL: Xbox original (2001).** Pentium III 733 MHz, 64 MB RAM, NV2A (DX8), toolchain RXDK/nxdk, target Rust `i686-pc-windows-msvc`, `lg-core` em `no_std`, `lg-xbox` como staticlib. As secoes sobre UWP/Series abaixo sao historicas.
> Fluxo obrigatorio: `tools\status-check.ps1` antes e depois de cada lote; o lote abre com `[..]` e fecha com `[ok] <hash>`; relatorio em `docs\AUDIT.md`.
# LookingGlass-PTBR → Xbox: plano de ação

> Documento de handoff. Cole no início de um novo chat para continuar daqui.
> Repositório: `D:\LookingGlass-PTBR` · Branch de trabalho: `xbox` (criada a partir da `main`)

---

## 0. Visão geral

**Situação atual:** jogo em Rust feito com `macroquad`/`miniquad` (OpenGL), com I/O direto (`std::fs`, `std::thread`) e assets extraídos de zip por `assets.rs`.

**Objetivo:** rodar no Xbox Series/One.

**Estratégia:** dividir o projeto em um workspace com três crates:

```
LookingGlass-PTBR/
├─ Cargo.toml            # [workspace]
├─ lg-core/              # lógica pura: sem render, sem I/O, sem relógio
├─ lg-desktop/           # app atual (macroquad): render, input, áudio, fs
└─ lg-xbox/              # novo frontend: D3D, gamepad, XAudio2, armazenamento UWP/GDK
```

O `lg-core` expõe uma API do tipo `Game::step(input, dt) -> Frame`, em que `Frame` é uma lista de comandos de desenho e som. Cada frontend recebe esses comandos e os executa.

---

## 1. Caminhos de publicação (escolher um)

| Caminho | Requisitos | Prós | Contras |
|---|---|---|---|
| **A. Xbox Dev Mode (UWP)** | Conta Partner Center (taxa única, cerca de US$ 19) e console em Dev Mode | Barato e rápido para testar | UWP/D3D11, limites de memória, só para sideload, sem loja |
| **B. Microsoft GDK (ID@Xbox)** | Aprovação no ID@Xbox e devkit | Acesso total ao console, D3D12 e loja | Burocracia, NDA, toolchain fechado |

**Recomendação:** desenvolver pelo caminho **A** e manter o frontend isolado para dar para migrar para o **B** depois.

**Rust no Xbox:**
- Target UWP: `x86_64-uwp-windows-msvc`. É tier 3, então exige nightly e `-Z build-std`.
- GDK: target `x86_64-pc-windows-msvc` linkando as libs do GDK.

**Render:** o `miniquad` não roda em UWP. As opções são:
1. Escrever um renderer **D3D11** próprio com o crate `windows`. É o caminho mais controlável.
2. Usar `wgpu` com backend DX12. O suporte a UWP precisa ser validado.
3. Usar **ANGLE** (GLES sobre D3D11), aproveitando os shaders atuais. Exige adaptar o miniquad.

> ⚠️ Os assets do jogo original são protegidos por direitos autorais. O port deve continuar exigindo que o usuário forneça os próprios arquivos, como o `assets.rs` já faz. Nunca empacotar os assets.

---

## 2. Regras do `lg-core`

- **Proibido:** `macroquad`/`miniquad`, `std::fs`, `std::io`, `std::thread`, `std::env`, `std::process`, `Instant`, `get_time`, `rand` do macroquad.
- **Matemática:** `glam` direto, na **mesma versão** do macroquad (ver com `cargo tree -i glam`).
- **Tempo:** sempre recebido como parâmetro (`dt`, `FIXED_DT`). Nunca lido do relógio.
- **Aleatoriedade:** RNG próprio e determinístico, com seed salva no save (já existe `dice`).
- **Dados:** o core recebe `&[u8]` ou estruturas já parseadas. Nunca recebe caminhos de arquivo.
- **Ferramentas de dev** (`registry_check` etc.) ficam no desktop, atrás de `#[cfg(not(feature = "xbox"))]`.
- **Meta futura (opcional):** `no_std` + `alloc`.

---

## 3. Diagnóstico feito até agora

### Lições sobre os scripts
- Imports em várias linhas escondiam `fs`. Agora a análise usa `Get-Content -Raw`.
- Imports agrupados `crate::{a, b}` exigem regex própria (já corrigida).
- Submódulos usam `super::*`, que a regex não pega. `Deps` vazio **não** quer dizer independente.
- Nomes em maiúscula em `Deps` (`Kind`, `Skeleton`…) são ruído de imports aninhados.

### Módulos centrais (dependências de cerca de 35 arquivos)

| Módulo | Encontrado | Ação | Status |
|---|---|---|---|
| `collision` | só `vec3`, `Quat`, `Vec3` | trocar o import por `glam` | ✅ pronto para migrar |
| `bsp` | só `vec2`, `vec3`, `Vec2`, `Vec3`; deps `visibility`, `powerups` | migrar junto com essas duas deps | ✅ pronto (verificar deps) |
| `skeletal` | `prelude::*` | trocar por imports explícitos e compilar | 🔍 |
| `interaction` | `prelude::*` | idem | 🔍 |
| `fortress` | `prelude::*` | idem | 🔍 |
| `combat` | `prelude::*` | idem | 🔍 |
| `dice` | render perto da linha 1410 (`set_camera`, `draw_effects`, `draw_text`) | separar em `dice_render.rs` no desktop | 🔧 |
| `ant` | `draw_shot` e `draw_blasts` (linha 581 em diante) | separar em `ant_render.rs` | 🔧 |
| `cinematic` | `draw_rectangle` na linha 219 (HUD) | trocar por emissão de `DrawCmd` | 🔧 |
| `audio` | `fs` de configurações, `thread::sleep` | `reaction` e `events` vão para o core, backend e config ficam no frontend | 🔧 |
| `assets` | I/O puro (zip, cópia, `read_dir`) | **fica no frontend** | ⛔ |
| `campaign_route` | executor de rotas | fica no desktop | ⛔ |
| `registry_check` | ferramenta de dev | desktop, `cfg(not(xbox))` | ⛔ |
| `animation_events` | depende de `assets`, `bsp`, `interaction` | trocar `&Assets` por dados | 🔧 |

### Folhas candidatas
Cerca de 150 arquivos passaram no filtro de IO e gráficos: `levels\*`, `npc\*`, `pool\*`, `*\tests.rs` etc. Eles migram **depois** dos módulos centrais. Os que dependem só de `collision`, `cinematic`, `audio` ou `dice` vêm primeiro.

---

## 4. Roteiro por fases

### Fase 1: workspace e core mínimo
- [ ] Criar a branch `xbox` (`git checkout -b xbox && git push -u origin xbox`).
- [ ] Converter o projeto em workspace e mover `src/` para `lg-desktop/`.
- [ ] Criar `lg-core` com `glam = "=X.Y.Z"` e `anyhow = { version = "1", default-features = false }`.
- [ ] Migrar `collision`, `visibility`, `powerups` e `bsp`, com reexport no desktop (`pub use lg_core::collision;`).
- [ ] `cargo build` e `cargo test` verdes nas duas pontas.

### Fase 2: limpar os módulos centrais
- [ ] Trocar `prelude::*` por imports explícitos em `skeletal`, `interaction`, `fortress` e `combat`.
- [ ] Separar o render de `dice`, `ant` e `cinematic`.
- [ ] Separar `audio` em lógica (core) e backend.
- [ ] Trocar `rand`/`get_time` por RNG e `dt` injetados.
- [ ] Rodar o diagnóstico de novo (seção 6).

### Fase 3: folhas e desacoplar `Assets`
- [ ] Migrar `levels\*`, `npc\*`, `pool\*`, `story`, `save` e `campaign`.
- [ ] Definir as traits do core:
  ```rust
  pub trait Storage { fn read(&self, key: &str) -> Option<Vec<u8>>; fn write(&mut self, key: &str, data: &[u8]); }
  pub trait AssetSource { fn get(&self, name: &str) -> Option<&[u8]>; }
  ```
- [ ] Trocar `&Assets` por `&dyn AssetSource` nos módulos de lógica.

### Fase 4: contrato entre core e frontend
- [ ] `Input`: estado de botões e eixos, independente do dispositivo (teclado ou gamepad).
- [ ] `Frame { draws: Vec<DrawCmd>, sounds: Vec<SoundCmd>, camera: Camera }`.
- [ ] Reescrever o desktop para consumir `Frame`, com o mesmo visual de antes (teste de regressão visual por screenshot).
- [ ] Loop com passo fixo (`FIXED_DT`) no frontend.

### Fase 5: frontend Xbox
- [ ] Toolchain: `rustup toolchain install nightly`, `rust-src`, target `x86_64-uwp-windows-msvc` com `-Z build-std`.
- [ ] Janela e loop via `CoreWindow` (crate `windows`).
- [ ] Renderer D3D11: meshes BSP, modelos esqueléticos, texturas, HUD e texto. Portar os shaders GLSL para HLSL.
- [ ] Input: `Windows.Gaming.Input.Gamepad`, com mapeamento e zona morta.
- [ ] Áudio: XAudio2.
- [ ] Storage: `ApplicationData::Current().LocalFolder()` para saves e configurações.
- [ ] Assets: o usuário copia o zip para a pasta do app (via Dev Home ou portal web do Xbox) e o `lg-xbox` extrai na primeira execução.
- [ ] Empacotar MSIX/AppX e instalar pelo Device Portal.

### Fase 6: ajustes para console
- [ ] UI navegável 100% pelo controle (menus, console de dev desativado).
- [ ] Resolução e escala: 1080p/4K, safe area de TV.
- [ ] Orçamento de memória do Dev Mode e streaming de níveis.
- [ ] Suspend/resume: salvar o estado ao suspender.
- [ ] Desempenho: 60 fps estáveis.

---

## 5. Pendências em aberto
- [ ] Versão do `glam` do macroquad.
- [ ] Deps de `visibility` e `powerups` (precisam estar limpas para o `bsp` migrar).
- [ ] Se `rand` do macroquad é usado na lógica.
- [ ] `no_std` ou só "sem I/O".
- [ ] Renderer: D3D11 próprio, `wgpu` ou ANGLE.
- [ ] Caminho A (Dev Mode) ou B (GDK).

---

## 6. Scripts de diagnóstico

### Módulos sem IO e sem gráficos
```powershell
Get-ChildItem src -Recurse -Filter *.rs | % {
  $t = Get-Content $_.FullName -Raw
  $deps = [regex]::Matches($t, 'crate::\{([^}]*(?:\{[^}]*\}[^}]*)*)\}|(?<![\w:])crate::(\w+)') | % {
    if ($_.Groups[1].Success) { [regex]::Matches($_.Groups[1].Value, '(?m)(?:^|,)\s*(\w+)') | % { $_.Groups[1].Value } }
    else { $_.Groups[2].Value }
  }
  $mq  = [regex]::Matches($t, 'macroquad::[\w:]+|\b(draw_\w+|load_texture|Texture2D|Camera3D|next_frame|is_key_\w+|screen_\w+|get_time)\b') | % { $_.Value } | sort -Unique
  $gfx = $mq | ? { $_ -notmatch '^macroquad::(math|prelude)$' }
  $io  = $t -match '\bfs::|\bfs,|std::io|std::thread|Instant|std::env|std::process'
  [pscustomobject]@{
    Mod  = $_.FullName.Substring((Resolve-Path src).Path.Length + 1)
    IO   = $io; Gfx = ($gfx -join ','); Deps = (($deps | sort -Unique) -join ',')
  }
} | ? { -not $_.IO -and -not $_.Gfx } | sort { ($_.Deps -split ',').Count } | ft -AutoSize -Wrap
```

### Por que um módulo foi reprovado
```powershell
"visibility","powerups","skeletal","interaction" | % {
  $f = @("src/$_.rs","src/$_/mod.rs") | ? { Test-Path $_ } | select -First 1
  "== $_ ($f)"
  Select-String $f -Pattern 'macroquad::[\w:]+|\b(draw_\w+|load_texture|Texture2D|Camera3D|get_time|rand)\b|\bfs::|\bfs,|std::io|std::thread|Instant|std::env' |
    % { "  $($_.LineNumber): $($_.Line.Trim())" } | select -First 8
}
```

---

## 7. Próxima ação imediata
1. `cargo tree -i glam`, para descobrir a versão.
2. Rodar o script 6.2 para `visibility` e `powerups`.
3. Criar o workspace e migrar `collision` e `bsp` (Fase 1).

