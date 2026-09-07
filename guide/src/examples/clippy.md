# การไบเซกต์ Clippy

คุณสามารถใช้ `cargo-bisect-rustc` เพื่อตรวจสอบรีเกรสชันของ Clippy ได้เช่นกัน
โดยคุณจำเป็นต้องระบุให้ระบบดาวน์โหลดคอมโพเนนต์ clippy และกำหนดคำสั่งที่จะรันให้ถูกต้อง:

```sh
cargo bisect-rustc --start=1.67.0 --end=1.68.0 -c clippy -- clippy
```

ทั้งนี้ ผลลัพธ์ที่ได้อาจขึ้นอยู่กับลักษณะปัญหาที่คุณกำลังค้นหา ในบางกรณี การไบเซกต์อาจชี้ไปเจอเพียง PR ที่ทำหน้าที่ซิงก์รีโพสิทอรี [`rust-clippy`] เข้าสู่ `rust-lang/rust` เท่านั้น
ซึ่งคุณอาจสามารถลองไล่ดูรายการโค้ดที่มีการเปลี่ยนแปลงใน PR นั้นเพื่อค้นหาสาเหตุ แต่หากรายการการเปลี่ยนแปลงมีขนาดใหญ่เกินไป หรือไม่พบจุดที่น่าสงสัย ให้พิจารณาใช้ [`git bisect`] กับตัวรีโพสิทอรีของ clippy โดยตรงแทน (ซึ่งจำเป็นต้องบิลด์ clippy ขึ้นมาเอง)

สำหรับการไบเซกต์ warning ของ clippy คุณสามารถปรับระดับจาก warning ให้กลายเป็น error ได้ดังนี้:

```sh
cargo bisect-rustc --start=1.84.0 --end=1.85.0 -c clippy -- clippy -- --forbid clippy::useless_conversion
```

[`rust-clippy`]: https://github.com/rust-lang/rust-clippy/
[`git bisect`]: https://git-scm.com/docs/git-bisect