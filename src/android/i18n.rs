//! Launcher translations (English is the source text; Russian is used when the
//! launcher language is Russian) and the Cyrillic-capable font for the launcher.
//!
//! The launcher language comes from `LOOKING_GLASS_ANDROID_LANG`, which the Java
//! side sets to `ru` or `en` (see `MainActivity.isRussianLocale`).
//!
//! The built-in macroquad font only has ASCII glyphs, so Russian text needs a
//! system font. Android ships `Roboto-Regular.ttf` (Cyrillic included) on every
//! release. If none of the candidates can be read, Russian text falls back to
//! the built-in font and may show boxes.

use macroquad::text::{load_ttf_font_from_bytes, Font};
use std::cell::{Cell, RefCell};

/// True when the launcher should show Russian text.
pub fn is_ru() -> bool {
    std::env::var("LOOKING_GLASS_ANDROID_LANG").is_ok_and(|lang| lang == "pt")
}

/// Whole-label translations, matched exactly.
const EXACT: &[(&str, &str)] = &[
    ("Looking Glass - Android Launcher", "Looking Glass - Launcher Android"),
    ("Download game", "Baixar jogo"),
    ("Check Updates", "Atualizações"),
    ("Credits", "Créditos"),
    ("Controller Setup", "Configurar controle"),
    (
        "Tap 'Choose Game Folder' or 'Select PK3 / ZIP' to import your game, then 'START GAME'.",
        "Toque em 'Escolher pasta do jogo' ou 'Selecionar PK3 / ZIP' para importar o jogo e depois em 'INICIAR JOGO'.",
    ),
    (
        "Needs: pak0, pak1_large, pak2, pak3, pak4_english, pak5_mod (.pk3)",
        "Necessário: pak0, pak1_large, pak2, pak3, pak4_english, pak5_mod (.pk3)",
    ),
    ("FPS Limit:", "Limite de FPS:"),
    ("Graphics:", "Gráficos:"),
    ("Controls:", "Controles:"),
    ("Unlimited", "Sem limite"),
    ("Auto", "Auto"),
    ("Quality", "Qualidade"),
    ("Balanced", "Equilibrado"),
    ("Performance", "Desempenho"),
    ("Always On", "Sempre ligado"),
    ("Off", "Desligado"),
    ("Touch: Auto", "Toque: Auto"),
    ("Touch: ON", "Toque: LIGADO"),
    ("Touch: OFF (Pad)", "Toque: DESL. (Controle)"),
    ("Edit Touch HUD", "Editar HUD de toque"),
    ("Choose Game Folder", "Escolher pasta do jogo"),
    ("Select PK3 / ZIP", "Selecionar PK3 / ZIP"),
    ("START GAME", "INICIAR JOGO"),
    (".. Parent", ".. Pasta acima"),
    ("Down >", "Abaixo >"),
    ("Copy From This Folder", "Copiar desta pasta"),
    ("Back", "Voltar"),
    ("Opening archive.org ...", "Iniciando download ..."),
    ("Checking for updates on GitHub...", "Verificando atualizações no GitHub..."),
    (
        "Opening Android system folder chooser...",
        "Abrindo o seletor de pastas do Android...",
    ),
    (
        "Opening Android system file chooser (.pk3 / .zip)...",
        "Abrindo o seletor de arquivos do Android (.pk3 / .zip)...",
    ),
    (
        "Touch controls DISABLED (Gamepad mode active).",
        "Controles de toque DESATIVADOS (modo controle ativo).",
    ),
    ("Starting extraction...", "Iniciando extração..."),
    ("Extracting game data: ", "Extraindo dados do jogo: "),
    ("Damaged archive, tap Download again: ", "Arquivo corrompido, toque em Baixar novamente: "),
    ("Installed ", "Instalado(s) "),
    (" archive(s), missing: ", " arquivo(s), faltando: "),
    ("no .pk3 files inside the archive", "nenhum .pk3 dentro do arquivo"),
];

/// Fragments inside longer messages (applied in order, so longer phrases come first).
const PARTS: &[(&str, &str)] = &[
    (
        "Use Choose Game Folder or Select PK3 / ZIP.",
        "Use 'Escolher pasta do jogo' ou 'Selecionar PK3 / ZIP'.",
    ),
    (
        "Select your game folder or PK3 archive to copy into ",
        "Selecione a pasta do jogo ou o arquivo PK3 para copiar em ",
    ),
    (
        "Game data ready in ",
        "Dados do jogo prontos em ",
    ),
    (
        ". Select FPS / Preset and tap START GAME.",
        ". Escolha FPS / gráficos e toque em INICIAR JOGO.",
    ),
    ("Import finished. ", "Importação concluída. "),
    ("Missing game files: ", "Arquivos do jogo ausentes: "),
    ("Cannot open game files in ", "Não foi possível abrir os arquivos do jogo em "),
    ("Current Folder: ", "Pasta atual: "),
    ("Game files folder: ", "Pasta dos arquivos do jogo: "),
    ("[PK3 FOUND]", "[PK3 ENCONTRADO]"),
    ("[DIR] ", "[PASTA] "),
    ("Copied ", "Copiado(s) "),
    (" archive(s). ", " arquivo(s). "),
    ("Unsupported file: ", "Arquivo não suportado: "),
    ("Invalid PK3 file name", "Nome de arquivo PK3 inválido"),
    ("does not contain any .pk3 files", "não contém arquivos .pk3"),
    (
        "Expected a folder, .pk3 file, or .zip archive",
        "Esperado uma pasta, arquivo .pk3 ou .zip",
    ),
    ("Folder does not exist: ", "Pasta não encontrada: "),
    ("No .pk3 archives found in Download / Documents / LookingGlass folders", "Nenhum .pk3 encontrado nas pastas Download / Documents / LookingGlass"),
    ("No .pk3 archives found in ", "Nenhum .pk3 encontrado em "),
];

/// Translate a launcher label. Returns the English text unchanged when the
/// launcher language is English or when no translation is known.
pub fn localize(en: &str) -> String {
    if !is_ru() {
        return en.to_owned();
    }
    if let Some((_, ru)) = EXACT.iter().find(|(source, _)| *source == en) {
        return (*ru).to_owned();
    }
    let mut out = en.to_owned();
    for (source, ru) in PARTS {
        if out.contains(*source) {
            out = out.replace(*source, *ru);
        }
    }
    out
}

/// Pick the English or Russian version of a formatted message.
pub fn pick(en: String, ru: String) -> String {
    if is_ru() {
        ru
    } else {
        en
    }
}

/// System font paths that have Cyrillic glyphs, most preferred first.
const FONT_CANDIDATES: &[&str] = &[
    "/system/fonts/Roboto-Regular.ttf",
    "/system/fonts/NotoSans-Regular.ttf",
    "/system/fonts/DroidSans.ttf",
];

thread_local! {
    static LAUNCHER_FONT: RefCell<Option<Font>> = const { RefCell::new(None) };
    static FONT_TRIED: Cell<bool> = const { Cell::new(false) };
}

/// Load the Cyrillic system font once. Call from the launcher, after the
/// macroquad context exists.
pub fn ensure_launcher_font() {
    if FONT_TRIED.with(|tried| tried.replace(true)) {
        return;
    }
    for path in FONT_CANDIDATES {
        let Ok(bytes) = std::fs::read(path) else {
            continue;
        };
        if let Ok(font) = load_ttf_font_from_bytes(&bytes) {
            LAUNCHER_FONT.with(|slot| *slot.borrow_mut() = Some(font));
            return;
        }
    }
}

/// Font to use for launcher text: the Cyrillic system font in Russian mode,
/// otherwise `None` (the built-in font).
pub fn launcher_font() -> Option<Font> {
    if !is_ru() {
        return None;
    }
    LAUNCHER_FONT.with(|slot| slot.borrow().clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_and_parts_keys_are_unique_and_non_empty() {
        for (source, ru) in EXACT.iter().chain(PARTS.iter()) {
            assert!(!source.is_empty());
            assert!(!ru.is_empty());
        }
        for (i, (source, _)) in EXACT.iter().enumerate() {
            assert!(
                EXACT.iter().skip(i + 1).all(|(other, _)| other != source),
                "duplicate exact key {source}"
            );
        }
    }
}
