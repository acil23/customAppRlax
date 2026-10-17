# Progress Aplikasi Rlax

Dokumen ini mencatat fitur yang sudah tersedia dan pekerjaan yang masih perlu dilakukan. Tanggal pembaruan: 17 Oktober 2026.

## Rlax Notes

**Status:** prototipe awal

### Sudah dibuat

- UI editor teks native menggunakan Rust dan GTK4.
- Menulis dan mengedit teks, dengan kemampuan dasar copy/paste dari widget GTK.
- Membuat catatan baru.
- Membuka dan menyimpan file teks `.txt` dan `.md`.
- Save, Save as, serta pintasan `Ctrl+S`.
- Penanda perubahan belum disimpan pada judul jendela.
- Build release terukur sekitar 318 KiB pada lingkungan pengembangan awal.
- PKGBUILD Arch Linux, ikon hicolor, dan entri launcher `.desktop` telah disiapkan; build paket perlu tag release source terlebih dahulu.

### Belum dibuat / perlu ditingkatkan

- Peringatan sebelum menutup atau mengganti dokumen yang belum disimpan.
- Undo/redo eksplisit dan pintasan standar tambahan.
- Pengujian interaksi UI dan publikasi paket release/AUR.
- Uji instalasi paket pada lingkungan Arch bersih.
- Asosiasi file MIME masih belum tersedia karena aplikasi belum membuka nama file dari argumen launcher.

### Validasi terakhir

- `cargo fmt --check` — berhasil.
- `cargo check` — berhasil.
- `cargo build --release -p rlax-notes` — berhasil.