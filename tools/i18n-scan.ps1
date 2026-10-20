$i18n = [IO.File]::ReadAllText("$PWD\src\i18n.rs")
$skipFiles = 'check|route|_render|_art|^campaign|^i18n|^main|^setup|^materials|^graph|^provenance|^reservations|^regression|^performance|^saves|^android|^controller_map|^particles|^format12'
$skipLine  = 'println!|eprintln!|ensure!|bail!|assert|panic!|anyhow!|#\[|^\s*//|log::|expect\(|context\(|private/|STAGED|PASS '
$lit = '"((?:[^"\\]|\\.)+)"'
$out = Get-ChildItem src -Recurse -Filter *.rs | ? { $_.BaseName -notmatch $skipFiles } | % {
  $f = $_; $n = 0; $tests = $false
  foreach ($line in [IO.File]::ReadLines($f.FullName)) {
    $n++
    if ($line -match '#\[cfg\(test\)\]') { $tests = $true }
    if ($tests -or $line -match $skipLine) { continue }
    foreach ($m in [regex]::Matches($line, $lit)) {
      $t = $m.Groups[1].Value
      if ($t -notmatch '[A-Za-z]{2,}\s+[A-Za-z]') { continue }      # precisa ser frase
      if ($t -notmatch '^[A-Z{]' ) { continue }                     # começa maiúscula ou {tecla}
      if ($t -match '[ãçéêíóõúâ]|\.tik|\.tga|\.wav|gl_|_') { continue }
      $parts = $t -split '\{[^}]*\}' | % { $_.Trim(' /:') } | ? { $_ -match '[A-Za-z]{2,}' }
      if ($parts | ? { -not $i18n.Contains($_) }) {
        [pscustomobject]@{ Arquivo = "$($f.Name):$n"; Texto = $t }
      }
    }
  }
}
$u = $out | Sort-Object Texto -Unique
$u | Format-Table -AutoSize -Wrap | Out-String -Width 300
"Total: $($u.Count)"
