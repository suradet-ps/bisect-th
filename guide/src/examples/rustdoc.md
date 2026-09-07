# การไบเซกต์ Rustdoc

`cargo-bisect-rustc` ใช้ตรวจสอบรีเกรสชันของ Rustdoc ได้เช่นกัน
สิ่งที่คุณต้องทำคือบอกให้มันใช้คำสั่งที่ถูกต้อง

ตัวอย่างต่อไปนี้จะตรวจหารีเกรสชันเมื่อ `cargo doc` เริ่มล้มเหลวทันที

```sh
cargo bisect-rustc --start=2022-08-05 --end=2022-09-09 -- doc
```

รีเกรสชันของ rustdoc บางอย่างอาจอยู่ในผลลัพธ์ HTML ที่สร้างขึ้น
เพื่อสแกนผลลัพธ์ คุณสามารถใช้สคริปต์ประมาณนี้:

`test.sh`:
```sh
#!/bin/sh

# Exit if any command fails.
set -e

cargo doc

grep "some example text" $CARGO_TARGET_DIR/doc/mycrate/fn.foo.html
```

สคริปต์นี้ใช้กับตัวเลือก `--script` ได้:

```sh
cargo-bisect-rustc --start=2023-01-22 --end=2023-03-18 --script=./test.sh \
    --term-old="Found example text" --term-new="Failed, or did not find text"
```