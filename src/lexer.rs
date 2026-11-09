use crate::token::{TipeToken, Token};

pub struct Lexer {
    input: Vec<char>,
    posisi: usize,
    baris: usize,
}

impl Lexer {
    pub fn baru(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            posisi: 0,
            baris: 1,
        }
    }

    pub fn analisis_leksikal(&mut self) -> Result<Vec<Token>, String> {
        let mut daftar_token = Vec::new();

        while self.posisi < self.input.len() {
            let c = self.input[self.posisi];

            match c {
                '\n' => {
                    self.baris += 1;
                    self.posisi += 1;
                }
                ' ' | '\r' | '\t' => {
                    self.posisi += 1;
                }
                '#' => {
                    while self.posisi < self.input.len() && self.input[self.posisi] != '\n' {
                        self.posisi += 1;
                    }
                }
                '/' => {
                    if self.intip(1) == Some('/') {
                        while self.posisi < self.input.len() && self.input[self.posisi] != '\n' {
                            self.posisi += 1;
                        }
                    } else {
                        daftar_token.push(Token {
                            tipe: TipeToken::OpAritmatika('/'),
                            nilai: "/".to_string(),
                            baris: self.baris,
                        });
                        self.posisi += 1;
                    }
                }
                '=' => {
                    if self.intip(1) == Some('=') {
                        daftar_token.push(Token {
                            tipe: TipeToken::OpPerbandingan("==".to_string()),
                            nilai: "==".to_string(),
                            baris: self.baris,
                        });
                        self.posisi += 2;
                    } else {
                        daftar_token.push(Token {
                            tipe: TipeToken::SamaDengan,
                            nilai: "=".to_string(),
                            baris: self.baris,
                        });
                        self.posisi += 1;
                    }
                }
                '!' => {
                    if self.intip(1) == Some('=') {
                        daftar_token.push(Token {
                            tipe: TipeToken::OpPerbandingan("!=".to_string()),
                            nilai: "!=".to_string(),
                            baris: self.baris,
                        });
                        self.posisi += 2;
                    } else {
                        return Err(format!("Baris {}: Karakter tidak dikenal '!'", self.baris));
                    }
                }
                '>' => {
                    if self.intip(1) == Some('=') {
                        daftar_token.push(Token {
                            tipe: TipeToken::OpPerbandingan(">=".to_string()),
                            nilai: ">=".to_string(),
                            baris: self.baris,
                        });
                        self.posisi += 2;
                    } else {
                        daftar_token.push(Token {
                            tipe: TipeToken::OpPerbandingan(">".to_string()),
                            nilai: ">".to_string(),
                            baris: self.baris,
                        });
                        self.posisi += 1;
                    }
                }
                '<' => {
                    if self.intip(1) == Some('=') {
                        daftar_token.push(Token {
                            tipe: TipeToken::OpPerbandingan("<=".to_string()),
                            nilai: "<=".to_string(),
                            baris: self.baris,
                        });
                        self.posisi += 2;
                    } else {
                        daftar_token.push(Token {
                            tipe: TipeToken::OpPerbandingan("<".to_string()),
                            nilai: "<".to_string(),
                            baris: self.baris,
                        });
                        self.posisi += 1;
                    }
                }
                '+' | '-' | '*' => {
                    daftar_token.push(Token {
                        tipe: TipeToken::OpAritmatika(c),
                        nilai: c.to_string(),
                        baris: self.baris,
                    });
                    self.posisi += 1;
                }
                '(' => {
                    daftar_token.push(Token {
                        tipe: TipeToken::KurungBuka,
                        nilai: "(".to_string(),
                        baris: self.baris,
                    });
                    self.posisi += 1;
                }
                ')' => {
                    daftar_token.push(Token {
                        tipe: TipeToken::KurungTutup,
                        nilai: ")".to_string(),
                        baris: self.baris,
                    });
                    self.posisi += 1;
                }
                '[' => {
                    daftar_token.push(Token {
                        tipe: TipeToken::KurungSikuBuka,
                        nilai: "[".to_string(),
                        baris: self.baris,
                    });
                    self.posisi += 1;
                }
                ']' => {
                    daftar_token.push(Token {
                        tipe: TipeToken::KurungSikuTutup,
                        nilai: "]".to_string(),
                        baris: self.baris,
                    });
                    self.posisi += 1;
                }
                '{' => {
                    daftar_token.push(Token {
                        tipe: TipeToken::KurungKurawalBuka,
                        nilai: "{".to_string(),
                        baris: self.baris,
                    });
                    self.posisi += 1;
                }
                '}' => {
                    daftar_token.push(Token {
                        tipe: TipeToken::KurungKurawalTutup,
                        nilai: "}".to_string(),
                        baris: self.baris,
                    });
                    self.posisi += 1;
                }
                ',' => {
                    daftar_token.push(Token {
                        tipe: TipeToken::Koma,
                        nilai: ",".to_string(),
                        baris: self.baris,
                    });
                    self.posisi += 1;
                }
                ';' => {
                    daftar_token.push(Token {
                        tipe: TipeToken::TitikKoma,
                        nilai: ";".to_string(),
                        baris: self.baris,
                    });
                    self.posisi += 1;
                }
                '.' => {
                    daftar_token.push(Token {
                        tipe: TipeToken::Titik,
                        nilai: ".".to_string(),
                        baris: self.baris,
                    });
                    self.posisi += 1;
                }
                '"' => {
                    let teks = self.baca_teks()?;
                    daftar_token.push(Token {
                        tipe: TipeToken::Teks(teks.clone()),
                        nilai: teks,
                        baris: self.baris,
                    });
                }
                _ if c.is_ascii_digit() => {
                    let angka_str = self.baca_angka();
                    let angka_val: f64 = angka_str.parse().map_err(|_| {
                        format!("Baris {}: Format angka tidak valid '{}'", self.baris, angka_str)
                    })?;
                    daftar_token.push(Token {
                        tipe: TipeToken::Angka(angka_val),
                        nilai: angka_str,
                        baris: self.baris,
                    });
                }
                _ if c.is_alphabetic() || c == '_' => {
                    let ident = self.baca_identitas();
                    let tipe = match ident.to_uppercase().as_str() {
                        "MULAI" => TipeToken::Mulai,
                        "PROGRAM" => TipeToken::Program,
                        "AKHIR" => TipeToken::Akhir,
                        "BUAT" => TipeToken::Buat,
                        "UBAH" => TipeToken::Ubah,
                        "TETAPKAN" => TipeToken::Tetapkan,
                        "HAPUS" => TipeToken::Hapus,
                        "TAMPILKAN" => TipeToken::Tampilkan,
                        "PANGGIL" => TipeToken::Panggil,
                        "DENGAN" => TipeToken::Dengan,
                        "JIKA" => TipeToken::Jika,
                        "MAKA" => TipeToken::Maka,
                        "JIKA_TIDAK" => TipeToken::JikaTidak,
                        "ULANGI" => TipeToken::Ulangi,
                        "KALI" => TipeToken::Kali,
                        "FUNGSI" => TipeToken::Fungsi,
                        "KEMBALIKAN" => TipeToken::Kembalikan,
                        "BENAR" => TipeToken::Benar,
                        "SALAH" => TipeToken::Salah,
                        _ => TipeToken::Identitas(ident.clone()),
                    };
                    daftar_token.push(Token {
                        tipe,
                        nilai: ident,
                        baris: self.baris,
                    });
                }
                _ => {
                    return Err(format!("Baris {}: Karakter tidak dikenal '{}'", self.baris, c));
                }
            }
        }

        daftar_token.push(Token {
            tipe: TipeToken::Eof,
            nilai: "".to_string(),
            baris: self.baris,
        });

        Ok(daftar_token)
    }

    fn intip(&self, offset: usize) -> Option<char> {
        self.input.get(self.posisi + offset).copied()
    }

    fn baca_teks(&mut self) -> Result<String, String> {
        self.posisi += 1; // Lewati tanda kutip pembuka '"'
        let mut hasil = String::new();

        while self.posisi < self.input.len() {
            let c = self.input[self.posisi];

            if c == '\\' {
                // Penanganan karakter escape (\", \n, \t, \\)
                self.posisi += 1;
                if self.posisi >= self.input.len() {
                    return Err(format!("Baris {}: Karakter escape tidak lengkap di akhir file", self.baris));
                }
                let next_c = self.input[self.posisi];
                match next_c {
                    'n' => hasil.push('\n'),
                    't' => hasil.push('\t'),
                    '"' => hasil.push('"'),
                    '\\' => hasil.push('\\'),
                    _ => {
                        hasil.push('\\');
                        hasil.push(next_c);
                    }
                }
            } else if c == '"' {
                self.posisi += 1; // Lewati tanda kutip penutup '"'
                return Ok(hasil);
            } else {
                if c == '\n' {
                    self.baris += 1;
                }
                hasil.push(c);
            }
            self.posisi += 1;
        }

        Err(format!("Baris {}: Teks literal tidak ditutup tanda kutip", self.baris))
    }

    fn baca_angka(&mut self) -> String {
        let mut hasil = String::new();
        let mut ada_titik = false;
        while self.posisi < self.input.len() {
            let c = self.input[self.posisi];
            if c.is_ascii_digit() {
                hasil.push(c);
                self.posisi += 1;
            } else if c == '.' && !ada_titik && self.intip(1).map_or(false, |next| next.is_ascii_digit()) {
                ada_titik = true;
                hasil.push(c);
                self.posisi += 1;
            } else {
                break;
            }
        }
        hasil
    }

    fn baca_identitas(&mut self) -> String {
        let mut hasil = String::new();
        while self.posisi < self.input.len() {
            let c = self.input[self.posisi];
            // Identitas HANYA boleh huruf, angka, dan garis bawah (_)
            if c.is_alphanumeric() || c == '_' {
                hasil.push(c);
                self.posisi += 1;
            } else {
                break;
            }
        }
        hasil
    }
}