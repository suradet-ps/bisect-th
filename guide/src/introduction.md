# บทนำ

เครื่องมือ [`cargo-bisect-rustc`] ทำให้การค้นหาจุดที่พฤติกรรมของ rustc เกิดการถดถอย (regression) ทำได้ง่ายมาก
เครื่องมือจะดาวน์โหลดอาร์ติแฟกต์ของ rustc โดยอัตโนมัติและทดสอบกับโปรเจกต์ที่คุณกำหนด จนกว่าจะพบจุดที่เกิดรีเกรสชัน

บท [การติดตั้ง](installation.md) อธิบายวิธีติดตั้ง `cargo-bisect-rustc`
สำหรับการเริ่มต้นอย่างรวดเร็ว ดูที่บท [บทช่วยสอน](tutorial.md)
หรือเริ่มจากบท [การใช้งานพื้นฐาน](usage.md) เพื่อเรียนรู้ว่า `cargo-bisect-rustc` ทำงานอย่างไร

[`cargo-bisect-rustc`]: https://github.com/rust-lang/cargo-bisect-rustc