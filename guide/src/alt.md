# บิลด์ทางเลือก (Alt builds)

ทุกคอมมิตยังสร้างสิ่งที่เรียกว่าบิลด์ "alt" ด้วย
นี่คือบิลด์ของ rustc ที่ตั้งค่าตัวเลือกบางอย่างต่างออกไป
ณ เดือนสิงหาคม 2023 บิลด์เหล่านี้ได้แก่:

* `rust.parallel-compiler`
* `llvm.assertions`
* `rust.verify-llvm-ir`

สำหรับข้อมูลเพิ่มเติมเกี่ยวกับการตั้งค่าเหล่านี้ ดูที่ [เอกสาร `config.toml`]
การตั้งค่า alt เหล่านี้ถูกกำหนดใน [`ci/run.sh`]

บิลด์ alt มีให้ใช้เฉพาะไม่กี่แพลตฟอร์มเป้าหมายเท่านั้น
ค้นหาบิลด์ `-alt` ใน [`ci.yml`]

วิธีนี้มีประโยชน์ถ้าคุณกำลังไบเซกต์ปัญหาเกี่ยวกับ LLVM
เมื่อเปิดใช้ LLVM assertions บิลด์ alt จะมีการตรวจสอบที่ช่วยระบุสมมติฐานที่แตกหักได้

บิลด์ alt สร้างเฉพาะสำหรับบิลด์คอมมิต ไม่ใช่สำหรับรีลีส nightly
คุณจะต้องระบุ `--by-commit` (หรือใช้ hash ในตัวเลือก `--start` หรือ `--end`) เพื่อใช้เฉพาะบิลด์คอมมิต

```sh
cargo bisect-rustc --alt --by-commit
```

[`config.toml` docs]: https://github.com/rust-lang/rust/blob/HEAD/config.example.toml
[`ci/run.sh`]: https://github.com/rust-lang/rust/blob/c0b6ffaaea3ebdf5f7a58fc4cf7ee52c91077fb9/src/ci/run.sh#L99-L105
[`ci.yml`]: https://github.com/rust-lang/rust/blob/HEAD/src/ci/github-actions/ci.yml