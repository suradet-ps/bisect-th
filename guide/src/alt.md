# บิลด์ทางเลือก (Alt builds)

ในแต่ละคอมมิต ระบบยังมีการสร้างสิ่งที่เรียกว่าบิลด์ "alt" (alternative builds) ควบคู่ไปด้วย
ซึ่งก็คือบิลด์ของ rustc ที่มีการเปิดใช้งานตัวเลือกพิเศษบางอย่างแตกต่างออกไป
โดย ณ เดือนสิงหาคม 2023 ตัวเลือกเหล่านี้ประกอบด้วย:

* `rust.parallel-compiler`
* `llvm.assertions`
* `rust.verify-llvm-ir`

สำหรับข้อมูลเพิ่มเติมเกี่ยวกับการตั้งค่าเหล่านี้ สามารถดูได้ที่ [เอกสาร `config.toml`][`config.toml` docs]
โดยการตั้งค่าบิลด์ alt เหล่านี้จะถูกกำหนดไว้ใน [`ci/run.sh`]

ทั้งนี้ บิลด์ alt จะมีให้ใช้งานเฉพาะบางทาร์เก็ตเท่านั้น
สามารถตรวจสอบรายการบิลด์ที่มีคำว่า `-alt` ได้ใน [`ci.yml`]

บิลด์ลักษณะนี้มีประโยชน์อย่างยิ่งเมื่อคุณกำลังไบเซกต์ปัญหาที่เกี่ยวข้องกับ LLVM
เนื่องจากการเปิดใช้งาน LLVM assertions จะทำให้บิลด์ alt มีระบบตรวจสอบเงื่อนไขภายใน ซึ่งช่วยชี้จุดที่ข้อกำหนดหรือสมมติฐานการทำงานของคอมไพเลอร์เกิดข้อผิดพลาด (broken assumptions) ได้ง่ายขึ้น

อนึ่ง บิลด์ alt จะถูกสร้างขึ้นสำหรับคอมมิตบิลด์ (commit builds) เท่านั้น ไม่ได้สร้างสำหรับรีลีส nightly ปกติ
คุณจึงจำเป็นต้องระบุตัวเลือก `--by-commit` (หรือส่ง commit hash ให้กับแฟล็ก `--start` หรือ `--end`) เพื่อบังคับให้ใช้เฉพาะคอมมิตบิลด์:

```sh
cargo bisect-rustc --alt --by-commit
```

[`config.toml` docs]: https://github.com/rust-lang/rust/blob/HEAD/config.example.toml
[`ci/run.sh`]: https://github.com/rust-lang/rust/blob/c0b6ffaaea3ebdf5f7a58fc4cf7ee52c91077fb9/src/ci/run.sh#L99-L105
[`ci.yml`]: https://github.com/rust-lang/rust/blob/HEAD/src/ci/github-actions/ci.yml