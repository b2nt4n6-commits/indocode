# Script Pembuat Paket Portable INDOCODE
$Versi = "v0.1.0"
$FolderTujuan = "D:\indocode\dist\indocode_$Versi"
$FileZip = "D:\indocode\indocode_$Versi`_portable.zip"

Write-Host "[1/4] Membuat struktur folder distribusi..." -ForegroundColor Cyan
if (Test-Path $FolderTujuan) { Remove-Item -Recurse -Force $FolderTujuan }
New-Item -ItemType Directory -Path "$FolderTujuan\contoh" -Force | Out-Null

Write-Host "[2/4] Menyalin executable dan contoh program..." -ForegroundColor Cyan
Copy-Item "D:\indocode\indo.exe" -Destination "$FolderTujuan\indo.exe"
Copy-Item "D:\indocode\uji_fitur.indo" -Destination "$FolderTujuan\contoh\uji_fitur.indo"

Write-Host "[3/4] Membuat skrip penambah PATH (pasang_path.bat)..." -ForegroundColor Cyan
$BatContent = @"
@echo off
echo Memasang PATH INDOCODE ke Environment Variables User...
setx PATH "%PATH%;%~dp0"
echo.
echo Pemasangan sukses! Silakan buka Command Prompt baru dan ketik: indo
pause
"@
Set-Content -Path "$FolderTujuan\pasang_path.bat" -Value $BatContent -Encoding ASCII

# Membuat Dokumentasi BACA_SAYA.txt
$ReadmeContent = @"
===================================================
INDOCODE Bahasa Pemrograman Indonesia ($Versi)
===================================================

CARA PENGGUNAAN:
1. Jalankan 'pasang_path.bat' (Klik kanan -> Run as Administrator) untuk mendaftarkan perintah 'indo'.
2. Buka Command Prompt (CMD) atau PowerShell baru.
3. Jalankan program contoh:
   indo contoh\uji_fitur.indo

Dibuat dengan Rust Engine Native.
"@
Set-Content -Path "$FolderTujuan\BACA_SAYA.txt" -Value $ReadmeContent -Encoding UTF8

Write-Host "[4/4] Kompresi menjadi file .zip..." -ForegroundColor Cyan
if (Test-Path $FileZip) { Remove-Item -Force $FileZip }
Compress-Archive -Path "$FolderTujuan\*" -DestinationPath $FileZip -CompressionLevel Optimal

Write-Host "Paket Portable Berhasil Dibuat: $FileZip" -ForegroundColor Green