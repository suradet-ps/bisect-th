# การคอมไพล์ที่ช้าหรือค้าง

รีเกรสชันบางประเภทอาจเกิดในลักษณะที่คอมไพเลอร์ค้าง (hang) หรือใช้เวลาประมวลผลนานผิดปกติ
คุณสามารถใช้ตัวเลือก CLI `--timeout` เพื่อตรวจจับปัญหานี้ได้
ลองดูตัวอย่างจาก [#89524](https://github.com/rust-lang/rust/issues/89524)
ซึ่งการรวมกันของปัจจัยและเงื่อนไขเฉพาะบางประการทำให้คอมไพเลอร์เริ่มเกิดอาการค้าง

แก้ไขไฟล์ `Cargo.toml` เป็นดังนี้:

```toml
[package]
name = "slow"
version = "0.1.0"

[dependencies]
config = "=0.9.3"

[profile.release]
panic = "abort"
codegen-units = 1
```

จากนั้นสั่งรันพร้อมกำหนดตัวเลือก timeout:

```sh
cargo-bisect-rustc --start=2021-09-01 --end=2021-10-02 --timeout 30 -- build --release
```

คุณอาจจำเป็นต้องปรับค่า timeout ให้เหมาะสมกับความเร็วของเครื่องของคุณ

> **หมายเหตุ**: ปัจจุบันตัวเลือก `--timeout` ยังไม่รองรับการทำงานบน macOS ดูรายละเอียดได้ที่ <https://github.com/rust-lang/cargo-bisect-rustc/issues/232>

ในบางกรณี การตรวจจับแค่ว่าเกิด timeout หรือไม่เพียงอย่างเดียวอาจไม่เพียงพอ เพราะอาจมีข้อผิดพลาดในการคอมไพล์ (compilation error) ร่วมด้วย (ดูตัวอย่างได้ใน [rustc#139197](https://github.com/rust-lang/rust/issues/139197)) ในสถานการณ์เช่นนี้ เราจึงควรให้สคริปต์เป็นผู้ควบคุมเงื่อนไขทั้งหมด ตัวอย่างสคริปต์ Bash ต่อไปนี้ (กำหนดให้ `main.rs` เป็นโค้ด Rust ที่ทำให้เกิดปัญหาคอมไพล์ค้าง):
```sh
#!/bin/bash
res=$( timeout 3 rustc main.rs )
if [ "$?" -eq 124 ]; then
    # Excessive compile time
    exit 1
else
    # Compilation fails as expected *but* it doesn't hang
    exit 0
fi
```

จากนั้นสั่งรัน (ตัวอย่าง):
```sh
cargo-bisect-rustc [...params...] --script test.sh
```