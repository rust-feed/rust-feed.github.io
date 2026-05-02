# สถาปัตยกรรม Compilation Pipeline ของ Rust

> 📅 วันที่เผยแพร่: 2026-02-22

สำหรับ Rust Developer การสั่ง `cargo build` เพื่อให้ได้ Executable file ที่ทำงานได้รวดเร็วและปลอดภัยแบบ Memory-safe โดยไม่ต้องอาศัย Garbage Collector ถือเป็นเรื่องปกติในชีวิตประจำวัน แต่ภายใต้กระบวนการนั้น สถาปัตยกรรมของ `rustc` (Rust Compiler) ถูกออกแบบมาอย่างเป็นระบบผ่าน Multi-stage Pipeline ที่ทำหน้าที่ตรวจสอบโค้ดอย่างเข้มงวดในแต่ละระดับ ก่อนหน้านี้มี topic compiler ช้า งั้นเรามาดูว่า Source Code ของเราผ่านกระบวนการอะไรบ้างกว่าจะกลายเป็น Machine Code เพื่อจะได้เห็นภาพว่า Compiler ทำอะไรบ้าง

กระบวนการเริ่มต้นที่ระดับ Front-end Compiler จะรับ Raw text code เข้ามาทำกระบวนการ Parsing โดยแยกแยะองค์ประกอบออกเป็น Tokens (เช่น Keywords และ Identifiers ต่างๆ) จากนั้นนำมาจัดเรียงเป็นโครงสร้าง Abstract Syntax Tree (AST) ในขั้นตอนนี้ Compiler จะจัดการกับ Expansion ด้วย เช่น การขยาย Macros หรือการทำ Desugaring โครงสร้างอย่าง `for-loops` ให้อยู่ในรูปแบบที่ Compiler จัดการได้ง่ายขึ้น

เนื่องจาก AST ยังยึดติดกับรูปแบบ Syntax ของ Source code มากเกินไป `rustc` จึงทำการ "Lower" (ลดระดับโครงสร้าง) AST ลงมาเป็น High-Level Intermediate Representation (HIR) ใน State นี้ Compiler จะเริ่มทำ Semantic Analysis เบื้องต้น ได้แก่ Name Resolution เพื่อสแกนหาและระบุตัวแปรหรือฟังก์ชันทั้งหมดใน Scope และ Type Checking ซึ่งจะตรวจสอบความถูกต้องของการใช้งาน Types หากมี Typo หรือ Type Mismatch จะถูก Report Error ตั้งแต่ขั้นตอนนี้

เมื่อผ่านการตรวจสอบเบื้องต้น HIR จะถูก Lower ลงอีกขั้นกลายเป็น Mid-Level Intermediate Representation (MIR) ซึ่งออกแบบมาเฉพาะสำหรับการทำ Complex Analysis MIR คือหัวใจสำคัญของ #Rust เพราะมันทำหน้าที่เป็น Input หลักให้กับ Borrow Checker ในขั้นตอนนี้ `rustc` จะบังคับใช้กฎ Ownership, Borrowing และ Lifetimes อย่างเคร่งครัด นอกจากนี้ยังมีกระบวนการ Lifetime Elision ที่ Compiler จะพยายามอนุมาน (Infer) ความสัมพันธ์ของ Lifetime โดยอัตโนมัติ เพื่อลดภาระการเขียน Generic Lifetime Annotations ที่ไม่จำเป็น การวิเคราะห์แบบ Static ในระดับ MIR นี้เอง ที่ทำให้ #Rust สามารถป้องกันปัญหา Dangling References และการันตี Memory Safety ได้ตั้งแต่ตอน Compile

เมื่อ Code ผ่าน Verification ด้านความปลอดภัยทั้งหมดจาก MIR แล้ว `rustc` จะทำหน้าที่ประหนึ่ง Front-end ที่ส่งต่อหน้าที่ให้กับ Backend อย่าง LLVM MIR จะถูกแปลงไปเป็น LLVM IR (Intermediate Representation ของ LLVM) จากนั้น LLVM จะทำหน้าที่รัน Optimization Passes จำนวนมาก เพื่อปรับแต่งโครงสร้างโค้ดให้ทำงานได้รวดเร็วและใช้ทรัพยากรอย่างมีประสิทธิภาพที่สุด (นี่คือเหตุผลหลักของ Performance ในโปรแกรม #Rust) ท้ายที่สุด LLVM จะแปล Optimized IR ให้กลายเป็น Machine Code (Object code) ที่เฉพาะเจาะจงกับ Architecture ของเครื่องปลายทาง

ในขั้นตอนสุดท้าย Linker จะรับ Object code ที่ถูกสร้างขึ้น นำมาประกอบ (Bundle) รวมกับ Libraries อื่นๆ ที่เกี่ยวข้อง (เช่น ไฟล์ Archive `.rlib` จาก dependencies ต่างๆ) เพื่อประกอบออกมาเป็น Final Executable ที่สมบูรณ์พร้อมทำงาน

Summary Pipeline
`Cargo` ➔ `rustc` ➔ `AST` ➔ `HIR` (Type Checking) ➔ `MIR` (Borrow Checking) ➔ `LLVM IR` ➔ `Optimization` ➔ `Executable`

จาก Pipeline ทั้งหมด จะเห็นได้ว่า #Rust ถูกออกแบบมาให้เป็นมากกว่าแค่ภาษาโปรแกรม แต่มันคือ "Rigorous Verification System" หรือระบบตรวจสอบแบบรัดกุม ที่บังคับให้โค้ดต้องปลอดภัย 100% ตั้งแต่ตอน Compile

ในมุมมองของ Architecture การที่ `rustc` แยก State ของ Intermediate Representation (HIR/MIR) ออกจากกันอย่างชัดเจน ช่วยให้การจัดการกับ Control-flow graph สำหรับ Borrow Checker ทำได้โดยอิสระ ก่อนที่จะโยนภาระหนักเรื่อง Optimization ไปให้ LLVM

**Credit & Reference:**

1. [Understanding Rust Complier](https://dev.to/saurabh2836/understanding-rust-complier-3fgg)
2. [Overview of the compiler](https://rustc-dev-guide.rust-lang.org/overview.html)

<!-- NAVIGATION:START -->
<!-- markdownlint-disable MD033 -->
<div class="article-nav">
  <a class="nav-left" href="../index.html">← Introduction</a>
  <a class="nav-right" href="./index.html">Category index →</a>
</div>
<!-- markdownlint-enable MD033 -->
<!-- NAVIGATION:END -->
