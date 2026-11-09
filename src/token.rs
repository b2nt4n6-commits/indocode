#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq)]
pub enum TipeToken {
    Mulai,
    Program,
    Akhir,
    Buat,
    Ubah,
    Tetapkan,
    Hapus,
    Tampilkan,
    Panggil,
    Dengan,
    Jika,
    Maka,
    JikaTidak,
    Ulangi,
    Kali,
    Fungsi,
    Kembalikan,
    Benar,
    Salah,
    Identitas(String),
    Angka(f64),
    Teks(String),
    OpAritmatika(char),
    OpPerbandingan(String),
    SamaDengan,
    KurungBuka,          // (
    KurungTutup,         // )
    KurungSikuBuka,      // [
    KurungSikuTutup,     // ]
    KurungKurawalBuka,   // {
    KurungKurawalTutup,  // }
    Koma,                // ,
    TitikKoma,           // ;
    Titik,               // .
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub tipe: TipeToken,
    pub nilai: String,
    pub baris: usize,
}