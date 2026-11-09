#![allow(dead_code)]

#[derive(Debug, Clone)]
pub enum NilaiLiteral {
    Angka(f64),
    Teks(String),
    Logika(bool),
}

#[derive(Debug, Clone)]
pub enum Ekspresi {
    Literal(NilaiLiteral),
    ReferensiVar(String),
    OperasiBiner {
        kiri: Box<Ekspresi>,
        operator: String,
        kanan: Box<Ekspresi>,
    },
    PanggilanFungsi {
        nama: String,
        argumen: Vec<Ekspresi>,
    },
}

#[derive(Debug, Clone)]
pub enum NodeAST {
    DeklarasiVar {
        nama: String,
        ekspresi: Ekspresi,
    },
    PenugasanVar {
        nama: String,
        ekspresi: Ekspresi,
    },
    HapusVar {
        nama: String,
    },
    Cetak {
        ekspresi: Ekspresi,
    },
    PanggilProgram {
        nama_program: Ekspresi,
        argumen: Option<Ekspresi>,
    },
    Percabangan {
        kondisi: Ekspresi,
        blok_jika: Vec<NodeAST>,
        blok_tidak: Vec<NodeAST>,
    },
    Perulangan {
        jumlah_ekspresi: Ekspresi,
        isi_blok: Vec<NodeAST>,
    },
    DeklarasiFungsi {
        nama: String,
        parameter: Vec<String>,
        isi_blok: Vec<NodeAST>,
    },
    Kembalikan {
        ekspresi: Ekspresi,
    },
    PakaiFile {
        path: String,
    },
    EkspresiStmt(Ekspresi),
}
