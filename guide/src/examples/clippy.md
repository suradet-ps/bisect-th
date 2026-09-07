# การไบเซกต์ Clippy

`cargo-bisect-rustc` ใช้ตรวจสอบรีเกรสชันของ Clippy ได้เช่นกัน
คุณจะต้องบอกให้มันดาวน์โหลด clippy และรันคำสั่งให้ถูกต้อง:

```sh
cargo bisect-rustc --start=1.67.0 --end=1.68.0 -c clippy -- clippy
```

โปรดทราบว่าขึ้นอยู่กับสิ่งที่คุณกำลังค้นหา วิธีนี้อาจพบแค่ PR ที่ซิงก์รีโพสิทอรี [`rust-clippy`] เข้ากับ `rust-lang/rust`
คุณอาจสามารถสแกนรายการการเปลี่ยนแปลงใน PR นั้นเพื่อค้นหาสิ่งที่คุณต้องการ
ถ้ารายการการเปลี่ยนแปลงใหญ่เกินไปหรือไม่มีอะไรโดดเด่นออกมาเป็นผู้ต้องสงสัย ให้พิจารณาใช้ [`git bisect`] บนรีโพสิทอรี clippy เอง (ซึ่งจะต้องบิลด์ clippy)

เพื่อไบเซกต์คำเตือนของ clippy คุณสามารถอัปเกรดคำเตือนเป็นข้อผิดพลาดได้:

```sh
cargo bisect-rustc --start=1.84.0 --end=1.85.0 -c clippy -- clippy -- --forbid clippy::useless_conversion
```

[`rust-clippy`]: https://github.com/rust-lang/rust-clippy/
[`git bisect`]: https://git-scm.com/docs/git-bisect