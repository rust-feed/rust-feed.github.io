# สถาปัตยกรรม Compilation Pipeline ของ Rust

## 1. บทนำ — ทำไมเรื่องนี้ถึงสำคัญ

สำหรับ Rust Developer การสั่ง `cargo build` เพื่อให้ได้ Executable file ที่ทำงานได้รวดเร็วและปลอดภัยแบบ Memory-safe โดยไม่ต้องอาศัย Garbage Collector ถือเป็นเรื่องปกติในชีวิตประจำวัน แต่ก่อนที่เราจะได้ไฟล์ที่ทำงานได้อย่างสมบูรณ์แบบนั้น Source Code ที่เราเขียนต้องผ่านกระบวนการอะไรบ้าง?

ภายใต้ความเรียบง่ายของการสั่ง Build สถาปัตยกรรมของ `rustc` (Rust Compiler) ถูกออกแบบมาอย่างเป็นระบบผ่าน Multi-stage Pipeline ที่ทำหน้าที่ตรวจสอบโค้ดอย่างเข้มงวดในแต่ละระดับ ก่อนหน้านี้มักมีประเด็นเรื่อง "Compiler ช้า" บทความนี้จะพาดำดิ่งไปดูว่า Compiler ทำอะไรบ้างกว่าโค้ดของเราจะกลายเป็น Machine Code

---

## 2. แนวคิดพื้นฐาน (Mental Model)

กระบวนการ Compile ของ Rust ไม่ได้แปลง Source Code ไปเป็น Machine Code ในรวดเดียว แต่จะผ่านขั้นตอนการ Lowering (การลดระดับโครงสร้าง) เป็นระยะๆ โดยในแต่ละระยะ (Intermediate Representation) จะมีหน้าที่การตรวจสอบและวิเคราะห์ที่แตกต่างกันไป

> **Summary Pipeline**
>
> `Cargo` ➔ `rustc` ➔ `AST` ➔ `HIR` (Type Checking) ➔ `MIR` (Borrow Checking) ➔ `LLVM IR` ➔ `Optimization` ➔ `Executable`

---

## 3. เจาะลึกกลไกภายใน — Multi-stage Pipeline

### 3.1 Front-end: Parsing และ AST

กระบวนการเริ่มต้นที่ระดับ Front-end Compiler จะรับ Raw text code เข้ามาทำกระบวนการ Parsing โดยแยกแยะองค์ประกอบออกเป็น **Tokens** (เช่น Keywords และ Identifiers ต่างๆ) จากนั้นนำมาจัดเรียงเป็นโครงสร้าง **Abstract Syntax Tree (AST)**

ในขั้นตอนนี้ Compiler จะจัดการกับ Expansion ด้วย เช่น การขยาย Macros หรือการทำ Desugaring โครงสร้างระดับสูงอย่าง `for-loops` ให้อยู่ในรูปแบบที่ Compiler จัดการได้ง่ายขึ้น

---

### 3.2 Lowering to HIR (High-Level Intermediate Representation)

เนื่องจาก AST ยังยึดติดกับรูปแบบ Syntax ของ Source code มากเกินไป `rustc` จึงทำการ "Lower" AST ลงมาเป็น **HIR**

ใน State นี้ Compiler จะเริ่มทำ Semantic Analysis เบื้องต้น ได้แก่:

- **Name Resolution:** สแกนหาและระบุตัวแปรหรือฟังก์ชันทั้งหมดใน Scope
- **Type Checking:** ตรวจสอบความถูกต้องของการใช้งาน Types

หากมี Typo หรือ Type Mismatch จะถูก Report Error และแจ้งเตือนนักพัฒนาตั้งแต่ขั้นตอนนี้

---

### 3.3 Lowering to MIR (Mid-Level Intermediate Representation)

เมื่อผ่านการตรวจสอบเบื้องต้น HIR จะถูก Lower ลงอีกขั้นกลายเป็น **MIR** ซึ่งออกแบบมาเฉพาะสำหรับการทำ Complex Analysis

> **Key Insight: The Heart of Rust Safety**
>
> MIR คือหัวใจสำคัญของภาษา Rust เพราะมันทำหน้าที่เป็น Input หลักให้กับ **Borrow Checker** ในขั้นตอนนี้ `rustc` จะบังคับใช้กฎ Ownership, Borrowing และ Lifetimes อย่างเคร่งครัด

นอกจากนี้ยังมีกระบวนการ **Lifetime Elision** ที่ Compiler จะพยายามอนุมาน (Infer) ความสัมพันธ์ของ Lifetime โดยอัตโนมัติ เพื่อลดภาระการเขียน Generic Lifetime Annotations ที่ไม่จำเป็น การวิเคราะห์แบบ Static ในระดับ MIR นี้เอง ที่ทำให้ Rust สามารถป้องกันปัญหา Dangling References และการันตี Memory Safety ได้ตั้งแต่ตอน Compile

---

### 3.4 Backend: LLVM IR และ Executable

เมื่อ Code ผ่าน Verification ด้านความปลอดภัยทั้งหมดจาก MIR แล้ว `rustc` จะทำหน้าที่ประหนึ่ง Front-end ที่ส่งต่อหน้าที่ให้กับ Backend อย่าง LLVM

MIR จะถูกแปลงไปเป็น **LLVM IR** (Intermediate Representation ของ LLVM) จากนั้น LLVM จะทำหน้าที่รัน Optimization Passes จำนวนมาก เพื่อปรับแต่งโครงสร้างโค้ดให้ทำงานได้รวดเร็วและใช้ทรัพยากรอย่างมีประสิทธิภาพที่สุด นี่คือเหตุผลหลักเบื้องหลัง Performance ในระดับ C/C++ ของโปรแกรม Rust

ในขั้นตอนสุดท้าย **Linker** จะรับ Object code ที่ถูกสร้างขึ้น นำมาประกอบ (Bundle) รวมกับ Libraries อื่นๆ ที่เกี่ยวข้อง (เช่น ไฟล์ Archive `.rlib` จาก dependencies ต่างๆ) เพื่อประกอบออกมาเป็น Final Executable ที่สมบูรณ์พร้อมทำงาน

---

## 4. ผลกระทบต่อ Ecosystem

ในมุมมองของ Architecture การที่ `rustc` แยก State ของ Intermediate Representation (HIR/MIR) ออกจากกันอย่างชัดเจน ช่วยให้การจัดการกับ Control-flow graph สำหรับ Borrow Checker ทำได้โดยอิสระ

การออกแบบสถาปัตยกรรมแบบนี้ทำให้เรามี Ecosystem ของ Tools เชิงลึกได้ เช่น `clippy` (ที่อาศัย HIR/MIR ในการวิเคราะห์ Lints) หรือ `rust-analyzer` ที่สามารถอาศัยโครงสร้างลำดับชั้นของ Compiler เพื่อให้ Feedback แบบเรียลไทม์ได้ดีขึ้น ก่อนที่จะโยนภาระหนักเรื่อง Optimization ไปให้ LLVM

---

## 5. ข้อควรระวังและ Trade-offs

ความรัดกุมของ Pipeline นี้ต้องแลกมาด้วยต้นทุนบางอย่าง:

> **Compile Time Trade-off**
>
> กระบวนการทำ Verification อย่างละเอียดที่ด่าน HIR และ MIR รวมถึง Optimization Passes จำนวนมหาศาลที่ฝั่ง LLVM เป็นสาเหตุหลักที่ทำให้ **Rust Compile ช้า** เมื่อเทียบกับภาษาอื่น

อย่างไรก็ตาม นี่คือ Trade-off ที่ชุมชน Rust ยอมรับได้ เพราะการเสียเวลาแก้ Compile Error เล็กๆ น้อยๆ ดีกว่าการปล่อยให้เกิด Runtime Error แบบเงียบๆ หรือช่องโหว่ทาง Memory ในระดับ Production

---

## 6. บทสรุป

จาก Pipeline ทั้งหมด จะเห็นได้ว่า Rust ถูกออกแบบมาให้เป็นมากกว่าแค่ภาษาโปรแกรม แต่มันคือ **"Rigorous Verification System"** หรือระบบตรวจสอบแบบรัดกุม

> **Key Takeaway**
>
> รหัส Executable สุดท้ายที่ออกจาก `cargo build` เป็นผลลัพธ์จาก Multi-stage Pipeline ที่บังคับให้โค้ดต้องปลอดภัย 100% ตั้งแต่ตอน Compile สิ่งที่ Compiler ทำไม่ใช่แค่การแปลงโค้ดลวกๆ แต่มันคือการพิสูจน์ตรรกะระดับลึก

นี่จึงเป็นเหตุผลว่าทำไมโค้ด Rust เมื่อ Compile ผ่านแล้วถึงมักจะทำงานได้ถูกต้องในทันที!

---

**Credit & Reference:**

1. [Understanding Rust Complier](https://dev.to/saurabh2836/understanding-rust-complier-3fgg)
2. [Overview of the compiler](https://rustc-dev-guide.rust-lang.org/overview.html)
