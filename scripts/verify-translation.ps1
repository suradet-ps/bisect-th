$ErrorActionPreference = 'Stop'
$orig = 'C:\Users\goyga\Desktop\suradet-ps\cargo-bisect-rustc\guide\src'
$trans = 'C:\Users\goyga\Desktop\suradet-ps\bisect-th\guide\src'

function Read-Normalized($path) {
    return ([System.IO.File]::ReadAllText($path)).Replace("`r`n", "`n")
}

function Get-CodeBlocks($path) {
    $content = Read-Normalized $path
    $rx = [regex]'```(?s:.*?)```'
    return @($rx.Matches($content) | ForEach-Object { $_.Value })
}

function Get-Headings($path) {
    $content = Read-Normalized $path
    $rx = [regex]'(?m)^#{1,6} .*$'
    return @($rx.Matches($content) | ForEach-Object { ($_.Value -replace '^#+ ','') -replace '^#+','' | Out-Null; $_.Value })
}

function Get-RefLinks($path) {
    $content = Read-Normalized $path
    $rx = [regex]'(?m)^\[[^\]]+\]:\s+\S+.*$'
    return @($rx.Matches($content) | ForEach-Object { $_.Value -replace '\s+$','' })
}

function Get-InlineLinkTargets($path) {
    $content = Read-Normalized $path
    $rx = [regex]'\[[^\]]*\]\(([^)]+)\)'
    return @($rx.Matches($content) | ForEach-Object { $_.Groups[1].Value -replace '\s+$','' })
}

$origFiles = Get-ChildItem -Recurse -File $orig -Filter *.md
$fail = 0
$total = 0

foreach ($f in $origFiles) {
    $rel = $f.FullName.Substring($orig.Length + 1)
    $tPath = Join-Path $trans $rel
    $total++
    if (-not (Test-Path -LiteralPath $tPath)) {
        Write-Output "[FAIL] $rel : missing translated file"
        $fail++
        continue
    }

    $oc = Get-CodeBlocks $f.FullName
    $tc = Get-CodeBlocks $tPath
    if ($oc.Count -ne $tc.Count) {
        Write-Output "[FAIL] $rel : code block count differs (orig=$($oc.Count) trans=$($tc.Count))"
        $fail++
    } else {
        for ($i = 0; $i -lt $oc.Count; $i++) {
            if ($oc[$i] -cne $tc[$i]) {
                Write-Output "[FAIL] $rel : code block #$($i+1) differs"
                $fail++
            }
        }
    }

    $oh = Get-Headings $f.FullName
    $th = Get-Headings $tPath
    if ($oh.Count -ne $th.Count) {
        Write-Output "[FAIL] $rel : heading count differs (orig=$($oh.Count) trans=$($th.Count))"
        $fail++
    } else {
        for ($i = 0; $i -lt $oh.Count; $i++) {
            $ol = ($oh[$i] -split ' ')[0]
            $tl = ($th[$i] -split ' ')[0]
            if ($ol -cne $tl) {
                Write-Output "[FAIL] $rel : heading #$($i+1) level differs (orig='$($oh[$i])' trans='$($th[$i])')"
                $fail++
            }
        }
    }

    $or = Get-RefLinks $f.FullName
    $tr = Get-RefLinks $tPath
    if ($or.Count -ne $tr.Count) {
        Write-Output "[FAIL] $rel : ref-link count differs (orig=$($or.Count) trans=$($tr.Count))"
        $fail++
    } else {
        for ($i = 0; $i -lt $or.Count; $i++) {
            $ourl = ($or[$i] -split ':\s*',2)[1]
            $turl = ($tr[$i] -split ':\s*',2)[1]
            if ($ourl -cne $turl) {
                Write-Output "[FAIL] $rel : ref-link #$($i+1) url differs (orig='$ourl' trans='$turl')"
                $fail++
            }
        }
    }

    $oi = Get-InlineLinkTargets $f.FullName
    $ti = Get-InlineLinkTargets $tPath
    $os = $oi | Sort-Object -Unique
    $ts = $ti | Sort-Object -Unique
    $missing = @($os | Where-Object { $_ -notin $ts })
    $extra = @($ts | Where-Object { $_ -notin $os })
    if ($missing.Count -gt 0 -or $extra.Count -gt 0) {
        Write-Output "[FAIL] $rel : inline link targets differ (missing=[$($missing -join ', ')] extra=[$($extra -join ', ')])"
        $fail++
    }
}

Write-Output "---"
Write-Output "Checked $total files, $fail problem(s)"
if ($fail -eq 0) { Write-Output "ALL OK: code blocks, headings, links match 100%" }
exit ($fail -gt 0)