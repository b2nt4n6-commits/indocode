use crate::ast::{Ekspresi, NilaiLiteral, NodeAST};
use std::collections::HashMap;
use std::process::Command;

#[derive(Debug, Clone, PartialEq)]
pub enum Nilai {
    Angka(f64),
    Teks(String),
    Logika(bool),
    Kosong,
}

impl std::fmt::Display for Nilai {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Nilai::Angka(n) => write!(f, "{}", n),
            Nilai::Teks(s) => write!(f, "{}", s),
            Nilai::Logika(b) => write!(f, "{}", b),
            Nilai::Kosong => write!(f, "KOSONG"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Lingkungan {
    nilai: HashMap<String, Nilai>,
}

impl Lingkungan {
    pub fn baru() -> Self {
        Self {
            nilai: HashMap::new(),
        }
    }

    pub fn set(&mut self, nama: &str, val: Nilai) {
        self.nilai.insert(nama.to_string(), val);
    }

    pub fn get(&self, nama: &str) -> Result<Nilai, String> {
        self.nilai
            .get(nama)
            .cloned()
            .ok_or_else(|| format!("Variabel '{}' tidak ditemukan di memori.", nama))
    }

    pub fn hapus(&mut self, nama: &str) -> Result<(), String> {
        if self.nilai.remove(nama).is_some() {
            Ok(())
        } else {
            Err(format!("Variabel '{}' tidak dapat dihapus karena tidak ditemukan.", nama))
        }
    }
}

pub struct Evaluator {
    lingkungan: Lingkungan,
}

impl Evaluator {
    pub fn baru() -> Self {
        Self {
            lingkungan: Lingkungan::baru(),
        }
    }

    #[allow(dead_code)]
    pub fn get_var(&self, nama: &str) -> Result<Nilai, String> {
        self.lingkungan.get(nama)
    }

    pub fn eksekusi(&mut self, ast: &[NodeAST]) -> Result<(), String> {
        for node in ast {
            self.evaluasi_node(node)?;
        }
        Ok(())
    }

    fn evaluasi_node(&mut self, node: &NodeAST) -> Result<Nilai, String> {
        match node {
            NodeAST::DeklarasiVar { nama, ekspresi } | NodeAST::PenugasanVar { nama, ekspresi } => {
                let val = self.evaluasi_ekspresi(ekspresi)?;
                self.lingkungan.set(nama, val);
                Ok(Nilai::Kosong)
            }
            NodeAST::HapusVar { nama } => {
                self.lingkungan.hapus(nama)?;
                Ok(Nilai::Kosong)
            }
            NodeAST::Cetak { ekspresi } => {
                let val = self.evaluasi_ekspresi(ekspresi)?;
                println!("{}", val);
                Ok(Nilai::Kosong)
            }
            NodeAST::PanggilProgram { nama_program, argumen } => {
                let prog = self.evaluasi_ekspresi(nama_program)?.to_string();
                let arg = if let Some(a) = argumen {
                    self.evaluasi_ekspresi(a)?.to_string()
                } else {
                    String::new()
                };

                let perintah = format!("{} {}", prog, arg).trim().to_string();
                println!("[INDOCODE NATIVE RUST]: Menjalankan '{}'...", perintah);

                #[cfg(target_os = "windows")]
                let output = Command::new("cmd")
                    .args(["/C", &perintah])
                    .status();

                #[cfg(not(target_os = "windows"))]
                let output = Command::new("sh")
                    .arg("-c")
                    .arg(&perintah)
                    .status();

                if let Err(e) = output {
                    println!("[KESALAHAN EKSEKUSI]: Gagal menjalankan perintah: {}", e);
                }
                Ok(Nilai::Kosong)
            }
            NodeAST::Percabangan { kondisi, blok_jika, blok_tidak } => {
                let cond_val = self.evaluasi_ekspresi(kondisi)?;
                let is_true = match cond_val {
                    Nilai::Logika(b) => b,
                    _ => return Err("Kondisi JIKA harus menghasilkan nilai Logika (BENAR/SALAH)".to_string()),
                };

                if is_true {
                    for stmt in blok_jika {
                        self.evaluasi_node(stmt)?;
                    }
                } else {
                    for stmt in blok_tidak {
                        self.evaluasi_node(stmt)?;
                    }
                }
                Ok(Nilai::Kosong)
            }
            NodeAST::Perulangan { jumlah_ekspresi, isi_blok } => {
                let count_val = self.evaluasi_ekspresi(jumlah_ekspresi)?;
                let count = match count_val {
                    Nilai::Angka(n) => n as usize,
                    _ => return Err("Jumlah perulangan harus berupa Angka".to_string()),
                };

                for _ in 0..count {
                    for stmt in isi_blok {
                        self.evaluasi_node(stmt)?;
                    }
                }
                Ok(Nilai::Kosong)
            }
            NodeAST::EkspresiStmt(expr) => self.evaluasi_ekspresi(expr),
            _ => Ok(Nilai::Kosong),
        }
    }

    fn evaluasi_ekspresi(&mut self, expr: &Ekspresi) -> Result<Nilai, String> {
        match expr {
            Ekspresi::Literal(l) => match l {
                NilaiLiteral::Angka(n) => Ok(Nilai::Angka(*n)),
                NilaiLiteral::Teks(s) => Ok(Nilai::Teks(s.clone())),
                NilaiLiteral::Logika(b) => Ok(Nilai::Logika(*b)),
            },
            Ekspresi::ReferensiVar(nama) => self.lingkungan.get(nama),
            Ekspresi::OperasiBiner { kiri, operator, kanan } => {
                let val_kiri = self.evaluasi_ekspresi(kiri)?;
                let val_kanan = self.evaluasi_ekspresi(kanan)?;

                match (val_kiri, operator.as_str(), val_kanan) {
                    (Nilai::Angka(a), "+", Nilai::Angka(b)) => Ok(Nilai::Angka(a + b)),
                    (Nilai::Angka(a), "-", Nilai::Angka(b)) => Ok(Nilai::Angka(a - b)),
                    (Nilai::Angka(a), "*", Nilai::Angka(b)) => Ok(Nilai::Angka(a * b)),
                    (Nilai::Angka(a), "/", Nilai::Angka(b)) => Ok(Nilai::Angka(a / b)),
                    (Nilai::Teks(a), "+", b) => Ok(Nilai::Teks(format!("{}{}", a, b))),
                    (a, "+", Nilai::Teks(b)) => Ok(Nilai::Teks(format!("{}{}", a, b))),
                    (Nilai::Angka(a), "==", Nilai::Angka(b)) => Ok(Nilai::Logika(a == b)),
                    (Nilai::Angka(a), ">=", Nilai::Angka(b)) => Ok(Nilai::Logika(a >= b)),
                    (Nilai::Angka(a), "<=", Nilai::Angka(b)) => Ok(Nilai::Logika(a <= b)),
                    (Nilai::Angka(a), ">", Nilai::Angka(b)) => Ok(Nilai::Logika(a > b)),
                    (Nilai::Angka(a), "<", Nilai::Angka(b)) => Ok(Nilai::Logika(a < b)),
                    _ => Err(format!("Operasi '{}' tidak valid untuk tipe data terkait", operator)),
                }
            }
            _ => Ok(Nilai::Kosong),
        }
    }
}
