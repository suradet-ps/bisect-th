# การติดตั้ง

วิธีติดตั้ง `cargo-bisect-rustc` ขั้นพื้นฐานทำได้โดยใช้คำสั่ง:

```sh
cargo install cargo-bisect-rustc
```

สำหรับตัวเลือกการติดตั้งเพิ่มเติม มีรายละเอียดดังนี้

## ข้อกำหนดเบื้องต้น

นอกเหนือจากการติดตั้ง Rust ให้พร้อมใช้งานแล้ว คุณอาจจำเป็นต้องติดตั้งเครื่องมือและไลบรารีต่อไปนี้เพิ่มเติมในระบบด้วย:

- Unix:
    - pkg-config
    - OpenSSL (`libssl-dev` สำหรับ Ubuntu, `openssl-devel` สำหรับ Fedora หรือ Alpine)
- macOS:
    - OpenSSL (แนะนำให้ติดตั้งแพ็กเกจ `openssl` ผ่าน [homebrew])
- [rustup]

[homebrew]: https://brew.sh/
[rustup]: https://rustup.rs/

หากคุณพบปัญหาในการใช้งาน OpenSSL ที่ติดตั้งอยู่ในระบบ คุณสามารถเลือกบิลด์ OpenSSL ขึ้นมาใหม่ตั้งแต่ต้นได้ โดยคำสั่งต่อไปนี้จะเปิดใช้งานการบิลด์ OpenSSL แบบ vendored:

```sh
cargo install cargo-bisect-rustc --features git2/vendored-openssl
```

ข้อควรระวัง: วิธีนี้จำเป็นต้องติดตั้ง `perl` และ `make` ไว้ในระบบด้วยเช่นกัน

## `RUST_SRC_REPO`

`cargo-bisect-rustc` จำเป็นต้องเข้าถึง git log ของรีโพสิทอรี rust ซึ่งคุณสามารถกำหนดตำแหน่งเริ่มต้นของรีโพสิทอรีดังกล่าวได้ตั้งแต่ตอนสั่งติดตั้ง:

```sh
RUST_SRC_REPO=/path/to/rust cargo install cargo-bisect-rustc
```

ดูรายละเอียดเพิ่มเติมเกี่ยวกับวิธีตั้งค่าเพื่อให้ `cargo-bisect-rustc` ดึงข้อมูลนี้ได้ที่บท [ซอร์สโค้ดของ Rust]

[ซอร์สโค้ดของ Rust]: rust-src-repo.md