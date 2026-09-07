# การคอมไพล์แบบอินครีเมนทัล

การทดสอบหารีเกรสชันที่เกิดขึ้นกับการคอมไพล์แบบอินครีเมนทัล (incremental compilation) อาจจำเป็นต้องสั่งรันคำสั่งซ้ำหลายครั้ง
ตัวอย่างต่อไปนี้อ้างอิงจาก issue [#87384](https://github.com/rust-lang/rust/issues/87384) ซึ่งจะแสดง warning เฉพาะตอนสั่งบิลด์รอบที่สองด้วยโหมด incremental เท่านั้น (โดยที่คอมไพเลอร์เวอร์ชันก่อนหน้านั้นไม่เคยแสดง warning นี้เลย)

`foo.rs`:
```rust
#![type_length_limit = "95595489"]

pub fn main() {
    println!("Hello, world!");
}
```

สร้างสคริปต์ `test.sh`:

```sh
#!/bin/sh

# Exit if any command fails.
set -e

rm -rf incremental
rustc foo.rs --crate-type lib -C incremental=incremental
echo second
OUTPUT=`rustc foo.rs --crate-type lib -C incremental=incremental 2>&1`
echo $OUTPUT
! echo "$OUTPUT" | grep \
    "crate-level attribute should be in the root module"
```

สั่งรันสคริปต์นี้ด้วยคำสั่ง:

```sh
cargo-bisect-rustc --start 1.54.0 --end 1.55.0 --script ./test.sh
```