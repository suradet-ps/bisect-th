# การเปลี่ยนแปลงเอกสารประกอบ

`cargo-bisect-rustc` ใช้สแกนการเปลี่ยนแปลงในเอกสารประกอบที่แถมมากับแต่ละรีลีสได้
รวมถึงหนังสือทุกเล่มและเอกสารประกอบของไลบรารีมาตรฐาน
ในการทำเช่นนี้ ให้บอกให้มันดาวน์โหลดคอมโพเนนต์ และใช้สคริปต์ที่สแกนหาสิ่งที่คุณกำลังค้นหา
คุณสามารถใช้ `rustup doc --path` หรือ `rustc --print=sysroot` เพื่อหาตำแหน่งที่เหมาะสม ตัวอย่างเช่น:

`test.sh`:
```sh
#!/bin/sh

# Exit if any command fails.
set -e

STD=`dirname $(rustup doc --std --path)`

# Checks if a particular file exists.
# This could also be `grep` or any other kinds of tests you need.
if [ -e $STD/io/error/type.RawOsError.html ]
then
    echo "found"
    exit 1
fi
```

และรันด้วย:

```sh
cargo bisect-rustc --start 1.68.0 --end 1.69.0 -c rust-docs --script ./test.sh \
    --term-old="Did not find" --term-new="Found"
```

> **หมายเหตุ**: วิธีนี้อาจใช้ไม่ได้กับทุกแพลตฟอร์มเป้าหมาย เพราะ `cargo-bisect-rustc` ไม่ได้จัดการ rustup manifests อย่างถูกต้อง ซึ่ง manifests เหล่านี้จะทำนามแฝงให้บาง target ไปยัง target อื่น
> ให้ใช้ `--host x86_64-unknown-linux-gnu` ในสถานการณ์นั้น