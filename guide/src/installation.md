# การติดตั้ง

วิธีพื้นฐานในการติดตั้ง `cargo-bisect-rustc` คือ:

```sh
cargo install cargo-bisect-rustc
```

ตัวเลือกเพิ่มเติมอธิบายไว้ด้านล่าง

## ข้อกำหนดเบื้องต้น

นอกจากจะมีการติดตั้ง Rust ที่ใช้งานได้แล้ว คุณอาจต้องติดตั้งสิ่งอื่นอีกเล็กน้อยบนระบบของคุณ:

- Unix:
    - pkg-config
    - OpenSSL (`libssl-dev` บน Ubuntu, `openssl-devel` บน Fedora หรือ Alpine)
- macOS:
    - OpenSSL (แนะนำให้ใช้ [homebrew] เพื่อติดตั้งแพ็กเกจ `openssl`)
- [rustup]

[homebrew]: https://brew.sh/
[rustup]: https://rustup.rs/

หากคุณมีปัญหาในการใช้ OpenSSL ของระบบ คุณสามารถคอมไพล์ OpenSSL ขึ้นมาเองตั้งแต่ต้นได้
คำสั่งต่อไปนี้จะเปิดใช้งานการบิลด์ OpenSSL แบบ vendored:

```sh
cargo install cargo-bisect-rustc --features git2/vendored-openssl
```

โปรดระวังว่าวิธีนี้ยังต้องติดตั้ง `perl` และ `make` ด้วย

## `RUST_SRC_REPO`

`cargo-bisect-rustc` จำเป็นต้องเข้าถึง git log ของรีโพสิทอรี rust
คุณสามารถกำหนดตำแหน่งเริ่มต้นของรีโพสิทอรีนั้นได้ในตอนติดตั้ง:

```sh
RUST_SRC_REPO=/path/to/rust cargo install cargo-bisect-rustc
```

ดูบท [ซอร์สโค้ดของ Rust] สำหรับรายละเอียดเพิ่มเติมเกี่ยวกับการกำหนดค่าให้ `cargo-bisect-rustc` ดึงข้อมูลนี้มา

[ซอร์สโค้ดของ Rust]: rust-src-repo.md