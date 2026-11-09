use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub fn compile_indo_to_php(input_path: &str, output_path: &str) -> io::Result<()> {
    if !Path::new(input_path).exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound, 
            format!("File skrip INDOCODE tidak ditemukan di: {}", input_path)
        ));
    }

    let source_code = fs::read_to_string(input_path)?;
    let mut php_output = String::new();
    
    php_output.push_str("<?php\n");
    php_output.push_str("// ==========================================================\n");
    php_output.push_str("// Diterjemahkan oleh Mesin Universal INDOCODE (Global Connect)\n");
    php_output.push_str("// ==========================================================\n\n");
    
    // =========================================================
    // 1. HIDDEN RUNTIME INJECTION & AUTO-CONNECT
    // =========================================================
    let out_path_obj = Path::new(output_path);
    if let Some(cache_dir) = out_path_obj.parent() {
        let runtime_path = cache_dir.join("indocode_runtime.php");
        
        let core_runtime = r#"<?php
// RUNTIME MESIN INDOCODE (Otomatis dibuat oleh biner indocode.exe)
global $INDO_DB;
$INDO_DB = null;

if (!function_exists('indo_set_database')) {
    function indo_set_database($db_name = null) {
        global $INDO_DB;
        
        // 🚀 BACA .ENV OTOMATIS JIKA DIJALANKAN DARI CMD/CLI
        $env_path = __DIR__ . '/../../.env';
        if (file_exists($env_path)) {
            $lines = file($env_path, FILE_IGNORE_NEW_LINES | FILE_SKIP_EMPTY_LINES);
            foreach ($lines as $line) {
                if (strpos(trim($line), '#') === 0) continue;
                $parts = explode('=', $line, 2);
                if (count($parts) == 2) {
                    $_ENV[trim($parts[0])] = trim($parts[1]);
                }
            }
        }

        $host = $_ENV['DB_HOST'] ?? 'localhost'; 
        $user = $_ENV['DB_USER'] ?? 'root'; 
        $pass = $_ENV['DB_PASS'] ?? '';
        
        if (empty($db_name) || $db_name === 'DINAMIS') {
            $db_name = $_ENV['DB_NAME'] ?? 'indocode_db';
        }

        try {
            $pdo_temp = new PDO("mysql:host=$host", $user, $pass);
            $pdo_temp->setAttribute(PDO::ATTR_ERRMODE, PDO::ERRMODE_EXCEPTION);
            $pdo_temp->exec("CREATE DATABASE IF NOT EXISTS `$db_name` CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci");
            
            $INDO_DB = new PDO("mysql:host=$host;dbname=$db_name;charset=utf8mb4", $user, $pass);
            $INDO_DB->setAttribute(PDO::ATTR_ERRMODE, PDO::ERRMODE_EXCEPTION);
        } catch (PDOException $e) { die("Gagal Koneksi DB: " . $e->getMessage()); }
    }

    function indo_migrasi_tabel($tabel, $struktur_kolom) {
        global $INDO_DB;
        
        // 1. Buat tabel dasar jika belum wujud
        $sql_create = "CREATE TABLE IF NOT EXISTS `$tabel` (
            id INT AUTO_INCREMENT PRIMARY KEY, 
            tanggal_input TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )";
        try { $INDO_DB->exec($sql_create); } catch (PDOException $e) {}

        // 2. Ambil daftar kolom yang sudah ada di database
        $existing_columns = [];
        try {
            $stmt = $INDO_DB->query("SHOW COLUMNS FROM `$tabel`");
            while ($row = $stmt->fetch(PDO::FETCH_ASSOC)) {
                $existing_columns[] = strtolower($row['Field']);
            }
        } catch (PDOException $e) {}

        // 3. PARSER CERDAS: Memecah definisi kolom tanpa memutus koma di dalam ENUM(...)
        $kolom_array = [];
        $buffer = '';
        $depth = 0;
        $len = strlen($struktur_kolom);
        for ($i = 0; $i < $len; $i++) {
            $char = $struktur_kolom[$i];
            if ($char === '(') {
                $depth++;
            } else if ($char === ')') {
                $depth--;
            } else if ($char === ',' && $depth === 0) {
                $kolom_array[] = trim($buffer);
                $buffer = '';
                continue;
            }
            $buffer .= $char;
        }
        if (trim($buffer) !== '') {
            $kolom_array[] = trim($buffer);
        }

        // 4. SINKRONISASI CERDAS (Auto Diffing & ALTER TABLE)
        foreach ($kolom_array as $def) {
            $def = trim($def);
            if (empty($def)) continue;
            
            // Ekstrak nama kolom dengan regex agar aman dari spasi berlebih
            $parts = preg_split('/\s+/', $def);
            $nama_kolom = strtolower(str_replace('`', '', trim($parts[0])));
            
            // Tambahkan kolom jika belum ada
            if (!in_array($nama_kolom, $existing_columns) && $nama_kolom !== 'id' && $nama_kolom !== 'tanggal_input') {
                $sql_alter = "ALTER TABLE `$tabel` ADD COLUMN $def";
                try { $INDO_DB->exec($sql_alter); } catch (PDOException $e) {
                    echo "Gagal Alter Kolom '$nama_kolom': " . $e->getMessage() . "\n";
                }
            }
        }
        
        // Seeding otomatis admin
        if ($tabel === 'admin') {
            try {
                $cek = $INDO_DB->query("SELECT COUNT(*) FROM `admin`")->fetchColumn();
                if ($cek == 0) {
                    $stmt = $INDO_DB->prepare("INSERT INTO `admin` (username, password) VALUES (?, ?)");
                    $stmt->execute(['admin', password_hash('admin123', PASSWORD_DEFAULT)]);
                }
            } catch (PDOException $e) {}
        }
    }

    function indo_simpan_form($tabel) {
        global $INDO_DB;
        if ($_SERVER['REQUEST_METHOD'] === 'POST' && isset($_POST['indocode_tabel']) && $_POST['indocode_tabel'] === $tabel) {
            $id = isset($_POST['id']) ? trim($_POST['id']) : '';
            $kolom = []; $nilai = []; $placeholders = []; $set_update = [];
            foreach ($_POST as $key => $val) {
                if ($key === 'aksi' || $key === 'id' || $key === 'indocode_tabel') continue;
                $k = "`" . htmlspecialchars(trim($key)) . "`";
                $v = trim($val);
                $kolom[] = $k; $nilai[] = $v; $placeholders[] = "?"; $set_update[] = "$k = ?";
            }
            if (!empty($kolom)) {
                if (!empty($id)) {
                    $set_str = implode(", ", $set_update);
                    $stmt = $INDO_DB->prepare("UPDATE `$tabel` SET $set_str WHERE id = ?");
                    $nilai[] = $id;
                    $stmt->execute($nilai);
                } else {
                    $col_str = implode(", ", $kolom);
                    $plc_str = implode(", ", $placeholders);
                    $stmt = $INDO_DB->prepare("INSERT INTO `$tabel` ($col_str) VALUES ($plc_str)");
                    $stmt->execute($nilai);
                }
            }
            header("Location: " . strtok($_SERVER["REQUEST_URI"], "?"));
            exit;
        }
    }

    function indo_hapus_data($tabel) {
        global $INDO_DB;
        if (isset($_GET['aksi']) && $_GET['aksi'] === 'hapus' && isset($_GET['tabel']) && $_GET['tabel'] === $tabel && isset($_GET['id'])) {
            $stmt = $INDO_DB->prepare("DELETE FROM `$tabel` WHERE id = ?");
            $stmt->execute([intval($_GET['id'])]);
            header("Location: " . strtok($_SERVER["REQUEST_URI"], "?"));
            exit;
        }
    }

    function indo_ambil_edit($tabel) {
        global $INDO_DB;
        if (isset($_GET['edit_id'])) {
            try {
                $stmt = $INDO_DB->prepare("SELECT * FROM `$tabel` WHERE id = ?");
                $stmt->execute([intval($_GET['edit_id'])]);
                return $stmt->fetch(PDO::FETCH_ASSOC);
            } catch (PDOException $e) { return null; }
        }
        return null;
    }

    function indo_ambil_semua($tabel) {
        global $INDO_DB;
        try {
            $stmt = $INDO_DB->query("SELECT * FROM `$tabel` ORDER BY id DESC");
            return $stmt->fetchAll(PDO::FETCH_ASSOC);
        } catch (PDOException $e) { return []; }
    }

    function indo_hitung_total($tabel) {
        global $INDO_DB;
        try {
            $stmt = $INDO_DB->query("SELECT COUNT(*) as total FROM `$tabel`");
            $res = $stmt->fetch(PDO::FETCH_ASSOC);
            return $res ? $res['total'] : 0;
        } catch (PDOException $e) { return 0; }
    }

    function indo_hitung_grup($tabel, $kolom) {
        global $INDO_DB;
        try {
            $stmt = $INDO_DB->query("SELECT `$kolom` as label, COUNT(*) as jumlah FROM `$tabel` GROUP BY `$kolom`");
            return $stmt->fetchAll(PDO::FETCH_ASSOC);
        } catch (PDOException $e) { return []; }
    }
    
    function indo_proses_login($tabel) {
        global $INDO_DB;
        if (session_status() === PHP_SESSION_NONE) { session_start(); }
        if ($_SERVER['REQUEST_METHOD'] === 'POST' && isset($_POST['username']) && isset($_POST['password'])) {
            $username = trim($_POST['username']);
            $password = trim($_POST['password']);
            try {
                $stmt = $INDO_DB->prepare("SELECT * FROM `$tabel` WHERE username = ?");
                $stmt->execute([$username]);
                $admin = $stmt->fetch(PDO::FETCH_ASSOC);
                
                if ($admin && password_verify($password, $admin['password'])) {
                    $_SESSION['indocode_admin'] = $admin['username'];
                    header("Location: beranda");
                    exit;
                } else {
                    $GLOBALS['login_error'] = "Username atau Password salah!";
                }
            } catch (PDOException $e) {
                $GLOBALS['login_error'] = "Kesalahan Sistem: " . $e->getMessage();
            }
        }
    }

    function indo_wajib_login() {
        if (session_status() === PHP_SESSION_NONE) { session_start(); }
        if (!isset($_SESSION['indocode_admin'])) {
            header("Location: login");
            exit;
        }
    }

    function indo_logout() {
        if (session_status() === PHP_SESSION_NONE) { session_start(); }
        unset($_SESSION['indocode_admin']);
        session_destroy();
        header("Location: login");
        exit;
    }
}

// 🚀 EKSEKUSI OTOMATIS KONEKSI GLOBAL DI SETIAP SCRIPT PROSES
indo_set_database();
?>
"#;
        fs::write(&runtime_path, core_runtime)?;
        
        let req_path = runtime_path.to_string_lossy().replace("\\", "/");
        php_output.push_str(&format!("require_once '{}';\n\n", req_path));
    }

    // =========================================================
    // 2. PARSER BARIS PER BARIS DENGAN DUKUNGAN MULTILINE
    // =========================================================
    let mut in_skema = false;
    let mut skema_nama = String::new();
    let mut skema_kolom = String::new();

    for line in source_code.lines() {
        let trimmed = line.trim();
        
        if trimmed.is_empty() { php_output.push('\n'); continue; }
        if trimmed.starts_with('#') { php_output.push_str(&format!("//{}\n", &trimmed[1..])); continue; }
        if trimmed == "MULAI PROGRAM" { php_output.push_str("// [INDOCODE] Program Dimulai\n"); continue; }
        if trimmed == "AKHIR PROGRAM" { php_output.push_str("// [INDOCODE] Program Berakhir\n"); continue; }
        if trimmed.starts_with("TAMPILKAN") {
            let content = trimmed.strip_prefix("TAMPILKAN").unwrap().trim();
            php_output.push_str(&format!("echo {};\n", content)); continue;
        }

        // -> TAMBAHKAN PENANGANAN MODUL HEADER/FOOTER (SINTAKS KURUNG SIKU)
        if trimmed.starts_with("[pakai ") && trimmed.ends_with("\"]") {
            let path = trimmed
                .strip_prefix("[pakai \"").unwrap()
                .strip_suffix("\"]").unwrap();
            
            php_output.push_str(&format!("?>\n<?php include __DIR__ . '/../{}'; ?>\n<?php\n", path));
            continue;
        }

        // -> TAMBAHKAN PENANGANAN MODUL (SINTAKS HURUF KAPITAL STANDAR)
        if trimmed.starts_with("PAKAI ") {
            let path = trimmed.strip_prefix("PAKAI ").unwrap().trim_matches('"');
            php_output.push_str(&format!("include __DIR__ . '/../{}';\n", path));
            continue;
        }

        if trimmed.starts_with("SET DATABASE") {
            let param = trimmed.strip_prefix("SET DATABASE").unwrap().trim();
            if param.is_empty() || param.eq_ignore_ascii_case("DINAMIS") || param.eq_ignore_ascii_case("OTOMATIS") {
                php_output.push_str("indo_set_database();\n");
            } else {
                let db = param.trim_matches('"');
                php_output.push_str(&format!("indo_set_database('{}');\n", db));
            }
            continue;
        }

        // PENANGANAN BLOK MULTILINE SKEMA TABEL
        if in_skema {
            if trimmed.starts_with(')') {
                // AMANKAN TANDA PETIK SATU AGAR TIDAK MERUSAK STRING PHP
                let safe_kolom = skema_kolom.replace('\'', "\\'");
                php_output.push_str(&format!("indo_migrasi_tabel('{}', '{}');\n", skema_nama, safe_kolom));
                in_skema = false;
            } else {
                skema_kolom.push_str(trimmed);
                skema_kolom.push_str(" ");
            }
            continue;
        }

        // PENANGANAN SKEMA / MIGRASI (Support 1 Baris & Banyak Baris)
        if trimmed.starts_with("MIGRASI TABEL") || trimmed.starts_with("SKEMA TABEL") {
            let sisa = if trimmed.starts_with("MIGRASI TABEL") {
                trimmed.strip_prefix("MIGRASI TABEL").unwrap().trim()
            } else {
                trimmed.strip_prefix("SKEMA TABEL").unwrap().trim()
            };

            if sisa.ends_with('(') || sisa.ends_with("()") {
                if sisa.ends_with("()") {
                    let tabel = sisa[..sisa.len()-2].trim();
                    php_output.push_str(&format!("indo_migrasi_tabel('{}', '');\n", tabel));
                } else {
                    skema_nama = sisa[..sisa.len()-1].trim().to_string();
                    skema_kolom = String::new();
                    in_skema = true;
                }
            } else if let Some(idx_buka) = sisa.find('(') {
                let tabel = sisa[..idx_buka].trim();
                if sisa.ends_with(')') {
                    let kolom = sisa[idx_buka + 1..sisa.len() - 1].trim();
                    let safe_kolom = kolom.replace('\'', "\\'");
                    php_output.push_str(&format!("indo_migrasi_tabel('{}', '{}');\n", tabel, safe_kolom));
                }
            }
            continue;
        }

        if trimmed.starts_with("GUNAKAN ") || trimmed.starts_with("MUAT ") {
            php_output.push_str(&format!("// [INDOCODE] Mengabaikan modul lama: {}\n", trimmed));
            continue;
        }

        if trimmed.starts_with("SIMPAN FORM KE TABEL") {
            let tabel = trimmed.strip_prefix("SIMPAN FORM KE TABEL").unwrap().trim();
            php_output.push_str(&format!("indo_simpan_form('{}');\n", tabel)); continue;
        }

        if trimmed.starts_with("TANGKAP AKSI HAPUS TABEL") {
            let tabel = trimmed.strip_prefix("TANGKAP AKSI HAPUS TABEL").unwrap().trim();
            php_output.push_str(&format!("indo_hapus_data('{}');\n", tabel)); continue;
        }

        if trimmed.contains(" = AMBIL EDIT DARI ") {
            let parts: Vec<&str> = trimmed.split(" = AMBIL EDIT DARI ").collect();
            let var_name = parts[0].trim().to_lowercase();
            let tabel = parts[1].trim();
            php_output.push_str(&format!("${} = indo_ambil_edit('{}');\n", var_name, tabel));
            continue;
        }

        if trimmed.contains(" = AMBIL SEMUA DARI ") {
            let parts: Vec<&str> = trimmed.split(" = AMBIL SEMUA DARI ").collect();
            let var_name = parts[0].trim().to_lowercase(); 
            let tabel = parts[1].trim();
            php_output.push_str(&format!("${} = indo_ambil_semua('{}');\n", var_name, tabel));
            continue;
        }

        if trimmed.contains(" = HITUNG TOTAL DARI ") {
            let parts: Vec<&str> = trimmed.split(" = HITUNG TOTAL DARI ").collect();
            let var_name = parts[0].trim().to_lowercase();
            let tabel = parts[1].trim();
            php_output.push_str(&format!("${} = indo_hitung_total('{}');\n", var_name, tabel));
            continue;
        }

        if trimmed.contains(" = HITUNG GRUP DARI ") && trimmed.contains(" BERDASARKAN ") {
            let parts: Vec<&str> = trimmed.split(" = HITUNG GRUP DARI ").collect();
            let var_name = parts[0].trim().to_lowercase();
            let sisa: Vec<&str> = parts[1].split(" BERDASARKAN ").collect();
            let tabel = sisa[0].trim(); 
            let kolom = sisa[1].trim();
            php_output.push_str(&format!("${} = indo_hitung_grup('{}', '{}');\n", var_name, tabel, kolom));
            continue;
        }

        if trimmed.starts_with("PROSES LOGIN ADMIN DARI TABEL") {
            let tabel = trimmed.strip_prefix("PROSES LOGIN ADMIN DARI TABEL").unwrap().trim();
            php_output.push_str(&format!("indo_proses_login('{}');\n", tabel));
            continue;
        }

        if trimmed == "WAJIB LOGIN ADMIN" {
            php_output.push_str("indo_wajib_login();\n");
            continue;
        }

        if trimmed == "PROSES LOGOUT ADMIN" {
            php_output.push_str("indo_logout();\n");
            continue;
        }

        php_output.push_str(&format!("{}\n", trimmed));
    }

    php_output.push_str("\n?>");
    let mut file = fs::File::create(output_path)?;
    file.write_all(php_output.as_bytes())?;
    Ok(())
}