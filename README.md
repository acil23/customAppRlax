# Custom App untuk Hyprland

Aplikasi desktop ringan untuk Arch Linux dan Hyprland, dengan fokus pada fitur yang benar-benar digunakan. Fitur dan nama aplikasi belum ditentukan; dokumen ini menetapkan usulan awal tech stack agar pengembangan berikutnya punya arah yang jelas.

## Tujuan

- Menyediakan hanya fitur yang dipilih, tanpa membawa fitur bawaan yang tidak diperlukan.
- Berjalan sebagai aplikasi native di Linux/Wayland.
- Berintegrasi dengan Hyprland jika fitur yang dibuat memang membutuhkannya.
- Menjaga penggunaan dependensi dan proses latar belakang tetap minimal.

## Usulan tech stack

| Bagian | Teknologi | Alasan |
|---|---|---|
| Bahasa | **Rust** | Menghasilkan aplikasi native, efisien, dan memiliki pemeriksaan keamanan memori saat kompilasi. |
| UI | **GTK4** dengan binding Rust `gtk4-rs` | Toolkit native Linux dengan integrasi Wayland dan aksesibilitas yang baik. |
| Overlay/bar (opsional) | **gtk4-layer-shell** | Dipakai hanya jika aplikasi perlu membuat panel, OSD, atau overlay layer-shell di Wayland. |
| Integrasi Hyprland | **Hyprland IPC** melalui Unix socket | Untuk membaca event atau mengirim perintah Hyprland hanya ketika fitur aplikasi memerlukannya. |
| Konfigurasi | **TOML** | Format teks yang mudah dibaca dan diedit pengguna. |
| Data/serialisasi | **Serde** | Membaca dan menulis konfigurasi secara terstruktur. |
| Build dan dependensi Rust | **Cargo** | Toolchain standar Rust untuk build, pengujian, dan manajemen dependensi. |

### Prinsip pemilihan dependensi

- Hindari Electron dan runtime browser untuk UI sederhana.
- Jangan menambahkan daemon, database, atau framework async sebelum ada kebutuhan nyata.
- Gunakan GTK4 biasa untuk jendela aplikasi; tambahkan `gtk4-layer-shell` hanya untuk fitur yang memang membutuhkan layer-shell.
- Hubungkan ke Hyprland secara opsional agar fitur UI yang tidak terkait compositor tetap dapat berjalan sendiri.

## Target awal

- **Sistem operasi:** Arch Linux
- **Compositor:** Hyprland
- **Display server:** Wayland
- **Arsitektur:** mengikuti arsitektur yang didukung toolchain Rust dan paket GTK4 di sistem pengguna.

## Status

Dokumen ini adalah rancangan awal, bukan implementasi. Nama aplikasi, fitur versi pertama, desain UI, dan strategi distribusi (misalnya paket Arch atau build manual) belum ditentukan. Stack dapat disederhanakan atau disesuaikan setelah daftar fitur utama disepakati.

## Rujukan teknologi

- [Dokumentasi Hyprland IPC](https://wiki.hypr.land/IPC/)
- [gtk4-layer-shell untuk Rust](https://docs.rs/gtk4-layer-shell/latest/gtk4_layer_shell/)
- [Repositori gtk4-layer-shell](https://github.com/wmww/gtk4-layer-shell)