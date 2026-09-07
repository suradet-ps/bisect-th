# ข้อผิดพลาดที่ไม่เสถียร

การทดสอบบางอย่างอาจล้มเหลวแบบสุ่ม
สคริปต์ต่อไปนี้เป็นตัวอย่างที่จะรัน `rustc` ซ้ำ ๆ เพื่อตรวจสอบความล้มเหลว
ตัวอย่างนี้มาจาก [#108216](https://github.com/rust-lang/rust/issues/108216) (ซึ่งต้องใช้ macOS)

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

การทดสอบนี้รันได้ด้วย:

```sh
cargo bisect-rustc --start=1.57.0 --end=1.58.0 --script=./test.sh
```

โดยทั่วไป ให้กำหนดค่าสคริปต์ให้ทำงานตามที่คุณต้องการภายในลูป `for` ที่รันพอหลายรอบจนคุณมั่นใจสูงว่าเจอรีเกรสชันแล้ว