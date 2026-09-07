# การตรวจสอบไดแอกโนสติกส์

ต่อไปนี้เป็นตัวอย่างการตรวจสอบว่าผลลัพธ์ไดแอกโนสติกส์ของ `rustc` *เปลี่ยนแปลง* เมื่อใด
ตัวอย่างเช่น วิธีนี้สามารถตรวจสอบได้ว่าเมื่อใดถ้อยคำเปลี่ยนไป หรือเกิดข้อผิดพลาดหรือคำเตือนที่แตกต่างกัน

[#109067](https://github.com/rust-lang/rust/issues/109067) เป็นตัวอย่างที่จำเป็นต้องใช้วิธีนี้
มีคำเตือนเริ่มถูกปล่อยออกมา และเป็นคำเตือนประเภทที่ไม่สามารถเปลี่ยนเป็นข้อผิดพลาดด้วย `deny(warnings)` ได้

สคริปต์ต่อไปนี้มีไว้ใช้กับตัวเลือก `--script` (ตั้งแฟล็ก executable ให้กับสคริปต์ด้วย `chmod u+x`):

```sh
#!/bin/sh

OUTPUT=`cargo check 2>&1`
# Comment out this test if your example is intended to fail.
if [ $? -ne 0 ]
then
    echo "Build unexpectedly failed: $OUTPUT"
    exit 1
fi
# Display the output for debugging purposes.
# Run `cargo-bisect-rustc` with `-vv` to view the output.
echo "$OUTPUT"
# This indicates a regression when the text "non-ASCII" is in the output.
#
# If the regression is when the text is *not* in the output, remove the `!` prefix
# (and customize the `--term-old` and `--term-new` CLI options if you want).
! echo "$OUTPUT" | grep "non-ASCII"
```

จากนั้นรันประมาณว่า:

```sh
cargo bisect-rustc --start=1.67.0 --end=1.68.0 --script ./test.sh \
    --term-old="No warning" --term-new="Found non-ASCII warning"
```