<p align="center"><img src="docs/icon.png" width="160" alt="Looking Glass PT-BR"></p>

<h1 align="center">Looking Glass — Edição PT-BR</h1>

<p align="center">Port do recomp em Rust de <i>American McGee's Alice</i> para <b>Android</b> e <b>Windows</b>, com tradução em português do Brasil.</p>

<p align="center">
  <a href="https://github.com/Jean-DrEaD/LookingGlass-PTBR/releases/latest"><img src="https://img.shields.io/github/v/release/Jean-DrEaD/LookingGlass-PTBR?include_prereleases&label=release"></a>
  <a href="https://github.com/Jean-DrEaD/LookingGlass-PTBR/actions/workflows/android.yml"><img src="https://img.shields.io/github/actions/workflow/status/Jean-DrEaD/LookingGlass-PTBR/android.yml?label=android"></a>
  <a href="https://github.com/Jean-DrEaD/LookingGlass-PTBR/actions/workflows/windows.yml"><img src="https://img.shields.io/github/actions/workflow/status/Jean-DrEaD/LookingGlass-PTBR/windows.yml?label=windows"></a>
  <img src="https://img.shields.io/badge/Android-8.0+-3DDC84?logo=android&logoColor=white">
  <img src="https://img.shields.io/badge/Windows-10+-0078D6">
  <img src="https://img.shields.io/badge/idioma-pt--BR-009C3B">
  <img src="https://img.shields.io/badge/licen%C3%A7a-MIT-blue">
</p>

> ⚠️ **Os arquivos do jogo não estão incluídos.** Use uma cópia de *American McGee's Alice* que você tenha direito de usar.

## Sumário
- [Download](#download)
- [Instalação](#instalação)
- [Tradução PT-BR](#tradução-pt-br)
- [Compilando](#compilando)
- [Créditos](#créditos)
- [Licença](#licença)

## Download

| Plataforma | Arquivo | Checksum |
|---|---|---|
| Android 8.0+ | [APK](https://github.com/Jean-DrEaD/LookingGlass-PTBR/releases/latest/download/LookingGlass-PTBR-Android.apk) | [sha256](https://github.com/Jean-DrEaD/LookingGlass-PTBR/releases/latest/download/LookingGlass-PTBR-Android.apk.sha256) |
| Windows 10+ | [ZIP](https://github.com/Jean-DrEaD/LookingGlass-PTBR/releases/latest/download/LookingGlass-PTBR-Windows.zip) | [sha256](https://github.com/Jean-DrEaD/LookingGlass-PTBR/releases/latest/download/LookingGlass-PTBR-Windows.zip.sha256) |

[Todas as versões](https://github.com/Jean-DrEaD/LookingGlass-PTBR/releases)

## Instalação

### Windows
1. Extraia o ZIP inteiro numa pasta com permissão de escrita (ex.: `Documentos\LookingGlass`).
2. Rode o `Setup.exe`: ele baixa o jogo em PT-BR ou usa um `.7z`/pasta que você já tenha. Tudo fica na pasta do jogo (portátil).
3. Escolha o idioma. Para português, use o [pacote PT-BR](https://archive.org/details/alice_remastered_ptbr).
4. Jogue pelo `Launch.cmd`.

### Android
1. Instale o APK (permita a instalação de fontes desconhecidas).
2. Abra o app e selecione os seus arquivos `.pk3`.

> Faça backup dos saves antes de atualizar.

Detalhes em [`docs/INSTALL.md`](docs/INSTALL.md) e [`docs/ANDROID.md`](docs/ANDROID.md).

## Tradução PT-BR
O pacote PT-BR substitui o `pak4_english.pk3`, e o `Setup.cmd` o reconhece pela palavra `passo`. Pendências nas [issues](../../issues?q=label%3Atradu%C3%A7%C3%A3o).

## Compilando
```bash
cargo build --release
cargo test
python tools/test_windows_setup.py  # testes do instalador
python tools/package_android.py     # APK (branch android-br)
```
Requisitos: Rust estável, Python 3, Android SDK/NDK (para o APK). Veja [`CONTRIBUTING.md`](CONTRIBUTING.md).

## Créditos
- **[skulitom/LookingGlass](https://github.com/skulitom/LookingGlass)**: projeto original do engine.
- **[Player124413](https://github.com/Player124413)**: port Android.
- **[Wemino](https://github.com/Wemino/VorpalFix)**: autoria das texturas HD.
- **[MJay/MJarcos da Tribo Gamer](https://tribogamer.com)**: tradução e adaptação PT-BR.
- **American McGee's Alice**: © Electronic Arts / Rogue Entertainment. Projeto de fãs, sem afiliação ou relação ou admiração(não se admira empresas ou pessoas ;) ).
- Ícone: arte própria do projeto.

## Licença
Código sob [MIT](LICENSE-MIT). Avisos legais e mídia em [`LEGAL.md`](LEGAL.md).
Os dados do jogo pertencem aos seus detentores e **não** são distribuídos aqui.
