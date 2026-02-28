# จาก Feature สู่ Foundation: Rust มุ่งสู่การพัฒนาระดับโครงสร้างข้ามปี

ในเดือนมกราคม 2026 ที่ผ่านมา ทิศทางของ Rust ได้ขยับจากการตั้งเป้าหมายรายฟีเจอร์ไปสู่สิ่งที่เรียกว่า "Roadmaps" และ "Application Areas" อย่างเต็มตัว โดยเน้นการแก้ปัญหาระดับโครงสร้างที่ต้องใช้เวลาพัฒนาข้ามปี (Multi-year Implementation) ซึ่งความน่าสนใจในปีนี้ไม่ได้อยู่ที่ Syntax ใหม่ๆ แต่อยู่ที่การลงลึกไปใน Low-level Integration กับระบบยักษ์ใหญ่ที่มีอยู่เดิมอย่าง Linux Kernel และ CPython

> **เมื่อ Rust โตพ้นช่วง "เพิ่มฟีเจอร์"**
>
> การเปลี่ยนแปลงครั้งนี้สะท้อนให้เห็นว่า Rust ไม่ได้แข่งกันที่ Syntax Sugar หรือ Feature เล็กๆ น้อยๆ อีกต่อไป แต่กำลังเข้าสู่เฟสของการ **บูรณาการระดับ System** กับโครงสร้างพื้นฐานของโลก Computing ที่มีอยู่เดิม

---

## Rust for Linux: ก้าวข้ามเฟส Driver สู่ Language Semantics

ในฝั่งของ Rust for Linux เรากำลังก้าวข้ามเฟสของการเขียน Driver เบื้องต้น ไปสู่การแก้ปัญหาที่ระดับ Language Semantics เพื่อให้สอดคล้องกับ Kernel Model โดยตรง

> **โฟกัสหลักของ Rust for Linux ในปีนี้**
>
> - **Field Projections** และ **In-place Initialization** — หัวใจสำคัญของการจัดการ Memory ที่ปลอดภัยในสภาวะที่ struct ถูก `Pin` อยู่
> - **Supertrait `auto impl`** — เพื่อลด Boilerplate code จำนวนมหาศาลที่เกิดจากการ implement trait ซ้ำๆ
> - **RFC#3848** — อนุญาตให้ส่ง pointer ไปยัง `const` ใน assembly block ได้ ซึ่งจำเป็นมากสำหรับงาน Low-level ASM ที่ต้องการความถูกต้องแม่นยำสูง

ปัญหาเรื่อง `Pin` และ In-place Initialization นั้นเป็นหนึ่งในความท้าทายที่ซับซ้อนที่สุดของ Rust เมื่อต้องทำงานกับ Kernel เพราะใน Kernel นั้น struct จำนวนมากถูก allocate แล้วไม่สามารถย้ายตำแหน่งใน memory ได้อีก การที่ Rust จะทำงานกับ model นี้ได้อย่างปลอดภัย ต้องมีกลไกระดับภาษาที่รองรับอย่างแท้จริง ไม่ใช่แค่ workaround ด้วย `unsafe`

---

## CPython Integration: เมื่อ Rust อาจกลายเป็นส่วนหนึ่งของ Python Interpreter

ความท้าทายในระดับ System Integration ยังขยายวงไปถึงโปรเจกต์ CPython ซึ่งทีม Compiler และ Libs-API กำลังทำงานร่วมกับฝั่ง Python อย่างใกล้ชิดเพื่อพิจารณานำ Rust เข้าไปเป็นส่วนหนึ่งของ Interpreter แต่โจทย์ทางเทคนิคนั้นซับซ้อนกว่าที่คิด

> **ปัญหาทางเทคนิคที่ต้องแก้ไข**
>
> | ปัญหา                   | รายละเอียด                                                                                               |
> | ----------------------- | -------------------------------------------------------------------------------------------------------- |
> | **Bootstrapping Cycle** | ปัจจุบัน Rust ใช้ Python script ในการ build ตัวเอง — หาก Python มา depend บน Rust จะเกิดวงจร "ไก่กับไข่" |
> | **Symbol Visibility**   | การจัดการ Symbol เมื่อต้อง Link Rust std lib เข้ากับ Python extension modules                            |
> | **Linker Arguments**    | การจูน Linker Arguments ที่แตกต่างกันระหว่าง `bin` และ `lib` targets ใน Crate เดียวกัน                   |
> | **Async Interop**       | การออกแบบ Interop ระหว่าง `async` Rust กับ `asyncio` ของ Python ให้ทำงานร่วมกันได้โดยไม่เกิด Overhead    |

สิ่งที่น่าจับตามองที่สุดคือเรื่อง Bootstrapping Cycle เพราะถ้าแก้ตรงนี้ไม่ได้ ทุกอย่างที่เหลือก็ไม่มีความหมาย เป็นโจทย์ระดับ Infrastructure ที่ต้องการความร่วมมือข้ามทีมอย่างแท้จริง

---

## Developer Experience: cargo-script พร้อมใช้งานจริง

ในส่วนของ Developer Experience ฟีเจอร์ที่หลายคนรอคอยอย่าง `cargo-script` ได้เข้าสู่ช่วง Final Comment Period (FCP) แล้ว

> **cargo-script คืออะไร?**
>
> การเปลี่ยนแปลงที่ทำให้เราสามารถรันไฟล์ `.rs` เดี่ยวๆ ที่ระบุ Dependency ในรูปแบบ **Front Matter** (subset ของ `Cargo.toml`) ได้ทันที โดยไม่ต้องสร้าง project structure เลย

โดยเบื้องหลังความสำเร็จนี้ ทีม Lang ได้แก้ปัญหาทางเทคนิคเล็กๆ แต่สำคัญ อย่างการจัดการตัวอักษร Stray Carriage-Return (`\r`) ใน source code ซึ่งเคยเป็น blocker ที่ทำให้ tooling ตีความ new line ผิดพลาด ตอนนี้ทุกอย่างได้รับการแก้ไขและพร้อมสำหรับการใช้งานจริงแทน Shell script ที่ซับซ้อน

### ก่อนมี cargo-script

แต่เดิมถ้าอยากเทส library ตัวหนึ่ง หรือเขียน script สั้นๆ สิ่งที่เราจะต้องทำคือ:

1. `cargo new my-temp-project` — สร้าง folder structure ทั้งหมด
2. เข้าไปแก้ `Cargo.toml` เพื่อเพิ่ม dependency
3. เข้าไปเขียนโค้ดใน `src/main.rs`
4. สั่ง `cargo run`

### หลังมี cargo-script

เราสามารถสร้างไฟล์ `.rs` แค่ไฟล์เดียว แล้วรันได้เลย เช่น `cargo ./hello-world.rs` โดยไม่ต้องมี folder `src` หรือไฟล์ `Cargo.toml` แยก

> **ทีเด็ดคือ "Front Matter"**
>
> สิ่งที่ทำให้ `cargo-script` ต่างจากการเขียน script ทั่วไปคือ มันอนุญาตให้เราฝัง config ของ `Cargo.toml` ลงไปในส่วนหัวของไฟล์ `.rs` ได้เลย (Embedded Manifest) ซึ่งหน้าตาจะเป็นแบบนี้:
>
> ```toml
> ---cargo
> [dependencies]
> serde = { version = "1", features = ["derive"] }
> reqwest = { version = "0.12", features = ["json"] }
> tokio = { version = "1", features = ["full"] }
> ---
> ```
>
> จากนั้นก็เขียนโค้ด Rust ต่อได้เลยในไฟล์เดียวกัน — **สะดวก รวดเร็ว ไม่ต้องตั้ง project**

นี่คือการเปลี่ยนแปลงที่จะทำให้ Rust เข้าถึงได้ง่ายขึ้นมากสำหรับงาน scripting, prototyping และการทดลองไอเดียใหม่ๆ

---

## Supply Chain Security: ปิดช่องโหว่ระดับ Infrastructure

ในเรื่องความปลอดภัยของ Supply Chain ในระดับ Enterprise ทีมงานกำลังวางโครงสร้างระบบ Mirroring & Verification ใหม่บนพื้นฐานของ TUF (The Update Framework)

> **TUF สำหรับ Crates.io**
>
> เป้าหมายคือให้มั่นใจว่า Crate ที่ถูกดึงผ่าน **Mirror Server** ขององค์กรหรือ Linux Distro ต่างๆ นั้น เป็น **ไบนารีเดียวกับต้นฉบับจาก Crates.io 100%** โดยไม่มีการปลอมแปลงระหว่างทาง
>
> - ป้องกัน **Man-in-the-Middle Attack** บน Mirror Server
> - รองรับ **Enterprise Environment** ที่ต้อง audit ทุก dependency
> - สร้างความเชื่อมั่นให้กับ **Linux Distro** ที่ package Rust crates

นี่คือการปิดช่องโหว่ความปลอดภัยในระดับ Infrastructure ที่มักถูกมองข้าม แต่สำคัญอย่างยิ่งสำหรับองค์กรที่ต้องการนำ Rust ไปใช้ใน Production ระดับ Mission-critical

---

## บทสรุป

สิ่งที่เกิดขึ้นกับ Rust ในช่วงต้นปี 2026 นี้ สะท้อนให้เห็นภาพที่ชัดเจนว่า Rust กำลังเปลี่ยนผ่านจากภาษาที่เน้น "เพิ่มฟีเจอร์ใหม่" ไปสู่ภาษาที่เน้น "วางรากฐานระดับระบบ" อย่างแท้จริง

ไม่ว่าจะเป็นการลงลึกกับ Linux Kernel ในระดับ Language Semantics, การแก้โจทย์ Bootstrapping ที่ซับซ้อนกับ CPython, การทำให้ชีวิต Developer ง่ายขึ้นด้วย `cargo-script` หรือการสร้างความปลอดภัยระดับ Supply Chain ด้วย TUF — ทั้งหมดนี้ล้วนเป็นงานที่ต้องใช้เวลาข้ามปี แต่เมื่อสำเร็จแล้วจะเปลี่ยนแปลงวิธีที่โลกใช้ Rust ไปตลอดกาล

สำหรับ Rustacean ที่ติดตามอยู่ นี่คือช่วงเวลาที่น่าตื่นเต้นมาก เพราะเรากำลังเห็น Rust วางตัวเองเป็น Foundation ของ System Programming ยุคใหม่อย่างแท้จริงครับ

---

**Credit & Reference:**

1. [Inside Rust Blog - Program management update (January 2026)](https://blog.rust-lang.org/inside-rust/2026/02/11/program-management-update-2026-01/)
