# การเขียนสคริปต์บน Windows

การใช้ตัวเลือก `--script` บน Windows อาจยุ่งยาก เพราะ Windows ไม่รองรับสคริปต์ `#!` แบบ Unix และการเขียนสคริปต์ในตัวก็อาจไม่สะดวกเช่นกัน
หัวข้อต่อไปนี้แสดงวิธีต่าง ๆ ที่คุณสามารถใช้การเขียนสคริปต์ได้

## ไฟล์แบตช์

คุณสามารถใช้ไฟล์ `.bat` แบบ DOS ได้:

`test.bat`:
```bat
(cargo check 2>&1) | find "E0642"
```

ไฟล์นี้สามารถรันได้โดยตรงด้วย:

```sh
cargo-bisect-rustc --script ./test.bat
```

แต่ `.bat` อาจท้าทายสำหรับตัวเลือกที่ซับซ้อนกว่า หรือคุณอาจไม่คุ้นเคยกับมัน

## Powershell

คุณไม่สามารถรันไฟล์ `.ps1` ของ Powershell ได้โดยตรง ดังนั้นคุณจะต้องใช้ `pwsh` เพื่อรันมัน:

`test.ps1`:
```powershell
( cargo check 2>&1 ) | grep E0642
if ( -Not $? ) {
    exit 1
}
```

สามารถรันได้ด้วย:

```sh
cargo-bisect-rustc --script pwsh -- -File ./test.ps1
```

## Bash

ถ้าคุณติดตั้ง Git-for-Windows ไว้ คุณสามารถใช้ bash ที่มาพร้อมมันเพื่อรันสคริปต์ bash:

`test.sh`:
```sh
#!/bin/bash

cargo check 2>&1 | grep E0642
```

สามารถรันได้ด้วย:

```sh
cargo-bisect-rustc --script "C:\\Program Files\\Git\\usr\\bin\\bash.exe" -- ./test.sh
```

วิธีนี้ยังใช้ได้ถ้าคุณมี bash จากอะไรทำนอง msys2 ติดตั้งอยู่