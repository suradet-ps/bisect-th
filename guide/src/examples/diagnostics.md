# การตรวจสอบไดแอกโนสติกส์

ตัวอย่างต่อไปนี้แสดงวิธีตรวจสอบหาจุดที่ข้อความผลลัพธ์ไดแอกโนสติกส์ (diagnostic output) ของ `rustc` เกิดการ *เปลี่ยนแปลง*
เช่น ใช้ตรวจสอบเมื่อสำนวนข้อความเปลี่ยนไป หรือเมื่อคอมไพเลอร์เริ่มแสดงข้อผิดพลาดหรือคำเตือนที่ต่างไปจากเดิม

ตัวอย่างที่เห็นได้ชัดคือ issue [#109067](https://github.com/rust-lang/rust/issues/109067)
ซึ่งจู่ๆ มีการแสดง warning รูปแบบหนึ่งขึ้นมา และเป็นคำเตือนประเภทที่ไม่สามารถแปลงเป็น error ผ่านคำสั่ง `deny(warnings)` ได้

สคริปต์ต่อไปนี้ออกแบบมาสำหรับใช้งานร่วมกับตัวเลือก `--script` (อย่าลืมตั้งค่าสิทธิ์ให้ไฟล์สามารถรันได้ด้วย `chmod u+x`):

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

จากนั้นสั่งรันคำสั่งในลักษณะดังนี้:

```sh
cargo bisect-rustc --start=1.67.0 --end=1.68.0 --script ./test.sh \
    --term-old="No warning" --term-new="Found non-ASCII warning"
```