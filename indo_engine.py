import sys
import re
import subprocess

# -----------------------------------------------------------------------------
# 1. TIPE TOKEN (BAHASA INDONESIA SEPENUHNYA)
# -----------------------------------------------------------------------------
TIPE_TOKEN = [
    ('KOMENTAR',          r'#.*|//.*'),
    ('ANGKA',            r'\d+(\.\d+)?'),
    ('TEKS',             r'"[^"]*"'),
    ('BENAR',            r'\bBENAR\b|\bbenar\b'),
    ('SALAH',            r'\bSALAH\b|\bsalah\b'),
    ('BUAT',             r'\bBUAT\b|\bbuat\b'),
    ('UBAH',             r'\bUBAH\b|\bubah\b'),
    ('TETAPKAN',         r'\bTETAPKAN\b|\btetapkan\b'),
    ('HAPUS',            r'\bHAPUS\b|\bhapus\b'),
    ('TAMPILKAN',        r'\bTAMPILKAN\b|\btampilkan\b'),
    ('PANGGIL',          r'\bPANGGIL\b|\bpanggil\b'),
    ('DENGAN',           r'\bDENGAN\b|\bdengan\b'),
    ('JIKA_TIDAK',       r'\bJIKA_TIDAK\b|\bjika_tidak\b'),
    ('JIKA',             r'\bJIKA\b|\bjika\b'),
    ('MAKA',             r'\bMAKA\b|\bmaka\b'),
    ('ULANGI',           r'\bULANGI\b|\bulangi\b'),
    ('KALI',             r'\bKALI\b|\bkali\b'),
    ('FUNGSI',           r'\bFUNGSI\b|\bfungsi\b'),
    ('KEMBALIKAN',       r'\bKEMBALIKAN\b|\bkembalikan\b'),
    ('AKHIR',            r'\bAKHIR\b|\bakhir\b'),
    ('MULAI',            r'\bMULAI\b|\bmulai\b'),
    ('PROGRAM',          r'\bPROGRAM\b|\bprogram\b'),
    ('IDENTITAS',        r'[a-zA-Z_][a-zA-Z0-9_.]*'),
    ('OP_PERBANDINGAN',  r'==|!=|>=|<=|>|<'),
    ('OP_ARITMATIKA',    r'[+\-*/]'),
    ('SAMA_DENGAN',      r'='),
    ('KURUNG_BUKA',      r'\('),
    ('KURUNG_TUTUP',     r'\)'),
    ('KOMA',             r','),
    ('BARIS_BARU',       r'\n'),
    ('LEWATI',           r'[ \t\r]+'),
    ('TIDAK_COCOK',      r'.'),
]

class Token:
    def __init__(self, tipe, nilai, baris):
        self.tipe = tipe
        self.nilai = nilai
        self.baris = baris

    def __repr__(self):
        return f"Token({self.tipe}, {repr(self.nilai)})"

def analisis_leksikal(kode_sumber):
    daftar_token = []
    nomor_baris = 1
    pola_regex = '|'.join(f'(?P<{nama}>{pattern})' for nama, pattern in TIPE_TOKEN)
    
    for mo in re.finditer(pola_regex, kode_sumber):
        jenis = mo.lastgroup
        nilai = mo.group()
        
        if jenis == 'BARIS_BARU':
            nomor_baris += 1
            continue
        elif jenis in ('LEWATI', 'KOMENTAR'):
            continue
        elif jenis == 'ANGKA':
            nilai = float(nilai) if '.' in nilai else int(nilai)
        elif jenis == 'TEKS':
            nilai = nilai[1:-1]
        elif jenis == 'BENAR':
            nilai = True
        elif jenis == 'SALAH':
            nilai = False
        elif jenis == 'TIDAK_COCOK':
            raise SyntaxError(f"Baris {nomor_baris}: Karakter tidak dikenal '{nilai}'")
        
        daftar_token.append(Token(jenis, nilai, nomor_baris))
    return daftar_token

# -----------------------------------------------------------------------------
# 2. STRUKTUR NODE AST
# -----------------------------------------------------------------------------
class NodeAST: pass

class NodeDeklarasiVar(NodeAST):
    def __init__(self, nama, ekspresi): self.nama, self.ekspresi = nama, ekspresi

class NodePenugasanVar(NodeAST):
    def __init__(self, nama, ekspresi): self.nama, self.ekspresi = nama, ekspresi

class NodeHapusVar(NodeAST):
    def __init__(self, nama): self.nama = nama

class NodeCetak(NodeAST):
    def __init__(self, ekspresi): self.ekspresi = ekspresi

class NodePanggilProgram(NodeAST):
    def __init__(self, nama_program, argumen=None):
        self.nama_program = nama_program
        self.argumen = argumen

class NodePercabangan(NodeAST):
    def __init__(self, kondisi, blok_jika, blok_tidak=None):
        self.kondisi = kondisi
        self.blok_jika = blok_jika
        self.blok_tidak = blok_tidak

class NodePerulangan(NodeAST):
    def __init__(self, jumlah_ekspresi, isi_blok):
        self.jumlah_ekspresi = jumlah_ekspresi
        self.isi_blok = isi_blok

class NodeDeklarasiFungsi(NodeAST):
    def __init__(self, nama, parameter, isi_blok):
        self.nama, self.parameter, self.isi_blok = nama, parameter, isi_blok

class NodePanggilanFungsi(NodeAST):
    def __init__(self, nama, argumen):
        self.nama, self.argumen = nama, argumen

class NodeKembalikan(NodeAST):
    def __init__(self, ekspresi): self.ekspresi = ekspresi

class NodeOperasiBiner(NodeAST):
    def __init__(self, kiri, operator, kanan):
        self.kiri, self.operator, self.kanan = kiri, operator, kanan

class NodeLiteral(NodeAST):
    def __init__(self, nilai): self.nilai = nilai

class NodeReferensiVar(NodeAST):
    def __init__(self, nama): self.nama = nama

# -----------------------------------------------------------------------------
# 3. PARSER
# -----------------------------------------------------------------------------
class Parser:
    def __init__(self, daftar_token):
        self.tokens = daftar_token
        self.posisi = 0

    def intip(self):
        return self.tokens[self.posisi] if self.posisi < len(self.tokens) else Token('AKHIR_FILE', '', -1)

    def ambil(self, tipe_diharapkan=None):
        tok = self.intip()
        if tipe_diharapkan and tok.tipe != tipe_diharapkan:
            raise SyntaxError(f"Baris {tok.baris}: Diharapkan token '{tipe_diharapkan}', ditemukan '{tok.tipe}' ({tok.nilai})")
        self.posisi += 1
        return tok

    def parse(self):
        pernyataan = []
        if self.intip().tipe == 'MULAI':
            self.ambil('MULAI')
            if self.intip().tipe == 'PROGRAM':
                self.ambil('PROGRAM')

        while self.intip().tipe != 'AKHIR_FILE' and self.intip().tipe != 'AKHIR':
            node = self.parse_pernyataan()
            if node:
                pernyataan.append(node)

        if self.intip().tipe == 'AKHIR':
            self.ambil('AKHIR')
            if self.intip().tipe == 'PROGRAM':
                self.ambil('PROGRAM')

        return pernyataan

    def parse_pernyataan(self):
        tok = self.intip()

        if tok.tipe == 'BUAT':
            self.ambil('BUAT')
            nama = self.ambil('IDENTITAS').nilai
            self.ambil('SAMA_DENGAN')
            expr = self.parse_ekspresi()
            return NodeDeklarasiVar(nama, expr)

        elif tok.tipe in ('UBAH', 'TETAPKAN'):
            self.ambil(tok.tipe)
            nama = self.ambil('IDENTITAS').nilai
            self.ambil('SAMA_DENGAN')
            expr = self.parse_ekspresi()
            return NodePenugasanVar(nama, expr)

        elif tok.tipe == 'HAPUS':
            self.ambil('HAPUS')
            nama = self.ambil('IDENTITAS').nilai
            return NodeHapusVar(nama)

        elif tok.tipe == 'TAMPILKAN':
            self.ambil('TAMPILKAN')
            expr = self.parse_ekspresi()
            return NodeCetak(expr)

        elif tok.tipe == 'PANGGIL':
            self.ambil('PANGGIL')
            nama_prog = self.parse_ekspresi()
            argumen = None
            if self.intip().tipe == 'DENGAN':
                self.ambil('DENGAN')
                argumen = self.parse_ekspresi()
            return NodePanggilProgram(nama_prog, argumen)

        elif tok.tipe == 'JIKA':
            self.ambil('JIKA')
            kondisi = self.parse_ekspresi()
            if self.intip().tipe == 'MAKA':
                self.ambil('MAKA')
            
            blok_jika = []
            blok_tidak = []
            
            while self.intip().tipe not in ('AKHIR_FILE', 'AKHIR', 'JIKA_TIDAK'):
                blok_jika.append(self.parse_pernyataan())

            if self.intip().tipe == 'JIKA_TIDAK':
                self.ambil('JIKA_TIDAK')
                while self.intip().tipe not in ('AKHIR_FILE', 'AKHIR'):
                    blok_tidak.append(self.parse_pernyataan())

            self.ambil('AKHIR')
            if self.intip().tipe == 'JIKA':
                self.ambil('JIKA')
            return NodePercabangan(kondisi, blok_jika, blok_tidak)

        elif tok.tipe == 'ULANGI':
            self.ambil('ULANGI')
            jumlah = self.parse_ekspresi()
            self.ambil('KALI')
            isi_blok = []
            while self.intip().tipe not in ('AKHIR_FILE', 'AKHIR'):
                isi_blok.append(self.parse_pernyataan())
            self.ambil('AKHIR')
            if self.intip().tipe == 'ULANGI':
                self.ambil('ULANGI')
            return NodePerulangan(jumlah, isi_blok)

        elif tok.tipe == 'FUNGSI':
            self.ambil('FUNGSI')
            nama = self.ambil('IDENTITAS').nilai
            self.ambil('KURUNG_BUKA')
            parameter = []
            if self.intip().tipe == 'IDENTITAS':
                parameter.append(self.ambil('IDENTITAS').nilai)
                while self.intip().tipe == 'KOMA':
                    self.ambil('KOMA')
                    parameter.append(self.ambil('IDENTITAS').nilai)
            self.ambil('KURUNG_TUTUP')
            
            isi_blok = []
            while self.intip().tipe not in ('AKHIR_FILE', 'AKHIR'):
                isi_blok.append(self.parse_pernyataan())
            self.ambil('AKHIR')
            if self.intip().tipe == 'FUNGSI':
                self.ambil('FUNGSI')
            return NodeDeklarasiFungsi(nama, parameter, isi_blok)

        elif tok.tipe == 'KEMBALIKAN':
            self.ambil('KEMBALIKAN')
            expr = self.parse_ekspresi()
            return NodeKembalikan(expr)

        return self.parse_ekspresi()

    def parse_ekspresi(self):
        kiri = self.parse_suku()
        while self.intip().tipe in ('OP_PERBANDINGAN', 'OP_ARITMATIKA') and self.intip().nilai in ('==', '!=', '>', '<', '>=', '<='):
            op = self.ambil().nilai
            kanan = self.parse_suku()
            kiri = NodeOperasiBiner(kiri, op, kanan)
        return kiri

    def parse_suku(self):
        kiri = self.parse_faktor()
        while self.intip().tipe == 'OP_ARITMATIKA' and self.intip().nilai in ('+', '-'):
            op = self.ambil().nilai
            kanan = self.parse_faktor()
            kiri = NodeOperasiBiner(kiri, op, kanan)
        return kiri

    def parse_faktor(self):
        kiri = self.parse_primer()
        while self.intip().tipe == 'OP_ARITMATIKA' and self.intip().nilai in ('*', '/'):
            op = self.ambil().nilai
            kanan = self.parse_primer()
            kiri = NodeOperasiBiner(kiri, op, kanan)
        return kiri

    def parse_primer(self):
        tok = self.intip()
        if tok.tipe in ('ANGKA', 'TEKS', 'BENAR', 'SALAH'):
            self.ambil()
            return NodeLiteral(tok.nilai)
        elif tok.tipe == 'IDENTITAS':
            self.ambil()
            if self.intip().tipe == 'KURUNG_BUKA':
                self.ambil('KURUNG_BUKA')
                argumen = []
                if self.intip().tipe != 'KURUNG_TUTUP':
                    argumen.append(self.parse_ekspresi())
                    while self.intip().tipe == 'KOMA':
                        self.ambil('KOMA')
                        argumen.append(self.parse_ekspresi())
                self.ambil('KURUNG_TUTUP')
                return NodePanggilanFungsi(tok.nilai, argumen)
            return NodeReferensiVar(tok.nilai)
        elif tok.tipe == 'KURUNG_BUKA':
            self.ambil('KURUNG_BUKA')
            expr = self.parse_ekspresi()
            self.ambil('KURUNG_TUTUP')
            return expr
        raise SyntaxError(f"Baris {tok.baris}: Ekspresi tidak valid di dekat '{tok.nilai}'")

# -----------------------------------------------------------------------------
# 4. LINGKUNGAN & INTERPRETER
# -----------------------------------------------------------------------------
class SinyalKembalikan(Exception):
    def __init__(self, nilai): self.nilai = nilai

class Lingkungan:
    def __init__(self, induk=None):
        self.nilai = {}
        self.fungsi = {}
        self.induk = induk

    def cari_var(self, nama):
        if nama in self.nilai: return self.nilai[nama]
        if self.induk: return self.induk.cari_var(nama)
        raise NameError(f"Variabel '{nama}' tidak ditemukan.")

    def ubah_var(self, nama, val):
        if nama in self.nilai:
            self.nilai[nama] = val
        elif self.induk and self.induk.ada_var(nama):
            self.induk.ubah_var(nama, val)
        else:
            self.nilai[nama] = val

    def hapus_var(self, nama):
        if nama in self.nilai:
            del self.nilai[nama]
        elif self.induk:
            self.induk.hapus_var(nama)
        else:
            raise NameError(f"Variabel '{nama}' tidak dapat dihapus karena tidak ditemukan.")

    def ada_var(self, nama):
        if nama in self.nilai: return True
        if self.induk: return self.induk.ada_var(nama)
        return False

    def simpan_fungsi(self, nama, node):
        self.fungsi[nama] = node

    def cari_fungsi(self, nama):
        if nama in self.fungsi: return self.fungsi[nama]
        if self.induk: return self.induk.cari_fungsi(nama)
        raise NameError(f"Fungsi '{nama}' tidak ditemukan.")

class Interpreter:
    def __init__(self):
        self.lingkungan_global = Lingkungan()

    def evaluasi(self, node, env):
        if isinstance(node, NodeLiteral):
            return node.nilai

        elif isinstance(node, NodeReferensiVar):
            return env.cari_var(node.nama)

        elif isinstance(node, NodeDeklarasiVar):
            val = self.evaluasi(node.ekspresi, env)
            env.nilai[node.nama] = val
            return val

        elif isinstance(node, NodePenugasanVar):
            val = self.evaluasi(node.ekspresi, env)
            env.ubah_var(node.nama, val)
            return val

        elif isinstance(node, NodeHapusVar):
            env.hapus_var(node.nama)
            return None

        elif isinstance(node, NodeCetak):
            val = self.evaluasi(node.ekspresi, env)
            print(val)
            return val

        elif isinstance(node, NodePanggilProgram):
            prog = str(self.evaluasi(node.nama_program, env))
            arg = str(self.evaluasi(node.argumen, env)) if node.argumen else ""
            perintah = f"{prog} {arg}".strip()
            print(f"[INDOCODE SYSTEM]: Menjalankan perintah '{perintah}'...")
            subprocess.run(perintah, shell=True)
            return None

        elif isinstance(node, NodeOperasiBiner):
            kiri = self.evaluasi(node.kiri, env)
            kanan = self.evaluasi(node.kanan, env)
            op = node.operator
            if op == '+': 
                return str(kiri) + str(kanan) if isinstance(kiri, str) or isinstance(kanan, str) else kiri + kanan
            elif op == '-': return kiri - kanan
            elif op == '*': return kiri * kanan
            elif op == '/': return kiri / kanan
            elif op == '==': return kiri == kanan
            elif op == '!=': return kiri != kanan
            elif op == '>': return kiri > kanan
            elif op == '<': return kiri < kanan
            elif op == '>=': return kiri >= kanan
            elif op == '<=': return kiri <= kanan

        elif isinstance(node, NodePercabangan):
            kondisi = self.evaluasi(node.kondisi, env)
            if kondisi:
                for statement in node.blok_jika: self.evaluasi(statement, env)
            elif node.blok_tidak:
                for statement in node.blok_tidak: self.evaluasi(statement, env)

        elif isinstance(node, NodePerulangan):
            jumlah = self.evaluasi(node.jumlah_ekspresi, env)
            for _ in range(int(jumlah)):
                for statement in node.isi_blok: self.evaluasi(statement, env)

        elif isinstance(node, NodeDeklarasiFungsi):
            env.simpan_fungsi(node.nama, node)

        elif isinstance(node, NodePanggilanFungsi):
            node_fungsi = env.cari_fungsi(node.nama)
            env_lokal = Lingkungan(self.lingkungan_global)
            for param, expr_arg in zip(node_fungsi.parameter, node.argumen):
                env_lokal.nilai[param] = self.evaluasi(expr_arg, env)
            try:
                for statement in node_fungsi.isi_blok:
                    self.evaluasi(statement, env_lokal)
            except SinyalKembalikan as ret:
                return ret.nilai
            return None

        elif isinstance(node, NodeKembalikan):
            val = self.evaluasi(node.ekspresi, env)
            raise SinyalKembalikan(val)

# -----------------------------------------------------------------------------
# 5. CLI RUNNER
# -----------------------------------------------------------------------------
if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("Penggunaan: python indo_engine.py <nama_file.indo>")
        sys.exit(1)

    with open(sys.argv[1], 'r', encoding='utf-8') as f:
        kode_sumber = f.read()

    try:
        tokens = analisis_leksikal(kode_sumber)
        parser = Parser(tokens)
        ast = parser.parse()
        interpreter = Interpreter()
        for node in ast:
            interpreter.evaluasi(node, interpreter.lingkungan_global)
        print("\nProgram berhasil dijalankan.")
    except Exception as e:
        print(f"\n[KESALAHAN INDOCODE]: {e}")