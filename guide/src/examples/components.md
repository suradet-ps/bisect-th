# การใช้คอมโพเนนต์เพิ่มเติม

โดยค่าเริ่มต้น `cargo-bisect-rustc` ดาวน์โหลดเฉพาะ `rustc`, `cargo`, `rustdoc` และไลบรารีมาตรฐานสำหรับโฮสต์เท่านั้น
คุณอาจต้องใช้ [Rustup Components](https://rust-lang.github.io/rustup/concepts/components.html) เพิ่มเติมเพื่อรันการทดสอบของคุณ
ตัวอย่างบางส่วนของกรณีที่อาจต้องใช้ ได้แก่:

* คุณต้องการหารีเกรสชันใน Clippy (ดู [การไบเซกต์ Clippy](clippy.md)) หรือ miri
* การสแกนหาเวลาที่เอกสารประกอบบางอย่างเปลี่ยนแปลง (ดู [การเปลี่ยนแปลงเอกสารประกอบ](doc-change.md))
* แพลตฟอร์มต้องการสิ่งเพิ่มเติม
  ตัวอย่างเช่น การไบเซกต์โฮสต์ `x86_64-pc-windows-gnu` อาจต้องใช้คอมโพเนนต์ `rust-mingw`

ถ้าคุณกำลังทดสอบการคอมไพล์ข้ามแพลตฟอร์ม ให้ใช้ตัวเลือก `--target` เพื่อดาวน์โหลดไลบรารีมาตรฐานสำหรับแพลตฟอร์มเป้าหมายที่คุณใช้

ตัวอย่างต่อไปนี้แสดงวิธีใช้คอมโพเนนต์เพื่อไบเซกต์ด้วยฟีเจอร์ [build-std](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#build-std) ของ Cargo

```sh
cargo-bisect-rustc --start=2022-11-01 --end=2022-11-20 -c rust-src -- build -Zbuild-std
```

> **หมายเหตุ**: ตัวเลือก `--with-src` เป็นนามแฝงของ `-c rust-src` \
> ตัวเลือก `--with-dev` เป็นนามแฝงของ `-c rustc-dev -c llvm-tools`