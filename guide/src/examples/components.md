# การใช้คอมโพเนนต์เพิ่มเติม

โดยค่าเริ่มต้น `cargo-bisect-rustc` จะดาวน์โหลดเฉพาะ `rustc`, `cargo`, `rustdoc` และไลบรารีมาตรฐาน (standard library) สำหรับเครื่องโฮสต์เท่านั้น
คุณอาจจำเป็นต้องติดตั้ง [Rustup Components](https://rust-lang.github.io/rustup/concepts/components.html) เพิ่มเติมเพื่อให้สามารถรันชุดทดสอบของคุณได้
ตัวอย่างสถานการณ์ที่จำเป็นต้องใช้ ได้แก่:

* คุณต้องการค้นหารีเกรสชันใน Clippy (ดู [การไบเซกต์ Clippy](clippy.md)) หรือ miri
* การสแกนหาจุดที่เอกสารประกอบมีการเปลี่ยนแปลง (ดู [การเปลี่ยนแปลงเอกสารประกอบ](doc-change.md))
* แพลตฟอร์มปลายทางต้องการคอมโพเนนต์เฉพาะเพิ่มเติม
  เช่น การไบเซกต์บนโฮสต์ `x86_64-pc-windows-gnu` อาจจำเป็นต้องติดตั้งคอมโพเนนต์ `rust-mingw`

หากคุณกำลังทดสอบการคอมไพล์ข้ามแพลตฟอร์ม (cross-compilation) ให้ระบุตัวเลือก `--target` เพื่อให้ระบบดาวน์โหลดไลบรารีมาตรฐานสำหรับทาร์เก็ตปลายทางที่คุณใช้งานมาด้วย

ตัวอย่างต่อไปนี้แสดงวิธีระบุคอมโพเนนต์เพื่อทำการไบเซกต์ร่วมกับฟีเจอร์ [build-std](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#build-std) ของ Cargo:

```sh
cargo-bisect-rustc --start=2022-11-01 --end=2022-11-20 -c rust-src -- build -Zbuild-std
```

> **หมายเหตุ**: ตัวเลือก `--with-src` เป็น alias (คำสั่งย่อ) ของ `-c rust-src` \
> ตัวเลือก `--with-dev` เป็น alias (คำสั่งย่อ) ของ `-c rustc-dev -c llvm-tools`