# จาก Feature สู่ Foundation

> 📅 วันที่เผยแพร่: 2026-02-12

ในเดือนมกราคม 2026 ที่ผ่านมาทิศทางของ Rust ได้ขยับจากการตั้งเป้าหมายรายฟีเจอร์ไปสู่สิ่งที่เรียกว่า "Roadmaps" และ "Application Areas" อย่างเต็มตัว โดยเน้นการแก้ปัญหาระดับโครงสร้างที่ต้องใช้เวลาพัฒนาข้ามปี (Multi-year implementation) ซึ่งความน่าสนใจในปีนี้ไม่ได้อยู่ที่ Syntax ใหม่ๆ แต่อยู่ที่การลงลึกไปใน Low-level Integration กับระบบยักษ์ใหญ่ที่มีอยู่เดิมอย่าง Linux Kernel และ CPython

## ระดับ System Integration: Linux Kernel และ CPython

ในฝั่งของ Rust for Linux เรากำลังก้าวข้ามเฟสของการเขียน Driver เบื้องต้น ไปสู่การแก้ปัญหาที่ระดับ Language Semantics เพื่อให้สอดคล้องกับ Kernel Model โดยตรง ทีมงานกำลังโฟกัสเรื่อง Field Projections และ In-place Initialization ซึ่งเป็นหัวใจสำคัญของการจัดการ Memory ที่ปลอดภัยในสภาวะที่ struct ถูก `Pin` อยู่ นอกจากนี้ยังมีการผลักดันฟีเจอร์ Supertrait `auto impl` เพื่อลด Boilerplate code จำนวนมหาศาล และ RFC#3848 ที่อนุญาตให้ส่ง pointer ไปยัง `const` ใน assembly block ได้ ซึ่งจำเป็นมากสำหรับงาน Low-level ASM ที่ต้องการความถูกต้องแม่นยำสูง

ความท้าทายในระดับ System Integration ยังขยายวงไปถึงโปรเจกต์ CPython ซึ่งทีม Compiler และ Libs-API กำลังทำงานร่วมกับฝั่ง Python อย่างใกล้ชิดเพื่อพิจารณานำ Rust เข้าไปเป็นส่วนหนึ่งของ Interpreter แต่โจทย์ทางเทคนิคนั้นซับซ้อนกว่าที่คิด ตั้งแต่ปัญหา Bootstrapping Cycle (เพราะปัจจุบัน Rust ใช้ Python script ในการ build ตัวเอง หาก Python มา depend บน Rust จะเกิดวงจรไก่กับไข่), การจัดการ Symbol Visibility เมื่อต้อง Link Rust std lib เข้ากับ Python extension modules, ไปจนถึงการจูน Linker Arguments ที่แตกต่างกันระหว่าง `bin` และ `lib` targets ใน Crate เดียวกัน รวมถึงการออกแบบ Interop ระหว่าง `async` Rust กับ `asyncio` ของ Python ให้ทำงานร่วมกันได้โดยไม่เกิด Overhead

## Developer Experience และ cargo-script

ในส่วนของ Developer Experience ฟีเจอร์ที่หลายคนรอคอยอย่าง `cargo-script` ได้เข้าสู่ช่วง Final Comment Period (FCP) แล้ว การเปลี่ยนแปลงนี้จะทำให้เราสามารถรันไฟล์ `.rs` เดี่ยวๆ ที่ระบุ Dependency ในรูปแบบ Front matter (subset ของ `Cargo.toml`) ได้ทันที โดยเบื้องหลังความสำเร็จนี้ ทีม Lang ได้แก้ปัญหาทางเทคนิคเล็กๆ แต่สำคัญ อย่างการจัดการตัวอักษร Stray Carriage-Return (`\r`) ใน source code ซึ่งเคยเป็น blocker ที่ทำให้ tooling ตีความ new line ผิดพลาด ตอนนี้ทุกอย่างได้รับการแก้ไขและพร้อมสำหรับการใช้งานจริงแทน Shell script ที่ซับซ้อน

## ความปลอดภัยของ Supply Chain

ในเรื่องความปลอดภัยของ Supply Chain ในระดับ Enterprise ทีมงานกำลังวางโครงสร้างระบบ Mirroring & Verification ใหม่บนพื้นฐานของ TUF (The Update Framework) เพื่อให้มั่นใจว่า Crate ที่ถูกดึงผ่าน Mirror Server ขององค์กรหรือ Linux Distro ต่างๆ นั้น เป็นไบนารีเดียวกับต้นฉบับจาก Crates.io 100% โดยไม่มีการปลอมแปลงระหว่างทาง เป็นการปิดช่องโหว่ความปลอดภัยในระดับ Infrastructure ให้แน่นหนายิ่งขึ้น

---

## เพิ่มเติม: cargo-script เจ๋งยังไง?

เผื่อใครสงสัยเกี่ยวกับ `cargo-script` ว่ามันเจ๋งยังไง

แต่เดิมถ้าอยากเทส library ตัวนึง หรือเขียน script สั้นๆ สิ่งที่เราจะต้องทำคือ `cargo new my-temp-project` (สร้าง folder structure) จากนั้นเข้าไปแก้ `Cargo.toml` เพื่อเพิ่ม dependency เพิ่มแล้วก็เข้าไปเขียนโค้ดใน `src/main.rs` และสั่ง `cargo run`

แต่ด้วย `cargo-script` เราสามารถสร้างไฟล์ `.rs` แค่ไฟล์เดียว แล้วรันได้เลย (เช่น `cargo ./hello-world.rs`) โดยไม่ต้องมี folder `src` หรือไฟล์ `Cargo.toml` แยก นอกจากนี้ทีเด็ดคือ "Front Matter" สิ่งที่ทำให้ `cargo-script` ต่างจากการเขียน script ทั่วไปคือ มันอนุญาตให้เราฝัง config ของ `Cargo.toml` ลงไปในส่วนหัวของไฟล์ `.rs` ได้เลย (Embedded Manifest) ตามโค้ดที่เป็นรูปภาพประกอบครับ

---

**Credit & Reference:**

1. [Inside Rust Blog - Program management update (January 2026)](https://blog.rust-lang.org/inside-rust/2026/02/11/program-management-update-2026-01/)

<!-- NAVIGATION:START -->
<!-- markdownlint-disable MD033 -->
<div class="article-nav">
  <a class="nav-left" href="../index.html">← Introduction</a>
  <a class="nav-right" href="./index.html">Category index →</a>
</div>
<!-- markdownlint-enable MD033 -->
<!-- NAVIGATION:END -->
