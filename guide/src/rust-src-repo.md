# ซอร์สโค้ดของ Rust

เพื่อให้ `cargo-bisect-rustc` ทำงานได้ มันต้องอ่าน git log ของรีโพสิทอรี [`rust-lang/rust`]
`cargo-bisect-rustc` รองรับหลายวิธีในการทำเช่นนี้ตามที่อธิบายไว้ด้านล่าง

## GitHub API

โดยค่าเริ่มต้น `cargo-bisect-rustc` ใช้ GitHub API เพื่อดึงข้อมูลแทนการใช้โฟลเดอร์ checkout ในเครื่อง

```sh
cargo bisect-rustc --access=github
```

โปรดระวังว่า GitHub มีข้อจำกัดอัตราการเรียก (rate limit) ที่เข้มงวดสำหรับคำขอที่ไม่มีการยืนยันตัวตน
อนุญาต 60 คำขอต่อชั่วโมง และ `cargo-bisect-rustc` จะใช้ประมาณ 10 คำขอทุกครั้งที่รัน (ซึ่งอาจแตกต่างกันไปตามการไบเซกต์)
หากคุณเจอ rate limit คุณสามารถเพิ่มเป็น 5000 คำขอต่อชั่วโมงได้โดยตั้งตัวแปรสภาพแวดล้อม `GITHUB_TOKEN` เป็น [GitHub personal token]
หากคุณใช้ [`gh` CLI tool] คุณสามารถใช้มันเพื่อขอ token:

```sh
GITHUB_TOKEN=`gh auth token` cargo bisect-rustc --access=github
```

ถ้าคุณไม่ได้ใช้ `gh` คุณก็แค่ต้องคัดลอกและวาง token

## Local clone

`cargo-bisect-rustc` ยังสามารถ clone รีโพสิทอรี rust ในไดเรกทอรีปัจจุบัน (เป็น `rust.git`) ได้
ตัวเลือกนี้อาจช้าเอาเรื่องถ้าคุณไม่ระบุ path ของรีโพสิทอรีตอนติดตั้ง
คุณสามารถระบุได้ด้วยอาร์กิวเมนต์ CLI `--access`:
```sh
cargo bisect-rustc --access=checkout
```

## ตัวแปรสภาพแวดล้อม `RUST_SRC_REPO`

คุณสามารถระบุตำแหน่งของรีโพสิทอรี rust ด้วยตัวแปรสภาพแวดล้อม `RUST_SRC_REPO` ตอนรัน
วิธีนี้มีประโยชน์ถ้าคุณมีรีโพสิทอรี checkout ไว้ที่ไหนสักแห่งอยู่แล้ว แต่ใช้งานค่อนข้างยุ่งยาก

```sh
RUST_SRC_REPO=/path/to/rust cargo bisect-rustc
```

## ตัวแปรสภาพแวดล้อม `RUST_SRC_REPO` (ตอนติดตั้ง)

การตั้งตัวแปรสภาพแวดล้อม `RUST_SRC_REPO` ตอนติดตั้ง `cargo-bisect-rustc` จะตั้งตำแหน่งเริ่มต้นของรีโพสิทอรี rust
แนะนำให้ทำวิธีนี้ถ้าคุณมีรีโพสิทอรี rust checkout ไว้ที่ไหนสักแห่งอยู่แล้ว

```sh
RUST_SRC_REPO=/path/to/rust cargo install cargo-bisect-rustc
```

[`rust-lang/rust`]: https://github.com/rust-lang/rust/
[GitHub personal token]: https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token
[`gh` CLI tool]: https://cli.github.com/