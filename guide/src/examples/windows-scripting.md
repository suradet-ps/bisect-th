# การเขียนสคริปต์บน Windows

การใช้งานตัวเลือก `--script` บน Windows อาจไม่สะดวกเท่าใดนัก เนื่องจาก Windows ไม่รองรับ shebang (`#!`) เหมือนบน Unix และเครื่องมือเขียนสคริปต์พื้นฐานของ Windows เองก็อาจเขียนได้ยาก
หัวข้อต่อไปนี้จะแนะนำแนวทางต่างๆ ในการเขียนสคริปต์ทดสอบบน Windows

## ไฟล์แบตช์

คุณสามารถใช้ไฟล์ `.bat` สไตล์ DOS ได้:

`test.bat`:
```bat
(cargo check 2>&1) | find "E0642"
```

ไฟล์นี้สามารถสั่งรันได้โดยตรงด้วย:

```sh
cargo-bisect-rustc --script ./test.bat
```

อย่างไรก็ตาม ไฟล์ `.bat` อาจเขียนเงื่อนไขที่ซับซ้อนได้ยาก หรือคุณอาจไม่คุ้นเคยกับไวยากรณ์ของมัน

## Powershell

คุณไม่สามารถสั่งรันไฟล์ `.ps1` ของ Powershell ได้โดยตรง จึงจำเป็นต้องเรียกผ่านคำสั่ง `pwsh` เพื่อเปิดไฟล์ขึ้นมาทำงาน:

`test.ps1`:
```powershell
( cargo check 2>&1 ) | grep E0642
if ( -Not $? ) {
    exit 1
}
```

สามารถสั่งรันได้ด้วย:

```sh
cargo-bisect-rustc --script pwsh -- -File ./test.ps1
```

## Bash

หากคุณติดตั้ง Git-for-Windows ไว้อยู่แล้ว คุณสามารถนำ bash ที่ติดตั้งมาพร้อมกันนั้นมาใช้รันสคริปต์ bash ได้:

`test.sh`:
```sh
#!/bin/bash

cargo check 2>&1 | grep E0642
```

สามารถสั่งรันได้ด้วย:

```sh
cargo-bisect-rustc --script "C:\\Program Files\\Git\\usr\\bin\\bash.exe" -- ./test.sh
```

วิธีนี้ยังใช้ได้เช่นกันหากคุณติดตั้ง bash ผ่านสภาพแวดล้อมอื่นๆ เช่น MSYS2