# ทูลเชนของ Rustup

`cargo-bisect-rustc` ใช้ประโยชน์จาก [ทูลเชนของ rustup] ในการติดตั้งและเลือก `rustc` ที่ถูกต้องเพื่อรัน
โดยพื้นฐานแล้วมันจะรัน `cargo +bisector-nightly-2023-03-18-x86_64-unknown-linux-gnu build` โดยใช้ [ตัวย่อการโอเวอร์ไรด์ทูลเชน] ของ rustup เพื่อรันทูลเชนที่มันดาวน์โหลด
สิ่งนี้ตั้งตัวแปรสภาพแวดล้อม `RUSTUP_TOOLCHAIN` เป็นชื่อทูลเชน ซึ่งทำให้แน่ใจว่าการเรียกใช้ `rustc` ทุกครั้งจะใช้ทูลเชนที่ถูกต้อง

โดยค่าเริ่มต้น `cargo-bisect-rustc` จะลบทูลเชนทันทีหลังใช้งานเสร็จ
คุณสามารถใช้ตัวเลือก `--preserve` เพื่อเก็บทูลเชนไว้เพื่อใช้งานด้วยตนเอง
ดูตัวอย่าง [การรักษาทูลเชนไว้] สำหรับรายละเอียดเพิ่มเติม

เมื่อใช้ตัวเลือก `--script` สคริปต์ควรเรียกใช้ `cargo` หรือ `rustc` ตามปกติ และพึ่งพาตัวแปรสภาพแวดล้อม `RUSTUP_TOOLCHAIN` เพื่อเลือกทูลเชนที่ถูกต้อง

[ทูลเชนของ rustup]: https://rust-lang.github.io/rustup/concepts/toolchains.html
[ตัวย่อการโอเวอร์ไรด์ทูลเชน]: https://rust-lang.github.io/rustup/overrides.html#toolchain-override-shorthand
[การรักษาทูลเชนไว้]: examples/preserve.md