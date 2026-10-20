param([ValidateSet('Setup', 'Worker')][string]$Mode = 'Setup', [string]$JobFile)
# Instalador portátil PT-BR. Windows PowerShell 5.1, sem admin. Tudo fica nesta pasta.
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$priv = Join-Path $root 'private'
$gameDir = Join-Path $root 'Alice'
$exe = Join-Path $root 'looking-glass.exe'
$utf8 = New-Object Text.UTF8Encoding($false)
$archiveName = 'Alice-Remastered-PTBR.7z'
$downloadUrl = 'https://archive.org/download/alice_remastered_ptbr/American%20McGee%27s%20Alice%20Remastered%20HD%20%282011%29%20-%20VorpalFix%284.1.0%29%20-%20%5BPT-BR%5D.7z'
$packs = @('pak0.pk3', 'pak1_large.pk3', 'pak2.pk3', 'pak3.pk3', 'pak5_mod.pk3')
$sizes = @('800 x 600', '1024 x 768', '1200 x 680', '1280 x 720', '1600 x 900', '1920 x 1080', '2560 x 1440', '3840 x 2160')

function Test-Base([string]$d) {
    if (-not (Test-Path -LiteralPath $d -PathType Container)) { return $false }
    foreach ($p in $packs) { if (-not (Test-Path -LiteralPath (Join-Path $d $p))) { return $false } }
    return [bool](Get-ChildItem -LiteralPath $d -Filter 'pak4*.pk3' -File)
}
function Find-Data([string]$folder) {
    if (-not $folder -or -not (Test-Path -LiteralPath $folder -PathType Container)) { return $null }
    if (Test-Base $folder) { return $folder }
    foreach ($f in @(Get-ChildItem -LiteralPath $folder -Recurse -Filter 'pak0.pk3' -File -ErrorAction SilentlyContinue)) {
        if (Test-Base $f.DirectoryName) { return $f.DirectoryName }
    }
    return $null
}
function Read-ConfiguredData {
    $cfg = Join-Path $priv 'data-path.txt'
    if (-not (Test-Path -LiteralPath $cfg)) { return $null }
    $v = [IO.File]::ReadAllText($cfg).Trim()
    if (-not $v) { return $null }
    if (-not [IO.Path]::IsPathRooted($v)) { $v = Join-Path $root $v }
    if (Test-Base $v) { return $v } else { return $null }
}

if ($Mode -eq 'Worker') {
    $job = [IO.File]::ReadAllText($JobFile) | ConvertFrom-Json
    $st = Join-Path $job.dir 'status.json'; $cx = Join-Path $job.dir 'cancel'
    function Report([string]$m, [int]$pc = -1) {
        [IO.File]::WriteAllText($st, (@{ m = $m; p = $pc } | ConvertTo-Json -Compress), $utf8)
        if (Test-Path -LiteralPath $cx) { throw 'Instalação cancelada. Nada foi apagado.' }
    }
    $partial = $null
    try {
        $src = $job.source
        if ($job.download) {
            $dl = Join-Path $priv 'downloads'; [void][IO.Directory]::CreateDirectory($dl)
            $src = Join-Path $dl $archiveName
            if (-not (Test-Path -LiteralPath $src)) {
                $partial = $src + '.partial'
                Report 'Conectando ao Internet Archive...'
                [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
                $rq = [Net.HttpWebRequest]::Create($downloadUrl)
                $rq.UserAgent = 'LookingGlass-Setup/2.0'; $rq.ReadWriteTimeout = 30000
                $rs = $rq.GetResponse(); $total = $rs.ContentLength
                $in = $rs.GetResponseStream(); $out = [IO.File]::Create($partial)
                try {
                    $buf = New-Object byte[] (256 * 1024); $got = 0L
                    $sw = [Diagnostics.Stopwatch]::StartNew()
                    while (($n = $in.Read($buf, 0, $buf.Length)) -gt 0) {
                        $out.Write($buf, 0, $n); $got += $n
                        if ($sw.ElapsedMilliseconds -gt 300) {
                            $pc = if ($total -gt 0) { [int](100 * $got / $total) } else { -1 }
                            Report ('Baixando o jogo: {0:N0} de {1:N0} MB' -f ($got / 1MB), ($total / 1MB)) $pc
                            $sw.Restart()
                        }
                    }
                } finally { $out.Dispose(); $in.Dispose(); $rs.Close() }
                if ($total -gt 0 -and (Get-Item -LiteralPath $partial).Length -ne $total) { throw 'Download incompleto. Tente novamente.' }
                [IO.File]::Move($partial, $src); $partial = $null
            }
        }
        if (Test-Path -LiteralPath $src -PathType Container) {
            Report 'Procurando os arquivos do jogo na pasta...'
            $base = Find-Data $src
        } else {
            $7z = Join-Path $root 'tools\7zr.exe'
            if (-not (Test-Path -LiteralPath $7z)) { throw 'tools\7zr.exe não encontrado. Extraia o ZIP completo.' }
            Report 'Extraindo e verificando os arquivos (pode levar alguns minutos)...'
            [void][IO.Directory]::CreateDirectory($gameDir)
            $si = New-Object Diagnostics.ProcessStartInfo $7z
            $si.Arguments = 'x -y -bso0 -bsp0 "-o' + $gameDir + '" "' + $src + '"'
            $si.UseShellExecute = $false; $si.CreateNoWindow = $true
            $p = [Diagnostics.Process]::Start($si)
            while (-not $p.HasExited) {
                Start-Sleep -Milliseconds 300
                if (Test-Path -LiteralPath $cx) { $p.Kill(); throw 'Instalação cancelada.' }
            }
            if ($p.ExitCode -ne 0) { throw "Falha ao extrair o .7z (código $($p.ExitCode)). O arquivo pode estar corrompido." }
            $base = Find-Data $gameDir
            if ($base -and $job.download -and $job.purge) { Remove-Item -LiteralPath $src -Force }
        }
        if (-not $base) { throw 'Arquivos do jogo (pak0...pak5 .pk3) não encontrados.' }
        $rel = if ($base.StartsWith($root, [StringComparison]::OrdinalIgnoreCase)) { $base.Substring($root.Length).TrimStart('\') } else { $base }
        [void][IO.Directory]::CreateDirectory($priv)
        [IO.File]::WriteAllText((Join-Path $priv 'data-path.txt'), $rel, $utf8)
        Report 'Instalação concluída.' 100
        exit 0
    } catch {
        [IO.File]::WriteAllText($st, (@{ m = $_.Exception.Message; p = 0 } | ConvertTo-Json -Compress), $utf8)
        exit 1
    } finally {
        if ($partial -and (Test-Path -LiteralPath $partial)) { Remove-Item -LiteralPath $partial -Force }
    }
}

Add-Type -AssemblyName System.Windows.Forms, System.Drawing
[Windows.Forms.Application]::EnableVisualStyles()
$mutex = New-Object Threading.Mutex($false, 'Local\LookingGlassSetupPTBR')
if (-not $mutex.WaitOne(0)) { [void][Windows.Forms.MessageBox]::Show('A instalação já está aberta.', 'Looking Glass'); exit 0 }

$f = New-Object Windows.Forms.Form
$f.Text = 'Looking Glass - Instalação PT-BR'
$f.ClientSize = New-Object Drawing.Size(620, 640)
$f.StartPosition = 'CenterScreen'; $f.FormBorderStyle = 'FixedDialog'; $f.MaximizeBox = $false
$f.Font = New-Object Drawing.Font('Segoe UI', 10); $f.AutoScaleMode = 'Dpi'
if (Test-Path -LiteralPath $exe) { $f.Icon = [Drawing.Icon]::ExtractAssociatedIcon($exe) }

function Put($c, $x, $y, $w, $h, $t) { $c.SetBounds($x, $y, $w, $h); if ($t) { $c.Text = $t }; $f.Controls.Add($c); return $c }
function Lbl($t, $x, $y, $w, $h = 24) { Put (New-Object Windows.Forms.Label) $x $y $w $h $t }
function Btn($t, $x, $y, $w) { Put (New-Object Windows.Forms.Button) $x $y $w 34 $t }
function Chk($t, $x, $y, $w) { Put (New-Object Windows.Forms.CheckBox) $x $y $w 26 $t }
function Rad($t, $x, $y, $w) { Put (New-Object Windows.Forms.RadioButton) $x $y $w 26 $t }
function Num($min, $max, $inc, $dec, $v, $x, $y) {
    $n = New-Object Windows.Forms.NumericUpDown
    $n.Minimum = $min; $n.Maximum = $max; $n.Increment = $inc; $n.DecimalPlaces = $dec
    $n.Value = [decimal][Math]::Min($max, [Math]::Max($min, [double]$v))
    Put $n $x $y 80 28 $null
}
$bold = New-Object Drawing.Font('Segoe UI', 10, [Drawing.FontStyle]::Bold)

$h = Lbl 'Bem-vindo ao Looking Glass' 22 14 580 40
$h.Font = New-Object Drawing.Font('Segoe UI', 18, [Drawing.FontStyle]::Bold)
[void](Lbl 'Instalação portátil em português: o jogo, os saves e as configurações ficam nesta pasta.' 24 58 580 40)
$radDl = Rad 'Baixar o jogo em PT-BR (Internet Archive)' 24 102 580
$radLocal = Rad 'Usar um arquivo .7z ou uma pasta que já tenho' 24 130 580
$txt = Put (New-Object Windows.Forms.TextBox) 44 160 552 28 $null
$b7z = Btn 'Escolher .7z...' 44 194 170
$bDir = Btn 'Escolher pasta...' 224 194 170

(Lbl 'Vídeo' 24 244 300).Font = $bold
[void](Lbl 'Resolução:' 24 274 100)
$cbRes = Put (New-Object Windows.Forms.ComboBox) 126 271 170 28 $null
$cbRes.DropDownStyle = 'DropDownList'; [void]$cbRes.Items.AddRange($sizes)
$chkFull = Chk 'Tela cheia (resolução nativa)' 320 272 280

(Lbl 'Controles' 24 316 300).Font = $bold
$chkRun = Chk 'Sempre correr' 24 344 190
$chkSubs = Chk 'Legendas' 220 344 170
$chkInv = Chk 'Inverter eixo Y do controle' 396 344 210
[void](Lbl 'Sensibilidade do mouse:' 24 382 175)
[void](Lbl 'Zona morta do controle:' 316 382 175)

$chkDesk = Chk 'Criar atalho na área de trabalho' 24 428 290
$chkPlay = Chk 'Jogar ao concluir' 320 428 280
$chkPurge = Chk 'Apagar o .7z baixado após instalar' 24 456 290
$prog = Put (New-Object Windows.Forms.ProgressBar) 24 494 572 20 $null
$lblStatus = Lbl 'Escolha uma opção e clique em Instalar.' 24 522 572 56
$go = Btn 'Instalar' 236 590 130
$save = Btn 'Salvar' 376 590 110
$close = Btn 'Fechar' 496 590 100

$prefPath = Join-Path $priv 'preferences.json'
$pr = @{}
if (Test-Path -LiteralPath $prefPath) {
    try { ([IO.File]::ReadAllText($prefPath) | ConvertFrom-Json).PSObject.Properties | ForEach-Object { $pr[$_.Name] = $_.Value } } catch { }
}
function Get-P($k, $d) { if ($pr.ContainsKey($k) -and $null -ne $pr[$k]) { $pr[$k] } else { $d } }
$cbRes.SelectedIndex = [Math]::Min(7, [int](Get-P 'resolution' 3))
$chkFull.Checked = [bool](Get-P 'fullscreen' $true)
$chkRun.Checked = [bool](Get-P 'always_run' $false)
$chkSubs.Checked = [bool](Get-P 'subtitles' $true)
$chkInv.Checked = [bool](Get-P 'invert_pad' $false)
$nSens = Num 0.2 3 0.1 1 (Get-P 'sensitivity' 1) 202 379
$nDz = Num 0.1 0.4 0.01 2 (Get-P 'pad_deadzone' 0.24) 494 379
$chkPlay.Checked = Test-Path -LiteralPath $exe
$chkPurge.Checked = $true

function Save-Prefs {
    $pr['resolution'] = $cbRes.SelectedIndex; $pr['fullscreen'] = $chkFull.Checked
    $pr['always_run'] = $chkRun.Checked; $pr['subtitles'] = $chkSubs.Checked; $pr['invert_pad'] = $chkInv.Checked
    $pr['sensitivity'] = [double]$nSens.Value; $pr['pad_deadzone'] = [double]$nDz.Value
    [void][IO.Directory]::CreateDirectory($priv)
    [IO.File]::WriteAllText($prefPath, ($pr | ConvertTo-Json -Depth 8), $utf8)
}
function Start-Game {
    if (-not (Test-Path -LiteralPath $exe)) { throw 'looking-glass.exe não encontrado nesta pasta.' }
    $si = New-Object Diagnostics.ProcessStartInfo $exe
    $si.WorkingDirectory = $root; $si.UseShellExecute = $false
    [void][Diagnostics.Process]::Start($si)
}
function New-Shortcut {
    try {
        $l = (New-Object -ComObject WScript.Shell).CreateShortcut((Join-Path ([Environment]::GetFolderPath('Desktop')) 'Looking Glass.lnk'))
        $l.TargetPath = $exe; $l.WorkingDirectory = $root; $l.IconLocation = $exe; $l.Save()
    } catch { $lblStatus.Text = 'Jogo pronto, mas o atalho não pôde ser criado.' }
}
function Set-Busy([bool]$b) {
    foreach ($c in @($radDl, $radLocal, $txt, $b7z, $bDir, $cbRes, $chkFull, $chkRun, $chkSubs, $chkInv, $nSens, $nDz, $chkDesk, $chkPlay, $chkPurge, $go, $save)) { $c.Enabled = -not $b }
}
function Reset-Ready { $script:ready = $false; $go.Text = 'Instalar' }

$script:ready = [bool](Read-ConfiguredData)
$script:worker = $null; $script:job = $null
$existing = Join-Path $priv ('downloads\' + $archiveName)
if ($script:ready) {
    $go.Text = 'Jogar'; $radLocal.Checked = $true
    $lblStatus.Text = 'Jogo já instalado. Ajuste as opções e clique em Jogar (ou escolha outra fonte para reinstalar).'
} elseif (Find-Data $gameDir) {
    $txt.Text = $gameDir; $radLocal.Checked = $true
} elseif (Test-Path -LiteralPath $existing) {
    $txt.Text = $existing; $radLocal.Checked = $true
} else { $radDl.Checked = $true }

$radDl.Add_CheckedChanged({ Reset-Ready })
$txt.Add_TextChanged({ Reset-Ready })
$b7z.Add_Click({
    $d = New-Object Windows.Forms.OpenFileDialog; $d.Filter = 'Arquivo 7z (*.7z)|*.7z'; $d.Title = 'Escolha o .7z do jogo'
    if ($d.ShowDialog($f) -eq 'OK') { $txt.Text = $d.FileName; $radLocal.Checked = $true }
    $d.Dispose()
})
$bDir.Add_Click({
    $d = New-Object Windows.Forms.FolderBrowserDialog; $d.Description = 'Escolha a pasta do jogo (com os .pk3)'
    if ($d.ShowDialog($f) -eq 'OK') { $txt.Text = $d.SelectedPath; $radLocal.Checked = $true }
    $d.Dispose()
})
$save.Add_Click({
    try { Save-Prefs; $lblStatus.Text = 'Configurações salvas.' }
    catch { [void][Windows.Forms.MessageBox]::Show($f, $_.Exception.Message, 'Looking Glass') }
})
$go.Add_Click({
    try {
        Save-Prefs
        if ($script:ready) { Start-Game; $f.Close(); return }
        if ($radLocal.Checked -and -not (Test-Path -LiteralPath $txt.Text)) { throw 'Escolha um arquivo .7z ou uma pasta válida.' }
        $script:job = Join-Path $priv ('setup-jobs\' + [Guid]::NewGuid().ToString('N'))
        [void][IO.Directory]::CreateDirectory($script:job)
        $jp = Join-Path $script:job 'job.json'
        [IO.File]::WriteAllText($jp, (@{ dir = $script:job; source = $txt.Text; download = $radDl.Checked; purge = $chkPurge.Checked } | ConvertTo-Json -Compress), $utf8)
        $si = New-Object Diagnostics.ProcessStartInfo (Join-Path $PSHOME 'powershell.exe')
        $si.Arguments = '-NoLogo -NoProfile -ExecutionPolicy Bypass -File "' + $PSCommandPath + '" -Mode Worker -JobFile "' + $jp + '"'
        $si.UseShellExecute = $false; $si.CreateNoWindow = $true
        $script:worker = [Diagnostics.Process]::Start($si)
        $lblStatus.Text = 'Iniciando...'; $prog.Style = 'Marquee'; $close.Text = 'Cancelar'
        Set-Busy $true; $timer.Start()
    } catch { [void][Windows.Forms.MessageBox]::Show($f, $_.Exception.Message, 'Looking Glass') }
})
function Request-Cancel { [IO.File]::WriteAllText((Join-Path $script:job 'cancel'), 'x', $utf8); $lblStatus.Text = 'Cancelando com segurança...'; $close.Enabled = $false }
$close.Add_Click({ if ($script:worker) { Request-Cancel } else { $f.Close() } })
$f.Add_FormClosing({ if ($script:worker) { $_.Cancel = $true; Request-Cancel } })

function Read-Status {
    $sp = Join-Path $script:job 'status.json'
    if (Test-Path -LiteralPath $sp) {
        try {
            $s = [IO.File]::ReadAllText($sp) | ConvertFrom-Json
            $lblStatus.Text = $s.m
            if ($s.p -ge 0) { $prog.Style = 'Continuous'; $prog.Value = [Math]::Min(100, [Math]::Max(0, [int]$s.p)) } else { $prog.Style = 'Marquee' }
        } catch { }
    }
}
$timer = New-Object Windows.Forms.Timer; $timer.Interval = 250
$timer.Add_Tick({
    Read-Status
    if ($script:worker.HasExited) {
        $timer.Stop(); Read-Status
        $ok = $script:worker.ExitCode -eq 0
        $script:worker.Dispose(); $script:worker = $null
        Set-Busy $false; $close.Enabled = $true; $close.Text = 'Fechar'; $prog.Style = 'Continuous'
        if ($ok) {
            $prog.Value = 100; $script:ready = $true; $go.Text = 'Jogar'
            $lblStatus.Text = 'Instalação concluída! Da próxima vez, abra o looking-glass.exe.'
            if ($chkDesk.Checked) { New-Shortcut }
            if ($chkPlay.Checked) { try { Start-Game; $f.Close() } catch { $lblStatus.Text = $_.Exception.Message } }
        } else { $prog.Value = 0 }
    }
})
[void]$f.ShowDialog()
$timer.Dispose(); $f.Dispose(); $mutex.ReleaseMutex(); $mutex.Dispose()
