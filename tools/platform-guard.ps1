$hits = Get-ChildItem src -Recurse -Filter *.rs |
    Where-Object { $_.FullName -notmatch '\\platform\\' } |
    Select-String '(?<!#\[)\bmacroquad::' |
    Where-Object { $_.Line.Trim() -notmatch '^//' }
if ($hits) {
    $hits | ForEach-Object { "$($_.Filename):$($_.LineNumber): $($_.Line.Trim())" }
    Write-Error "macroquad usado fora de src/platform"; exit 1
}
"platform-guard: OK"
