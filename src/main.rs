mod ast;
mod compiler;
mod evaluator;
mod lexer;
mod parser;
mod token;

use compiler::compile_indo_to_php;
use evaluator::Evaluator;
use lexer::Lexer;
use parser::Parser;
use std::env;
use std::fs;
use std::io;
use std::path::Path;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("==================================================");
        println!("🚀 MESIN INDOCODE v1.0 (Developer: Fahmi)");
        println!("==================================================");
        println!("Penggunaan:");
        println!("  1. Interpreter   : indocode <nama_file.indo>");
        println!("  2. Transpilasi   : indocode <nama_file.indo> --transpile <output.php>");
        println!("  3. Scaffolding   : indocode init <nama_proyek>");
        println!("  4. Bersihkan Cache: indocode clear");
        println!("  5. Sinkronisasi DB: indocode migrasi");
        println!("==================================================");
        return;
    }

    let arg_pertama = &args[1];

    if arg_pertama == "--version" || arg_pertama == "-v" {
        println!("INDOCODE v1.0.0 (Developer: Fahmi Muslim)");
        return;
    }

    if arg_pertama == "clear" {
        clearkan_cache_proyek();
        return;
    }

    // =========================================================================
    // FITUR BARU: AUTO-SYNC SCHEMA (DECLARATIVE MIGRATION)
    // =========================================================================
    if arg_pertama == "migrasi" {
        println!("🔄 Memulai Sinkronisasi Database Cerdas INDOCODE...");
        let base_dir = "private_core/database";
        let skema_path = "private_core/database/skema.indo";
        let temp_php = "private_core/cache/migrasi_temp.php";

        // Jika file skema belum ada, buatkan otomatis!
        if !Path::new(skema_path).exists() {
            fs::create_dir_all(base_dir).unwrap_or_default();
            let default_skema = r#"MULAI PROGRAM

# Tulis cetak biru semua tabel database Anda di sini
SKEMA TABEL penduduk (
    nik VARCHAR(20) UNIQUE,
    nama VARCHAR(150),
    alamat TEXT
)

AKHIR PROGRAM
"#;
            fs::write(skema_path, default_skema).unwrap_or_default();
            println!("✔ Cetak biru dibuat otomatis di: {}", skema_path);
            println!("Silakan edit file tersebut sesuai kebutuhan dan jalankan 'indocode migrasi' lagi.");
            return;
        }

        // Pastikan folder cache wujud sebelum transpilasi
        fs::create_dir_all("private_core/cache").unwrap_or_default();

        // Transpilasi Skema ke file sementara dan Eksekusi Langsung via PHP CLI
        match compile_indo_to_php(skema_path, temp_php) {
            Ok(_) => {
                use std::process::Command;
                let status = Command::new("php")
                    .arg(temp_php)
                    .status();

                match status {
                    Ok(s) if s.success() => {
                        println!("✅ Skema tersinkronisasi murni dengan Database!");
                        let _ = fs::remove_file(temp_php); // Hapus file cache sementara
                    },
                    _ => println!("❌ Gagal mengeksekusi PHP (Pastikan 'php' masuk di System Environment Path komputer Anda)."),
                }
            }
            Err(e) => println!("❌ Gagal membaca file skema: {}", e),
        }
        return;
    }

    if arg_pertama == "init" {
        if args.len() < 3 {
            println!("❌ Error: Masukkan nama proyek. Contoh: indocode init web_desa");
            return;
        }
        let nama_proyek = &args[2];
        if let Err(e) = init_project(nama_proyek) {
            println!("❌ Gagal membangun proyek: {}", e);
        }
        return;
    }

    if args.len() >= 4 && args[2] == "--transpile" {
        let output_php = &args[3];
        match compile_indo_to_php(arg_pertama, output_php) {
            Ok(_) => {
                println!("Sukses: Berkas '{}' berhasil ditranspilasi ke '{}'", arg_pertama, output_php);
            }
            Err(e) => {
                println!("[KESALAHAN TRANSFILASI INDOCODE]: {}", e);
            }
        }
        return;
    }

    let kode_sumber = match fs::read_to_string(arg_pertama) {
        Ok(isi) => isi,
        Err(e) => {
            println!("[KESALAHAN INDOCODE]: Gagal membaca file '{}': {}", arg_pertama, e);
            return;
        }
    };

    let mut lexer = Lexer::baru(&kode_sumber);
    match lexer.analisis_leksikal() {
        Ok(tokens) => {
            let mut parser = Parser::baru(tokens);
            match parser.parse() {
                Ok(ast) => {
                    let mut evaluator = Evaluator::baru();
                    if let Err(e) = evaluator.eksekusi(&ast) {
                        println!("\n[KESALAHAN EKSEKUSI INDOCODE]: {}", e);
                    }
                }
                Err(e) => {
                    println!("\n[KESALAHAN PARSER INDOCODE]: {}", e);
                }
            }
        }
        Err(e) => {
            println!("\n[KESALAHAN LEXER INDOCODE]: {}", e);
        }
    }
}

fn clearkan_cache_proyek() {
    let cache_dirs = vec!["private_core/cache", "cache"];
    let mut cleared = false;

    for dir in cache_dirs {
        if Path::new(dir).exists() {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        let _ = fs::remove_file(path);
                    }
                }
                println!("✔ Sukses! Folder cache '{}' berhasil dibersihkan.", dir);
                cleared = true;
            }
        }
    }

    if !cleared {
        println!("⚠ Folder cache tidak ditemukan di direktori saat ini.");
    }
}

pub fn init_project(project_name: &str) -> io::Result<()> {
    let base_path = Path::new(project_name);
    
    // 1. Buat Struktur Folder
    let folders = vec![
        base_path.join("public_html"),
        base_path.join("public_html/assets/css"),
        base_path.join("public_html/assets/js"),
        base_path.join("public_html/assets/img"),
        base_path.join("private_core"),
        base_path.join("private_core/bin"),
        base_path.join("private_core/database"), // Folder baru untuk skema.indo
        base_path.join("private_core/proses"),
        base_path.join("private_core/casing"),
        base_path.join("private_core/cache"),
    ];

    for folder in folders {
        fs::create_dir_all(&folder)?;
        println!("✔ Folder dibuat: {:?}", folder);
    }

    // 2. File .env (Pusat Kendali Nama Database Dinamis)
    let env_content = format!(
        "DB_HOST=localhost\nDB_USER=root\nDB_PASS=\nDB_NAME={}_db\nAPP_ENV=local\n",
        project_name
    );
    fs::write(base_path.join(".env"), env_content)?;
    println!("✔ File .env dibuat");

    // 3. File .htaccess di dalam public_html
    let htaccess_content = r#"<IfModule mod_rewrite.c>
    RewriteEngine On
    RewriteCond %{REQUEST_FILENAME} !-f
    RewriteCond %{REQUEST_FILENAME} !-d
    RewriteRule ^(.*)$ index.php?$1 [L]
</IfModule>
"#;
    fs::write(base_path.join("public_html/.htaccess"), htaccess_content)?;
    println!("✔ File public_html/.htaccess dibuat");

    // 4. File CSS bawaan
    let css_content = "/* Styling Global INDOCODE */\nbody { font-family: ui-sans-serif, system-ui, sans-serif; }\n";
    fs::write(base_path.join("public_html/assets/css/style.css"), css_content)?;
    println!("✔ Berkas public_html/assets/css/style.css dibuat");

    // 5. File Beranda Proses (Sangat bersih, tanpa deklarasi database manual)
    let beranda_proses = r#"MULAI PROGRAM

# Contoh pengambilan data untuk casing beranda
# data_berita = AMBIL SEMUA DARI berita

AKHIR PROGRAM
"#;
    fs::write(base_path.join("private_core/proses/BerandaProses.indo"), beranda_proses)?;
    println!("✔ Berkas private_core/proses/BerandaProses.indo dibuat");

    // 6. File Beranda Casing (Halaman Selamat Datang ala Laravel)
    let beranda_casing = r#"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>INDOCODE - Bahasa Pemrograman Web Mandiri</title>
    <style>
        :root {
            --color-bg: #F8FAFC;
            --color-card: #FFFFFF;
            --color-text: #334155;
            --color-muted: #64748B;
            --color-border: #E2E8F0;
            --color-accent: #2563EB;
        }
        body {
            font-family: ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
            background-color: var(--color-bg);
            color: var(--color-text);
            margin: 0;
            padding: 0;
            min-height: 100vh;
            display: flex;
            flex-direction: column;
            justify-content: space-between;
        }
        .top-right {
            position: absolute;
            top: 25px;
            right: 35px;
            display: flex;
            gap: 15px;
        }
        .top-right a {
            color: var(--color-muted);
            text-decoration: none;
            font-weight: 600;
            font-size: 14px;
            padding: 8px 16px;
            border-radius: 6px;
            border: 1px solid var(--color-border);
            background: var(--color-card);
            transition: all 0.2s;
        }
        .top-right a:hover {
            border-color: var(--color-accent);
            color: var(--color-accent);
        }
        .content {
            max-width: 900px;
            margin: 0 auto;
            padding: 60px 20px;
            text-align: center;
            flex-grow: 1;
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
        }
        .logo-container h1 {
            font-size: 56px;
            font-weight: 800;
            letter-spacing: -1px;
            color: #0F172A;
            margin: 0;
        }
        .logo-container p {
            font-size: 16px;
            color: var(--color-muted);
            margin-top: 5px;
            letter-spacing: 2px;
            text-transform: uppercase;
            font-weight: 600;
        }
        .description {
            font-size: 18px;
            color: var(--color-muted);
            max-width: 650px;
            margin-bottom: 40px;
            line-height: 1.6;
        }
        .links-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
            gap: 20px;
            width: 100%;
            max-width: 800px;
            margin-top: 10px;
        }
        .link-card {
            background: var(--color-card);
            border: 1px solid var(--color-border);
            border-radius: 10px;
            padding: 25px;
            text-align: left;
            text-decoration: none;
            color: inherit;
            box-shadow: 0 1px 3px rgba(0,0,0,0.02);
            transition: transform 0.2s, border-color 0.2s;
        }
        .link-card:hover {
            transform: translateY(-3px);
            border-color: var(--color-accent);
        }
        .link-card h3 {
            margin: 0 0 8px 0;
            font-size: 17px;
            color: #0F172A;
        }
        .link-card p {
            margin: 0;
            font-size: 14px;
            color: var(--color-muted);
            line-height: 1.5;
        }
        footer {
            padding: 25px;
            text-align: center;
            font-size: 13px;
            color: var(--color-muted);
            border-top: 1px solid var(--color-border);
            background: var(--color-card);
            display: flex;
            justify-content: center;
            gap: 30px;
        }
    </style>
</head>
<body>
    <div class="top-right">
        <a href="/login">Panel Admin</a>
    </div>
    <div class="content">
        <div class="logo-container">
            <h1>INDOCODE</h1>
            <p>Pure Standalone Framework</p>
        </div>
        <p class="description">
            Selamat datang di kerangka kerja web berkecepatan tinggi berbasis bahasa Indonesia yang digerakkan oleh kompilator Rust.
        </p>
        <div class="links-grid">
            <div class="link-card">
                <h3>📖 Dokumentasi</h3>
                <p>Pelajari panduan sintaks, migrasi tabel, dan sistem pengendali rute INDOCODE.</p>
            </div>
            <div class="link-card">
                <h3>⚡ Ekosistem</h3>
                <p>Eksplorasi modul pustaka siap pakai untuk mempercepat pengembangan aplikasi web.</p>
            </div>
            <a href="/login" class="link-card">
                <h3>🔒 Otentikasi Admin</h3>
                <p>Masuk ke sistem menggunakan kredensial bawaan untuk mengelola basis data.</p>
            </a>
            <div class="link-card">
                <h3>💻 Compiler Rust</h3>
                <p>Dibangun sepenuhnya dari inti kode biner kustom <code>indocode.exe</code>.</p>
            </div>
        </div>
    </div>
    <footer>
        <span><strong>INDOCODE</strong> v1.0.0</span>
        <span>•</span>
        <span>Developer: Fahmi Muslim</span>
    </footer>
</body>
</html>
"#;
    fs::write(base_path.join("private_core/casing/beranda.indo"), beranda_casing)?;
    println!("✔ Berkas private_core/casing/beranda.indo dibuat");

    // 7. Pintu Gerbang Utama (index.php) dengan JIT Auto-Compiler Bawaan
    let index_php_content = r#"<?php
// ==========================================================
// PINTU GERBANG UTAMA INDOCODE (SINGLE ENTRY POINT)
// Dilengkapi JIT (Just-In-Time) Auto-Compiler
// ==========================================================

$env_path = __DIR__ . '/../.env';
if (file_exists($env_path)) {
    $lines = file($env_path, FILE_IGNORE_NEW_LINES | FILE_SKIP_EMPTY_LINES);
    foreach ($lines as $line) {
        if (strpos(trim($line), '#') === 0) continue;
        list($name, $value) = explode('=', $line, 2);
        $_ENV[trim($name)] = trim($value);
    }
}

$request_uri = strtok($_SERVER['REQUEST_URI'], '?');
$base_path = '/'; 
$route = str_replace($base_path, '', $request_uri);
if (empty($route) || $route == '/') { $route = 'beranda'; }

$biner_indo = realpath(__DIR__ . '/../private_core/bin/indocode.exe'); 

// ==========================================================
// 1. EKSEKUSI FILE PROSES (Logika INDOCODE -> PHP)
// ==========================================================
$proses_indo = __DIR__ . '/../private_core/proses/' . ucfirst($route) . 'Proses.indo';
$proses_php  = __DIR__ . '/../private_core/cache/' . md5($route . '_proses') . '.php';

if (file_exists($proses_indo)) {
    if (!file_exists($proses_php) || filemtime($proses_indo) > filemtime($proses_php)) {
        if ($biner_indo) {
            exec("\"$biner_indo\" \"$proses_indo\" --transpile \"$proses_php\"");
        }
    }
    if (file_exists($proses_php)) { 
        require_once $proses_php; 
    }
}

// ==========================================================
// 2. EKSEKUSI FILE CASING (Tampilan HTML / View Langsung)
// ==========================================================
$casing_indo = __DIR__ . '/../private_core/casing/' . $route . '.indo';

if (file_exists($casing_indo)) {
    $casing_php = __DIR__ . '/../private_core/cache/' . md5($route . '_casing') . '.php';
    if (!file_exists($casing_php) || filemtime($casing_indo) > filemtime($casing_php)) {
        copy($casing_indo, $casing_php);
    }
    require_once $casing_php;
} else {
    http_response_code(404);
    echo "<h1>404 - Halaman '$route' tidak ditemukan.</h1>";
}
?>
"#;
    fs::write(base_path.join("public_html/index.php"), index_php_content)?;
    println!("✔ Pintu gerbang public_html/index.php dibuat");

    // 8. Auto-Copy Biner indocode.exe ke Proyek Baru
    match env::current_exe() {
        Ok(exe_path) => {
            let target_bin = base_path.join("private_core/bin/indocode.exe");
            if let Err(e) = fs::copy(&exe_path, &target_bin) {
                println!("⚠ Peringatan: Gagal menyalin indocode.exe otomatis: {}", e);
            } else {
                println!("✔ Biner indocode.exe berhasil disalin ke private_core/bin/");
            }
        }
        Err(e) => {
            println!("⚠ Peringatan: Tidak dapat mendeteksi path biner aktif: {}", e);
        }
    }

    println!("🎉 PROYEK PURE INDOCODE '{}' BERHASIL DIBANGUN!", project_name);
    Ok(())
}