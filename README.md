# 🚀 INDOCODE Framework

> **Framework Web Mandiri Berkecepatan Tinggi yang Digerakkan oleh Kompilator Rust dan Dirancang untuk Pengembang Modern.**

[![Versi](https://img.shields.io/badge/versi-1.0.0-blue.svg)](https://github.com/username-anda/indocode)
[![Lisensi](https://img.shields.io/badge/lisensi-MIT-green.svg)](LICENSE)
[![Bahasa](https://img.shields.io/badge/kompilator-Rust-orange.svg)](https://www.rust-lang.org/)
[![Runtime](https://img.shields.io/badge/runtime-PHP%208+-purple.svg)](https://www.php.net/)

---

## 📖 Tentang INDOCODE

**INDOCODE** adalah framework web independen berbasis bahasa Indonesia yang menggabungkan kecepatan kompilasi tingkat rendah dari **Rust** (`indocode.exe`) dengan fleksibilitas eksekusi **PHP murni**. INDOCODE dirancang untuk menghilangkan kompleksitas penumpukan file riwayat migrasi dan konfigurasi database manual, memberikan pengalaman pengembangan yang bersih, cepat, dan terpusat.

---

## ✨ Fitur Utama

1. **⚡ Kompilator Rust-to-PHP (JIT Auto-Compiler)**
   Ditenagai oleh biner kustom `indocode.exe` yang secara otomatis mentranspilasi logika skrip `.indo` menjadi kode PHP yang dioptimalkan secara *Just-In-Time* di balik layar.
2. **🔄 Declarative Schema Sync (Auto-Diffing Migrasi)**
   Ucapkan selamat tinggal pada ratusan file migrasi riwayat ala framework tradisional. Cukup definisikan cetak biru akhir tabel Anda di `private_core/database/skema.indo`, dan jalankan perintah tunggal untuk menyinkronkan kolom secara cerdas tanpa menghapus data yang ada.
3. **🌐 Koneksi Global Otomatis (`.env`)**
   Pusat kendali database terpusat di file `.env`. Perubahan nama database otomatis memicu pembuatan database baru (`CREATE DATABASE IF NOT EXISTS`) secara aman tanpa memerlukan file koneksi manual di setiap file proses.
4. **📁 Pemisahan Logika & Tampilan (Proses & Casing)**
   Arsitektur modular yang memisahkan logika bisnis backend (`private_core/proses/`) dan antarmuka tampilan frontend (`private_core/casing/`) dengan sistem perutean (*routing*) tunggal berbasis `index.php`.

---

## 🛠️ Perintah CLI Inti (`indocode.exe`)

| Perintah Terminal | Deskripsi Singkat |
| :--- | :--- |
| `indocode init <nama_proyek>` | Membangun kerangka proyek baru beserta struktur folder dan biner compiler. |
| `indocode migrasi` | Melakukan sinkronisasi cerdas (*auto-diffing*) dari file `skema.indo` ke database MySQL/MariaDB. |
| `indocode clear` | Membersihkan seluruh file *cache* kompilasi sementara di dalam proyek. |
| `indocode <file.indo> --transpile <out.php>` | Mentranspilasi file skrip tunggal INDOCODE ke berkas PHP murni. |

---

## 🚀 Panduan Memulai Cepat (Quick Start)

### 1. Inisialisasi Proyek Baru
Buka terminal (CMD/PowerShell) di direktori server lokal Anda, lalu jalankan:
```powershell
indocode init projek_anda

### 2. Konfigurasi Database

Masuk ke dalam folder proyek dan sesuaikan nama database di dalam file .env:
DB_HOST=localhost
DB_USER=root
DB_PASS=
DB_NAME=projek_anda_db

### 3. Jalankan Migrasi Skema

Picu pembuatan cetak biru database otomatis dengan perintah:
cd projek_anda
indocode migrasi

(Edit file private_core/database/skema.indo untuk menambah tabel atau kolom sesuai kebutuhan).

### 4. Jalankan Server Web Lokal

php -S localhost:8000 -t public_html

Buka browser dan akses: http://localhost:8000

📂 Struktur Direktori Proyek
projek_anda/
├── .env                          # Pusat kendali konfigurasi & database
├── public_html/
│   ├── .htaccess                 # Konfigurasi URL Rewriting
│   └── index.php                 # Pintu gerbang tunggal (Single Entry Point & JIT)
└── private_core/
    ├── bin/
    │   └── indocode.exe          # Biner kompilator Rust mandiri
    ├── database/
    │   └── skema.indo            # Cetak biru deklaratif tabel database
    ├── proses/                   # Logika backend & pengendali proses
    ├── casing/                   # Tampilan antarmuka HTML (View)
    └── cache/                    # Penyimpanan hasil transpilasi sementara

👨‍💻 Pengembang

    Fahmi Muslim — Creator & Lead Developer

📝 Lisensi

Proyek ini bersifat open-source di bawah lisensi MIT. Silakan gunakan, modifikasi, dan kembangkan sesuai kebutuhan Anda!
