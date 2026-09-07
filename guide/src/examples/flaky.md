# ข้อผิดพลาดที่ไม่เสถียร

ชุดทดสอบบางอย่างอาจมีอาการล้มเหลวแบบสุ่มหรือไม่สม่ำเสมอ (flaky errors)
สคริปต์ต่อไปนี้เป็นตัวอย่างการสั่งรัน `rustc` ซ้ำๆ หลายรอบเพื่อดักจับข้อผิดพลาด
ตัวอย่างนี้อ้างอิงจาก issue [#108216](https://github.com/rust-lang/rust/issues/108216) (ซึ่งเกิดขึ้นเฉพาะบน macOS)

`test.sh`:
```sh
#!/bin/sh

rm -rf *.o incremental foo

echo "fn main() { let a: i64 = 1 << 64; }" > foo1.rs
echo "fn main() { let a: i64 = 1 << 63; }" > foo2.rs

ARGS="--crate-name foo -C split-debuginfo=unpacked -C debuginfo=2 -C incremental=incremental"

for i in {1..20}
do
    echo run $i
    rustc foo1.rs $ARGS && { echo "ERROR: first build should have failed"; exit 1; }
    rustc foo2.rs $ARGS || { echo "ERROR: second build should have passed"; exit 1; }
    ./foo || { echo "ERROR: executing should have passed"; exit 1; }
done
```

สามารถสั่งรันการทดสอบนี้ได้ด้วย:

```sh
cargo bisect-rustc --start=1.57.0 --end=1.58.0 --script=./test.sh
```

โดยทั่วไป คุณสามารถเขียนสคริปต์ให้ทำงานตามเงื่อนไขที่ต้องการภายในลูป `for` โดยกำหนดจำนวนรอบให้มากพอ จนมั่นใจได้ว่าตรวจพบรีเกรสชันจริง