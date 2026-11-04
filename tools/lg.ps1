#requires -Version 7
<#
  lg.ps1 - PAINEL do projeto Alice -> Xbox. Menu em portugues; nao precisa saber programar.
  Salvar em D:\LookingGlass-PTBR\tools\lg.ps1   (junto com lg-build.ps1 e lg-xemu-test.ps1)
  Para abrir:   cd D:\LookingGlass-PTBR ; .\tools\lg.ps1
#>
$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $false

$lgr    = 'D:\LookingGlass-PTBR'
$omo    = 'D:\openmohaa'
$tools  = "$lgr\tools"
$isoCli = 'G:\xemu\alice-cliente.iso'
$xbeCli = "$omo\xbe-out\cli.xbe"
$mapa   = "$omo\xbe-out\cli.map"

function Dizer([string]$texto, [string]$cor = 'Gray') { Write-Host $texto -ForegroundColor $cor }

function Perguntar-Numero([string]$texto, [int]$padrao) {
  $r = Read-Host "$texto (Enter = $padrao)"
  $tmp = 0
  if ([int]::TryParse($r, [ref]$tmp) -and $tmp -gt 0) { return $tmp }
  return $padrao
}

function CommitCurto([string]$repo) { return (git -C $repo rev-parse --short HEAD).Trim() }

# Anota um bloco no fim do STATUS.md (mesmo formato dos lotes antigos)
function Registrar([string]$titulo, [string[]]$linhas) {
  $arq = "$lgr\STATUS.md"
  $raw = [IO.File]::ReadAllText($arq).TrimEnd()
  $corpo = ($linhas | ForEach-Object { "- $_" }) -join "`n"
  $bloco = "`n`n## $titulo ($(Get-Date -Format 'yyyy-MM-dd HH:mm'))`n$corpo"
  [IO.File]::WriteAllText($arq, $raw + $bloco + "`n", (New-Object Text.UTF8Encoding $false))
}

# Lista (curta) do que foi alterado nos dois repositorios
function Mudancas {
  $lista = @()
  foreach ($r in @(@('port', $lgr), @('engine', $omo))) {
    $s = @(git -C $r[1] status --short 2>$null) | Where-Object { $_ -and ($_ -notmatch 'STATUS\.md') }
    foreach ($x in $s) { $lista += "[$($r[0])] $($x.Trim())" }
  }
  return $lista
}

# Commit + push de UM repositorio, com travas de seguranca
function Salvar-Repo([string]$nome, [string]$caminho, [string[]]$adicionar, [string]$mensagem) {
  $ramo = (git -C $caminho rev-parse --abbrev-ref HEAD).Trim()
  if ($ramo -ne 'xbox') {
    Dizer "[$nome] ATENCAO: voce esta no ramo '$ramo' e nao em 'xbox'. Nao salvei nada deste repositorio." 'Red'
    return
  }
  git -C $caminho add -u 2>&1 | Out-Null
  foreach ($a in $adicionar) {
    if (Test-Path "$caminho\$a") { git -C $caminho add $a 2>&1 | Out-Null }
  }
  $staged = @(git -C $caminho diff --cached --name-only)
  if ($staged.Count -eq 0) { Dizer "[$nome] nada novo para salvar" 'DarkGray'; return }
  $proibidos = @($staged | Where-Object { $_ -match '\.(pk3|iso|xbe|exe|zip|7z|rar)$' })
  $grandes   = @($staged | Where-Object { (Test-Path "$caminho\$_") -and ((Get-Item "$caminho\$_").Length -gt 20MB) })
  if ($proibidos.Count -or $grandes.Count) {
    git -C $caminho reset -q
    Dizer "[$nome] PAREI: ia salvar arquivos pesados/proibidos (jogo, ISO, exe). Nada foi salvo:" 'Red'
    ($proibidos + $grandes) | Select-Object -Unique | ForEach-Object { Dizer "    $_" 'Red' }
    return
  }
  git -C $caminho commit -m $mensagem -q 2>&1 | Out-Null
  $saida = git -C $caminho push origin xbox 2>&1 | Select-Object -Last 1
  Dizer "[$nome] salvo ($($staged.Count) arquivos) -> $saida" 'Green'
}

function Salvar([string]$mensagem) {
  Salvar-Repo 'port'   $lgr @('tools', 'docs') $mensagem
  Salvar-Repo 'engine' $omo @('code') $mensagem
}

function Compilar {
  Dizer 'Compilando... (a 1a vez demora ~1-2 min; as proximas so recompilam o que mudou)' 'Cyan'
  & "$tools\lg-build.ps1"
}

function Testar([int]$n) {
  if (-not (Test-Path $isoCli)) { throw "A ISO $isoCli ainda nao existe. Rode a opcao 1 primeiro." }
  Dizer "Testando $n vezes no xemu. NAO mexa no mouse/teclado ate acabar." 'Cyan'
  return (& "$tools\lg-xemu-test.ps1" -Iso $isoCli -N $n | Select-Object -Last 1)
}

function Interpretar($r) {
  if ($r.Travou -eq 0) { Dizer "Nenhuma trava em $($r.Total) execucoes. Pode ter sido corrigido (ou e raro: repita com mais execucoes)." 'Green' }
  elseif ($r.Travou -eq $r.Total) { Dizer 'Travou SEMPRE: o problema e reproduzivel. Use a opcao 5 para descobrir onde.' 'Red' }
  else { Dizer "Travou $($r.Travou) de $($r.Total): trava INTERMITENTE. Use a opcao 5 numa execucao que travar." 'Yellow' }
}

# Descobre em qual funcao a CPU estava parada, a partir do EIP lido no xemu
function Onde-Travou {
  if (-not (Test-Path $mapa)) { Dizer 'O arquivo cli.map nao existe. Rode a opcao 1 (compilar) e trave o jogo de novo.' 'Red'; return }
  Dizer ''
  Dizer 'Passo a passo (com o jogo TRAVADO no xemu):' 'Cyan'
  Dizer '  1. No menu do xemu clique em  Debug > Monitor'
  Dizer '  2. Na janela que abrir, digite:   info registers   e aperte Enter'
  Dizer '  3. Procure a linha que comeca com  EIP=  e copie o numero (ex.: 0021a4f0)'
  $txt = (Read-Host 'Cole aqui o valor do EIP').Trim()
  if (-not $txt) { return }
  $eip = [Convert]::ToUInt64(($txt -replace '^0[xX]', ''), 16)
  $rx = [regex]'^\s*[0-9a-fA-F]{4}:[0-9a-fA-F]{8}\s+(\S+)\s+([0-9a-fA-F]{8})\s+(?:[fi]\s+)?(\S+)'
  $melhor = $null; $dist = [uint64]::MaxValue; $lidas = 0
  foreach ($l in [IO.File]::ReadLines($mapa)) {
    $m = $rx.Match($l)
    if (-not $m.Success) { continue }
    $lidas++
    $va = [Convert]::ToUInt64($m.Groups[2].Value, 16)
    if ($va -le $eip -and ($eip - $va) -lt $dist) { $dist = $eip - $va; $melhor = $m }
  }
  if ($lidas -eq 0) { Dizer 'Nao consegui ler o cli.map (formato diferente do esperado). Me mande as 30 primeiras linhas dele.' 'Red'; return }
  if (-not $melhor -or $dist -gt 0x20000) {
    Dizer "Esse EIP (0x$($eip.ToString('x'))) nao cai em nenhuma funcao do jogo. Pode ser o kernel do Xbox ou um valor errado." 'Yellow'
    Registrar 'Auto (trava)' @("EIP=0x$($eip.ToString('x')) fora das funcoes do jogo")
    return
  }
  $nome = $melhor.Groups[1].Value; $obj = $melhor.Groups[3].Value
  Dizer ''
  Dizer "A CPU estava DENTRO de:  $nome" 'Green'
  Dizer "Arquivo de origem:       $obj   (+$dist bytes dentro da funcao)" 'Green'
  Dizer '(nomes com ? e @@ sao normais em C++; me mande esta tela que eu interpreto)' 'DarkGray'
  Registrar 'Auto (trava)' @("EIP=0x$($eip.ToString('x')) -> $nome em $obj (+$dist)")
}

function Resumo-Final($r) {
  Dizer ''
  Dizer '===== RESUMO =====' 'Cyan'
  Dizer "Build base: commit $(CommitCurto $omo)  |  XBE: $((Get-Item $xbeCli).Length) bytes"
  if ($r) { Dizer "Teste: $($r.Travou) de $($r.Total) travaram" }
  Dizer 'Anotado no STATUS.md e salvo no GitHub (se nao apareceu erro acima).'
}

# ------------------------------------------------------------------ MENU
while ($true) {
  Dizer ''
  Dizer '=========== ALICE -> XBOX : PAINEL ===========' 'Cyan'
  Dizer ' 1  Compilar e criar a ISO'
  Dizer ' 2  Testar a ISO no xemu sozinho (abre/fecha e diz se travou)'
  Dizer ' 3  FAZER TUDO: compilar + testar + anotar no STATUS + salvar no GitHub'
  Dizer ' 4  Salvar progresso agora (anota no STATUS + commit + push)'
  Dizer ' 5  Descobrir ONDE o jogo travou'
  Dizer ' 6  Ver o final do STATUS'
  Dizer ' 0  Sair'
  $op = Read-Host 'Escolha um numero'
  try {
    switch ($op) {
      '1' { Compilar }
      '2' {
        $n = Perguntar-Numero 'Quantas vezes testar' 5
        $r = Testar $n
        Interpretar $r
      }
      '3' {
        $msg = Read-Host 'Em uma frase: o que voce mudou? (Enter = automatico)'
        if (-not $msg) { $msg = 'ajustes (auto)' }
        $mud = @(Mudancas)
        Compilar
        $n = Perguntar-Numero 'Quantas vezes testar' 5
        $r = Testar $n
        Interpretar $r
        $linhas = @(
          "mudanca: $msg",
          "build cliente OK sobre o commit $(CommitCurto $omo) (engine) + alteracoes abaixo; XBE $((Get-Item $xbeCli).Length) bytes",
          "teste no xemu: $($r.Travou) de $($r.Total) execucoes TRAVARAM ($($r.Detalhe))"
        )
        if ($mud.Count -gt 0) { $linhas += 'arquivos alterados: ' + (($mud | Select-Object -First 12) -join '; ') }
        Registrar 'Auto' $linhas
        Salvar "auto: $msg | teste: $($r.Travou)/$($r.Total) travaram"
        Resumo-Final $r
      }
      '4' {
        $msg = Read-Host 'Em uma frase: o que esta salvando? (Enter = automatico)'
        if (-not $msg) { $msg = 'progresso (auto)' }
        $mud = @(Mudancas)
        $linhas = @("progresso salvo: $msg")
        if ($mud.Count -gt 0) { $linhas += 'arquivos alterados: ' + (($mud | Select-Object -First 12) -join '; ') }
        Registrar 'Auto' $linhas
        Salvar "auto: $msg"
        Resumo-Final $null
      }
      '5' { Onde-Travou }
      '6' { Get-Content "$lgr\STATUS.md" -Tail 30 | ForEach-Object { Dizer $_ } }
      '0' { return }
      default { Dizer 'Opcao invalida.' 'Yellow' }
    }
  } catch {
    Dizer ''
    Dizer "ERRO: $($_.Exception.Message)" 'Red'
    Dizer 'Copie esta mensagem (e as linhas acima dela) e mande para mim.' 'Yellow'
  }
}
