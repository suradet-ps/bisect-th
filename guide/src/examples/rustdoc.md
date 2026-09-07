# การไบเซกต์ Rustdoc

คุณสามารถใช้ `cargo-bisect-rustc` เพื่อตรวจสอบรีเกรสชันของ Rustdoc ได้เช่นกัน
สิ่งที่คุณต้องทำมีเพียงแค่ระบุคำสั่งที่ต้องการทดสอบให้ถูกต้อง

ตัวอย่างต่อไปนี้จะแสดงการค้นหารีเกรสชันในกรณีที่จู่ๆ คำสั่ง `cargo doc` เกิดบิลด์ไม่ผ่านขึ้นมา:

```sh
cargo bisect-rustc --start=2022-08-05 --end=2022-09-09 -- doc
```

รีเกรสชันของ rustdoc บางอย่างอาจแฝงอยู่ในเนื้อหาไฟล์ HTML ที่สร้างขึ้น
หากต้องการสแกนตรวจสอบผลลัพธ์ดังกล่าว คุณสามารถเขียนสคริปต์ได้ดังนี้:

`test.sh`:
```sh
#!/bin/sh

# Exit if any command fails.
set -e

cargo doc

grep "some example text" $CARGO_TARGET_DIR/doc/mycrate/fn.foo.html
```

สามารถนำสคริปต์นี้ไปใช้งานร่วมกับตัวเลือก `--script`:

```sh
cargo-bisect-rustc --start=2023-01-22 --end=2023-03-18 --script=./test.sh \
    --term-old="Found example text" --term-new="Failed, or did not find text"
```