"""Checagens estáticas do instalador portátil PT-BR."""
import pathlib, unittest
ROOT = pathlib.Path(__file__).resolve().parents[1]
RAW = (ROOT / "tools/windows_setup.ps1").read_bytes()
S = RAW.decode("utf-8-sig")

class WindowsSetupTests(unittest.TestCase):
    def test_bom_para_powershell_51(self):
        self.assertTrue(RAW.startswith(b"\xef\xbb\xbf"))
    def test_download_ptbr(self):
        self.assertIn("archive.org/download/alice_remastered_ptbr/", S)
    def test_portatil(self):
        self.assertIn("Join-Path $root 'private'", S)
        for bad in ("USERPROFILE", "APPDATA", "LOCALAPPDATA"):
            self.assertNotIn(bad, S)
    def test_interface_em_portugues(self):
        for en in ("'Welcome", "'Choose archive", "'Set up'", "'Close'", "'Language"):
            self.assertNotIn(en, S)
    def test_limites_iguais_ao_jogo(self):
        self.assertIn("Num 0.2 3 ", S)
        self.assertIn("Num 0.1 0.4 ", S)

if __name__ == "__main__":
    unittest.main()
