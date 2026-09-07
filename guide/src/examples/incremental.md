# การคอมไพล์แบบอินครีเมนทัล

การทดสอบรีเกรสชันกับการคอมไพล์แบบอินครีเมนทัลอาจต้องรันคำสั่งหลายครั้ง
ต่อไปนี้แสดงตัวอย่างสำหรับ [#87384](https://github.com/rust-lang/rust/issues/87384) ซึ่งจะเกิดคำเตือนเฉพาะตอนรันบิลด์ครั้งที่สองด้วยการคอมไพล์แบบอินครีเมนทัลเท่านั้น
ก่อนหน้านี้ไม่มีการปล่อยคำเตือนใด ๆ

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

รันสคริปต์นี้ด้วย:

```sh
cargo-bisect-rustc --start 1.54.0 --end 1.55.0 --script ./test.sh
```