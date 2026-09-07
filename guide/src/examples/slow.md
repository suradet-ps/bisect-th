# การคอมไพล์ที่ช้าหรือค้าง

รีเกรสชันบางอย่างอาจเกี่ยวข้องกับคอมไพเลอร์ที่ค้างหรือใช้เวลาในการรันนานผิดปกติ
ตัวเลือก CLI `--timeout` ใช้ตรวจสอบสิ่งนี้ได้
ลองใช้ [#89524](https://github.com/rust-lang/rust/issues/89524) เป็นตัวอย่าง
ปัจจัยหลายอย่างรวมกันทำให้คอมไพเลอร์เริ่มค้าง

เปลี่ยน `Cargo.toml` เป็นดังนี้:

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

จากนั้นใช้ตัวเลือก timeout:

```sh
cargo-bisect-rustc --start=2021-09-01 --end=2021-10-02 --timeout 30 -- build --release
```

คุณอาจต้องปรับค่า timeout ตามความเร็วของระบบของคุณ

> **หมายเหตุ**: `--timeout` ยังไม่ทำงานบน macOS ในตอนนี้ ดูที่ <https://github.com/rust-lang/cargo-bisect-rustc/issues/232>

ในบางกรณีการไบเซกต์เฉพาะว่ามี timeout เกิดขึ้นหรือไม่นั้นไม่พอ อาจมีข้อผิดพลาดการคอมไพล์ด้วย (ดูตัวอย่าง [rustc#139197](https://github.com/rust-lang/rust/issues/139197)) ในกรณีนี้สคริปต์ควรจัดการงานทั้งหมด ต่อไปนี้เป็นตัวอย่าง Bash (สมมติว่า `main.rs` คือโค้ด Rust ที่จำลองการคอมไพล์ที่ค้างได้):
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

จากนั้นรัน (ตัวอย่าง):
```sh
cargo-bisect-rustc [...params...] --script test.sh
```