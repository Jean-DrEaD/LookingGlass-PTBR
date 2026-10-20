<p align="center">
<img src="https://raw.githubusercontent.com/Jean-DrEaD/Looking-Glass-android-edition/assets/icon.png" alt="Looking Glass" width="256">
</p>

<h1 align="center">Looking Glass — Edição PT-BR</h1>

<p align="center">
Port do recomp em Rust de <em>American McGee's Alice</em> para <strong>Android</strong> e <strong>Windows</strong>, com tradução em português do Brasil.
</p>
<p align="center">
  <a href="https://github.com/Jean-DrEaD/Looking-Glass-android-edition/releases/latest"><img src="https://img.shields.io/github/v/release/Jean-DrEaD/Looking-Glass-android-edition?label=release&color=b3121f" alt="Release"></a>
  <a href="https://github.com/Jean-DrEaD/Looking-Glass-android-edition/actions"><img src="https://img.shields.io/github/actions/workflow/status/Jean-DrEaD/Looking-Glass-android-edition/android.yml?branch=android-br&label=build" alt="Build"></a>
  <img src="https://img.shields.io/badge/Android-8.0%2B-3DDC84?logo=android&logoColor=white" alt="Android">
  <img src="https://img.shields.io/badge/Windows-10%2B-0078D6?logo=windows&logoColor=white" alt="Windows">
  <img src="https://img.shields.io/badge/idioma-pt--BR-009c3b" alt="pt-BR">
  <a href="LICENSE-MIT"><img src="https://img.shields.io/badge/licen%C3%A7a-MIT-blue" alt="MIT"></a>
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
| Plataforma | Arquivo | Branch |
|---|---|---|
| Android | [APK (última versão)](https://github.com/Jean-DrEaD/Looking-Glass-android-edition/releases/latest) | [`android-br`](../../tree/android-br) |
| Windows | [ZIP (última versão)](https://github.com/Jean-DrEaD/Looking-Glass-android-edition/releases/latest) | [`windows-br`](../../tree/windows-br) |

Confira os arquivos com o [SHA256SUMS.txt](https://github.com/Jean-DrEaD/Looking-Glass-android-edition/releases/latest/download/SHA256SUMS.txt). Leia as [notas da versão](https://github.com/Jean-DrEaD/Looking-Glass-android-edition/releases/latest) antes de instalar.

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