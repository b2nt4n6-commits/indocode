use crate::ast::{Ekspresi, NilaiLiteral, NodeAST};
use crate::token::{TipeToken, Token};

pub struct Parser {
    tokens: Vec<Token>,
    posisi: usize,
}

impl Parser {
    pub fn baru(tokens: Vec<Token>) -> Self {
        Self { tokens, posisi: 0 }
    }

    fn intip(&self) -> &Token {
        self.tokens.get(self.posisi).unwrap_or_else(|| {
            self.tokens.last().unwrap()
        })
    }

    fn ambil(&mut self) -> Token {
        let tok = self.intip().clone();
        if self.posisi < self.tokens.len() {
            self.posisi += 1;
        }
        tok
    }

    fn cocokkan(&mut self, tipe: &TipeToken) -> bool {
        if &self.intip().tipe == tipe {
            self.ambil();
            true
        } else {
            false
        }
    }

    fn pastikan(&mut self, tipe: &TipeToken, pesan: &str) -> Result<Token, String> {
        let tok = self.intip().clone();
        if &tok.tipe == tipe {
            Ok(self.ambil())
        } else {
            Err(format!("Baris {}: {} (ditemukan '{:?}')", tok.baris, pesan, tok.tipe))
        }
    }

    pub fn parse(&mut self) -> Result<Vec<NodeAST>, String> {
        let mut daftar_node = Vec::new();

        if self.intip().tipe == TipeToken::Mulai {
            self.ambil();
            if self.intip().tipe == TipeToken::Program {
                self.ambil();
            }
        }

        while self.intip().tipe != TipeToken::Eof && self.intip().tipe != TipeToken::Akhir {
            let node = self.parse_pernyataan()?;
            daftar_node.push(node);
        }

        if self.intip().tipe == TipeToken::Akhir {
            self.ambil();
            if self.intip().tipe == TipeToken::Program {
                self.ambil();
            }
        }

        Ok(daftar_node)
    }

    fn parse_pernyataan(&mut self) -> Result<NodeAST, String> {
        let tok = self.intip().clone();

        match &tok.tipe {
            TipeToken::Buat => {
                self.ambil();
                let nama_tok = self.ambil();
                let nama = match nama_tok.tipe {
                    TipeToken::Identitas(ref n) => n.clone(),
                    _ => return Err(format!("Baris {}: Diharapkan nama variabel setelah BUAT", tok.baris)),
                };
                self.pastikan(&TipeToken::SamaDengan, "Diharapkan '=' setelah nama variabel")?;
                let expr = self.parse_ekspresi()?;
                Ok(NodeAST::DeklarasiVar { nama, ekspresi: expr })
            }

            TipeToken::Ubah | TipeToken::Tetapkan => {
                self.ambil();
                let nama_tok = self.ambil();
                let nama = match nama_tok.tipe {
                    TipeToken::Identitas(ref n) => n.clone(),
                    _ => return Err(format!("Baris {}: Diharapkan nama variabel setelah UBAH/TETAPKAN", tok.baris)),
                };
                self.pastikan(&TipeToken::SamaDengan, "Diharapkan '=' setelah nama variabel")?;
                let expr = self.parse_ekspresi()?;
                Ok(NodeAST::PenugasanVar { nama, ekspresi: expr })
            }

            TipeToken::Hapus => {
                self.ambil();
                let nama_tok = self.ambil();
                let nama = match nama_tok.tipe {
                    TipeToken::Identitas(ref n) => n.clone(),
                    _ => return Err(format!("Baris {}: Diharapkan nama variabel setelah HAPUS", tok.baris)),
                };
                Ok(NodeAST::HapusVar { nama })
            }

            TipeToken::Tampilkan => {
                self.ambil();
                let expr = self.parse_ekspresi()?;
                Ok(NodeAST::Cetak { ekspresi: expr })
            }

            TipeToken::Panggil => {
                self.ambil();
                let nama_prog = self.parse_ekspresi()?;
                let mut argumen = None;
                if self.intip().tipe == TipeToken::Dengan {
                    self.ambil();
                    argumen = Some(self.parse_ekspresi()?);
                }
                Ok(NodeAST::PanggilProgram { nama_program: nama_prog, argumen })
            }

            TipeToken::Jika => {
                self.ambil();
                let kondisi = self.parse_ekspresi()?;
                if self.intip().tipe == TipeToken::Maka {
                    self.ambil();
                }

                let mut blok_jika = Vec::new();
                let mut blok_tidak = Vec::new();

                while self.intip().tipe != TipeToken::Eof
                    && self.intip().tipe != TipeToken::Akhir
                    && self.intip().tipe != TipeToken::JikaTidak
                {
                    blok_jika.push(self.parse_pernyataan()?);
                }

                if self.cocokkan(&TipeToken::JikaTidak) {
                    while self.intip().tipe != TipeToken::Eof && self.intip().tipe != TipeToken::Akhir {
                        blok_tidak.push(self.parse_pernyataan()?);
                    }
                }

                self.pastikan(&TipeToken::Akhir, "Diharapkan AKHIR untuk menutup JIKA")?;
                self.cocokkan(&TipeToken::Jika);

                Ok(NodeAST::Percabangan {
                    kondisi,
                    blok_jika,
                    blok_tidak,
                })
            }

            TipeToken::Ulangi => {
                self.ambil();
                let jumlah_expr = self.parse_ekspresi()?;
                self.pastikan(&TipeToken::Kali, "Diharapkan KALI setelah jumlah perulangan")?;

                let mut isi_blok = Vec::new();
                while self.intip().tipe != TipeToken::Eof && self.intip().tipe != TipeToken::Akhir {
                    isi_blok.push(self.parse_pernyataan()?);
                }

                self.pastikan(&TipeToken::Akhir, "Diharapkan AKHIR untuk menutup ULANGI")?;
                self.cocokkan(&TipeToken::Ulangi);

                Ok(NodeAST::Perulangan {
                    jumlah_ekspresi: jumlah_expr,
                    isi_blok,
                })
            }

            TipeToken::Fungsi => {
                self.ambil();
                let nama_tok = self.ambil();
                let nama = match nama_tok.tipe {
                    TipeToken::Identitas(ref n) => n.clone(),
                    _ => return Err(format!("Baris {}: Diharapkan nama fungsi setelah FUNGSI", tok.baris)),
                };

                self.pastikan(&TipeToken::KurungBuka, "Diharapkan '(' setelah nama fungsi")?;
                let mut parameter = Vec::new();

                if let TipeToken::Identitas(p) = &self.intip().tipe {
                    parameter.push(p.clone());
                    self.ambil();
                    while self.cocokkan(&TipeToken::Koma) {
                        if let TipeToken::Identitas(p_next) = &self.intip().tipe {
                            parameter.push(p_next.clone());
                            self.ambil();
                        }
                    }
                }
                self.pastikan(&TipeToken::KurungTutup, "Diharapkan ')' setelah daftar parameter")?;

                let mut isi_blok = Vec::new();
                while self.intip().tipe != TipeToken::Eof && self.intip().tipe != TipeToken::Akhir {
                    isi_blok.push(self.parse_pernyataan()?);
                }

                self.pastikan(&TipeToken::Akhir, "Diharapkan AKHIR untuk menutup FUNGSI")?;
                self.cocokkan(&TipeToken::Fungsi);

                Ok(NodeAST::DeklarasiFungsi {
                    nama,
                    parameter,
                    isi_blok,
                })
            }

            TipeToken::Kembalikan => {
                self.ambil();
                let expr = self.parse_ekspresi()?;
                Ok(NodeAST::Kembalikan { ekspresi: expr })
            }

            _ => {
                let expr = self.parse_ekspresi()?;
                Ok(NodeAST::EkspresiStmt(expr))
            }
        }
    }

    fn parse_ekspresi(&mut self) -> Result<Ekspresi, String> {
        let mut kiri = self.parse_suku()?;
        while let TipeToken::OpPerbandingan(ref op) = self.intip().tipe.clone() {
            let op_str = op.clone();
            self.ambil();
            let kanan = self.parse_suku()?;
            kiri = Ekspresi::OperasiBiner {
                kiri: Box::new(kiri),
                operator: op_str,
                kanan: Box::new(kanan),
            };
        }
        Ok(kiri)
    }

    fn parse_suku(&mut self) -> Result<Ekspresi, String> {
        let mut kiri = self.parse_faktor()?;
        while let TipeToken::OpAritmatika(op) = self.intip().tipe {
            if op == '+' || op == '-' {
                self.ambil();
                let kanan = self.parse_faktor()?;
                kiri = Ekspresi::OperasiBiner {
                    kiri: Box::new(kiri),
                    operator: op.to_string(),
                    kanan: Box::new(kanan),
                };
            } else {
                break;
            }
        }
        Ok(kiri)
    }

    fn parse_faktor(&mut self) -> Result<Ekspresi, String> {
        let mut kiri = self.parse_primer()?;
        while let TipeToken::OpAritmatika(op) = self.intip().tipe {
            if op == '*' || op == '/' {
                self.ambil();
                let kanan = self.parse_primer()?;
                kiri = Ekspresi::OperasiBiner {
                    kiri: Box::new(kiri),
                    operator: op.to_string(),
                    kanan: Box::new(kanan),
                };
            } else {
                break;
            }
        }
        Ok(kiri)
    }

    fn parse_primer(&mut self) -> Result<Ekspresi, String> {
        let tok = self.ambil();
        match tok.tipe {
            TipeToken::Angka(val) => Ok(Ekspresi::Literal(NilaiLiteral::Angka(val))),
            TipeToken::Teks(val) => Ok(Ekspresi::Literal(NilaiLiteral::Teks(val))),
            TipeToken::Benar => Ok(Ekspresi::Literal(NilaiLiteral::Logika(true))),
            TipeToken::Salah => Ok(Ekspresi::Literal(NilaiLiteral::Logika(false))),
            TipeToken::Identitas(ref nama) => {
                let nama_str = nama.clone();
                if self.intip().tipe == TipeToken::KurungBuka {
                    self.ambil();
                    let mut argumen = Vec::new();
                    if self.intip().tipe != TipeToken::KurungTutup {
                        argumen.push(self.parse_ekspresi()?);
                        while self.cocokkan(&TipeToken::Koma) {
                            argumen.push(self.parse_ekspresi()?);
                        }
                    }
                    self.pastikan(&TipeToken::KurungTutup, "Diharapkan ')' setelah argumen fungsi")?;
                    Ok(Ekspresi::PanggilanFungsi {
                        nama: nama_str,
                        argumen,
                    })
                } else {
                    Ok(Ekspresi::ReferensiVar(nama_str))
                }
            }
            TipeToken::KurungBuka => {
                let expr = self.parse_ekspresi()?;
                self.pastikan(&TipeToken::KurungTutup, "Diharapkan ')' setelah ekspresi")?;
                Ok(expr)
            }
            _ => Err(format!("Baris {}: Token tidak diharapkan '{:?}'", tok.baris, tok.tipe)),
        }
    }
}
