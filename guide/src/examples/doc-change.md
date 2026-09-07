# การเปลี่ยนแปลงเอกสารประกอบ

คุณสามารถใช้ `cargo-bisect-rustc` เพื่อสแกนหาความเปลี่ยนแปลงในเอกสารประกอบที่เผยแพร่มาพร้อมกับแต่ละรีลีสได้
ซึ่งครอบคลุมทั้งหนังสือคู่มือทุกเล่มและเอกสารของไลบรารีมาตรฐาน
วิธีการคือระบุให้ระบบดาวน์โหลดคอมโพเนนต์เอกสารลงมา และใช้สคริปต์ช่วยสแกนหาสิ่งที่คุณต้องการ
คุณสามารถใช้คำสั่ง `rustup doc --path` หรือ `rustc --print=sysroot` เพื่อค้นหาพาธตำแหน่งไฟล์เอกสารที่ถูกต้อง ตัวอย่างเช่น:

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

และสั่งรันด้วยคำสั่ง:

```sh
cargo bisect-rustc --start 1.68.0 --end 1.69.0 -c rust-docs --script ./test.sh \
    --term-old="Did not find" --term-new="Found"
```

> **หมายเหตุ**: วิธีนี้อาจใช้งานไม่ได้บนบางทาร์เก็ต เนื่องจาก `cargo-bisect-rustc` ยังจัดการกับ rustup manifests ที่ทำ alias ชี้ทาร์เก็ตหนึ่งไปยังอีกทาร์เก็ตหนึ่งได้ไม่สมบูรณ์
> หากพบปัญหานี้ ให้ระบุ `--host x86_64-unknown-linux-gnu` เพื่อแก้ไข