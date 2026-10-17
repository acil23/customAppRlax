# Rlax Apps

Kumpulan aplikasi desktop ringan untuk Arch Linux dan Hyprland. Setiap aplikasi memiliki source code dan assets sendiri di bawah `apps/`.

## Aplikasi

- **Rlax Notes** — editor teks sederhana untuk menulis, membuka, dan menyimpan `.txt` serta `.md`. Detail progres ada di [PROGRESS.md](PROGRESS.md).

## Instalasi dan menjalankan Rlax Notes

### Instalasi dari paket Arch

Di Arch Linux, setelah source `v0.1.0` dipublikasikan, jalankan:

```sh
git clone https://github.com/acil23/customAppRlax.git
cd customAppRlax
git checkout v0.1.0
cd apps/rlax-notes
makepkg -si
```

Paket memasang binary, ikon launcher, dan entri menu. GTK4 dipasang sebagai dependensi runtime melalui pacman; library GTK sistem tidak disalin ke paket aplikasi. Setelah instalasi, **Rlax Notes** dapat dibuka dari launcher desktop atau terminal:

```sh
rlax-notes
```

Perintah tersebut membangun paket dari source dan memasangnya dengan pacman. Untuk menghapusnya, jalankan `sudo pacman -R rlax-notes`. PKGBUILD berada di `apps/rlax-notes/PKGBUILD`.

### Build dan jalankan dari source

Prasyarat build di Arch Linux: Rust/Cargo, GTK4 development files, dan toolchain build.

```sh
sudo pacman -S --needed base-devel rust gtk4
cargo run --release -p rlax-notes
```

Binary release pada build awal berukuran sekitar 323 KiB; ukuran dapat berbeda menurut versi toolchain dan dependensi. GTK4 tetap merupakan library sistem, bukan disalin ke dalam paket aplikasi.

## Tech stack

- Rust + Cargo workspace
- GTK4 melalui `gtk4-rs`
- Integrasi Hyprland hanya ditambahkan jika fitur aplikasi membutuhkannya

## Struktur proyek

```text
apps/
└── rlax-notes/
    ├── assets/     # logo, ikon, dan aset milik aplikasi ini
    └── src/        # source code aplikasi ini
```

Folder `assets/` disiapkan per aplikasi; aset akan ditambahkan saat diperlukan.