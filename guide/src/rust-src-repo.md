# ซอร์สโค้ดของ Rust

ในการทำงาน `cargo-bisect-rustc` จำเป็นต้องเข้าถึงและอ่าน git log ของรีโพสิทอรี [`rust-lang/rust`]
โดย `cargo-bisect-rustc` รองรับวิธีการเชื่อมต่อหลายรูปแบบดังที่อธิบายไว้ด้านล่างนี้

## GitHub API

โดยค่าเริ่มต้น `cargo-bisect-rustc` จะดึงข้อมูลผ่าน GitHub API แทนที่จะใช้โฟลเดอร์ local checkout บนเครื่องของคุณ

```sh
cargo bisect-rustc --access=github
```

ข้อควรระวัง: GitHub มีการจำกัดอัตราการเรียกใช้งาน (rate limit) ที่เข้มงวดสำหรับคำขอที่ไม่ได้ยืนยันตัวตน
โดยอนุญาตเพียง 60 คำขอต่อชั่วโมง และ `cargo-bisect-rustc` จะใช้ประมาณ 10 คำขอในแต่ละครั้งที่รัน (อาจมากหรือน้อยกว่านี้ขึ้นอยู่กับการไบเซกต์)
หากคุณพบปัญหาติด rate limit คุณสามารถเพิ่มขีดจำกัดเป็น 5,000 คำขอต่อชั่วโมงได้ โดยตั้งค่าตัวแปรสภาพแวดล้อม `GITHUB_TOKEN` ให้เป็น [GitHub personal token]
หากคุณติดตั้งเครื่องมือ [`gh` CLI tool] อยู่แล้ว สามารถใช้คำสั่งนี้เพื่อดึง token ได้อย่างสะดวก:

```sh
GITHUB_TOKEN=`gh auth token` cargo bisect-rustc --access=github
```

หากไม่ได้ติดตั้ง `gh` คุณก็เพียงแค่คัดลอก personal token จากหน้าเว็บ GitHub มาวางใส่ค่าตัวแปรโดยตรง

## Local clone

`cargo-bisect-rustc` สามารถสั่ง clone รีโพสิทอรี rust ลงมาไว้ในไดเรกทอรีปัจจุบัน (ในชื่อ `rust.git`) ได้เช่นกัน
ตัวเลือกนี้อาจใช้เวลานานมาก (เนื่องจากขนาดของรีโพสิทอรีมีขนาดใหญ่) หากคุณไม่ได้กำหนด path ของรีโพสิทอรีที่มีอยู่แล้วไว้ตั้งแต่ตอนบิลด์ติดตั้ง
คุณสามารถเลือกใช้วิธีนี้ได้โดยระบุอาร์กิวเมนต์ CLI `--access`:
```sh
cargo bisect-rustc --access=checkout
```

## ตัวแปรสภาพแวดล้อม `RUST_SRC_REPO`

คุณสามารถระบุตำแหน่งโฟลเดอร์ของรีโพสิทอรี rust ได้ผ่านตัวแปรสภาพแวดล้อม `RUST_SRC_REPO` ในขณะสั่งรันคำสั่ง
วิธีนี้เหมาะสำหรับกรณีที่คุณมีรีโพสิทอรี rust โคลนเก็บไว้ในเครื่องอยู่แล้ว แต่อาจไม่สะดวกเท่าใดนักเพราะต้องคอยใส่ตัวแปรทุกครั้งที่สั่งรัน

```sh
RUST_SRC_REPO=/path/to/rust cargo bisect-rustc
```

## ตัวแปรสภาพแวดล้อม `RUST_SRC_REPO` (ตอนติดตั้ง)

การตั้งค่าตัวแปรสภาพแวดล้อม `RUST_SRC_REPO` ในขั้นตอนการติดตั้ง `cargo-bisect-rustc` จะเป็นการกำหนดตำแหน่งเริ่มต้นของรีโพสิทอรี rust อย่างถาวร
วิธีนี้เป็นวิธีที่แนะนำอย่างยิ่งหากคุณมีรีโพสิทอรี rust โคลนไว้ในเครื่องอยู่แล้ว

```sh
RUST_SRC_REPO=/path/to/rust cargo install cargo-bisect-rustc
```

[`rust-lang/rust`]: https://github.com/rust-lang/rust/
[GitHub personal token]: https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token
[`gh` CLI tool]: https://cli.github.com/