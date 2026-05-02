# ทำไมเวลาใช้ AI เขียนโค้ด จึงควรเขียนด้วย Rust (และ Rust เท่านั้น 😉)

> 📅 วันที่เผยแพร่: 2026-01-28

ในปี 1959 วิศวกรของ Volvo ได้ประดิษฐ์เข็มขัดนิรภัยแบบ 3 จุดขึ้นมาเพื่อเปลี่ยนโลกยานยนต์ Patrick Gray (Staff Engineer @ baselayer.com) ได้หยิบยกเรื่องราวนี้มาเปรียบเทียบใน Session "AI should write Rust and only Rust" ได้อย่างน่าสนใจว่า ในยุคปัจจุบันที่ AI เปรียบเสมือน "รถยนต์ไร้คนขับ" (Self-driving cars) ที่มีความสามารถสูงแต่ยังมีความผิดพลาดโดยธรรมชาติ (Inherently fallible) เราไม่ได้ต้องการแค่คนขับที่เก่งขึ้น แต่เราต้องการ "เข็มขัดนิรภัย" ที่ดีที่สุด และสำหรับโลก Software Development เข็มขัดเส้นนั้นคือ Rust Compiler

นี่คือ Technical Deep Talk จากโปรเจกต์ `fpexif` และ `MADstack` ที่พิสูจน์ว่าทำไม Rust ถึงเป็นภาษาที่เหมาะสมที่สุดในการทำงานร่วมกับ AI

จุดเริ่มต้นคือความต้องการสร้าง Library สำหรับ Parse RAW Image Metadata (เช่น .CR2, .RAF, .NEF) เพื่อแทนที่ Legacy Tools อย่าง `exiv2` (C++) และ `exiftool` (Perl) ที่ Robin Mills (ผู้สร้าง exiv2) เองยังยอมรับว่าควรถูก Rewrite ด้วย #Rust เพื่อความปลอดภัย (Memory Safety)

โจทย์นี้ Patrick เรียกว่าเป็น "Embarrassingly AI-solvable problem" คือเป็นงานที่มี Input/Output ชัดเจน มีปริมาณมหาศาล (Mapping tags นับพัน) แต่ถึกและน่าเบื่อเกินกว่ามนุษย์จะทำ เขาจึงใช้ Claude Code เข้ามาจัดการ โดยใช้แนวคิด Test-Driven Development (TDD) on Steroids ประกอบด้วย Ground Truth ใช้ `raw.pixls.us` (Raw photos ~56GB) เป็น input Oracle รัน `exiftool` และ `exiv2` เพื่อเจน JSON output ออกมาเป็นเฉลย และ AI Loop ให้ AI เขียน Rust parser เพื่อให้ได้ output ตรงกับ Oracle

สิ่งที่ทำให้ #Rust โดดเด่นกว่า Python หรือ JavaScript ใน Loop นี้คือ Strictness ของ Compiler ในขณะที่พัฒนา Patrick พบว่าเมื่อใช้ AI เขียนโค้ด ภาษา dynamic มัก "fails late" (พังตอนรัน) แต่ #Rust บังคับให้ "Fail early" Patrick ใช้เทคนิค "Shift Left" อย่างสุดขั้ว โดยการโยน Error จาก `rustc` และ `cargo test` กลับเข้าไปใน Context ของ AI (ผ่าน Tooling ที่ชื่อว่า Ralph ซึ่งเป็น loop script ง่ายๆ เพื่อแก้ปัญหา AI หยุดทำงานกลางคัน)

ผลลัพธ์คือ AI ถูกบังคับให้แก้ปัญหาเรื่อง Memory Safety, Lifetimes, และ Type Mismatch จนกว่าจะ Compile ผ่าน ซึ่งเมื่อผ่านแล้ว โอกาสที่ Logic จะถูกต้องนั้นสูงมาก สถิติจากสไลด์โชว์ให้เห็นว่า `fpexif` สามารถทำ Match Rate กับ Fujifilm (RAF) ได้สูงถึง 90.3% และในบางเคส #Rust ให้ความแม่นยำของ Floating Point (เช่น Shutter Speed 1/3) ได้ดีกว่า Tool ต้นฉบับเสียอีก

ประเด็นทางเทคนิคที่น่าสนใจมากคือการจัดการ Token Budget ของ AI ในช่วงแรก Patrick พบว่า AI มักจะเขียนโค้ดซ้ำซ้อน (Boilerplate) หรือเปลี่ยน Implementation ไปมา (Flip-flopping) ทำให้เปลือง Token และควบคุมยาก ทางออกของเขาคือการเขียน `macro_rules!` ขึ้นมาเพื่อกำหนดโครงสร้าง (Structure) ของการ Map Tags และให้ AI เติมแค่ข้อมูลลงไปใน Macro นั้น วิธีนี้ลดปริมาณโค้ดที่ AI ต้องเขียน ลดโอกาส Hallucination และทำให้โค้ด Maintainable มากขึ้น

อีกโปรเจกต์ที่ยกมาคือ `MADstack` (Maud + Axum + Diesel) ซึ่งเป็นแนวคิดการสร้าง Web Framework ที่เป็นมิตรกับ AI
Patrick แชร์ประสบการณ์เจ็บปวดจากการใช้ Tera (Template engine คล้าย Jinja2) ที่ AI มักจะเขียน Template ผิดๆ ถูกๆ ทำให้เกิด Runtime Error บ่อยครั้ง เขาจึงเปลี่ยนมาใช้ `Maud` ซึ่งเป็น HTML Macro ใน #Rust

ทำไม Maud? เพราะ Maud ตรวจสอบ HTML Syntax ตั้งแต่ Compile Time หาก AI เขียน Tag ปิดไม่ครบ หรือใช้ Type ผิด Compiler จะด่าทันที นี่คือตัวอย่างของการใช้ Type System เป็น Constraint ให้ AI ทำงานอยู่ในลู่ในทาง

ในแง่ Performance โปรเจกต์นี้โชว์ศักยภาพของ #Rust ผ่านการใช้ `tokio` และ `futures::stream::StreamExt::inspect()` ในการ Slice เฉพาะส่วน Header ของไฟล์ RAW ขนาด 50MB+ เพื่อประมวลผลแบบ Streaming (Zero-copy logic) ซึ่ง Python ทำได้ไม่ดีเท่า

และที่พีคที่สุดคือ Patrick สามารถ Compile `fpexif` เป็น WebAssembly (WASM) เพื่อรันบน Browser ได้ทันที ทำให้ Client สามารถ Parse RAW file และดึง JPEG Preview ออกมาโชว์ได้โดยไม่ต้องส่งไฟล์ไป Server

Patrick ทิ้งท้ายไว้ว่า แม้ AI จะเก่งขึ้นเรื่อยๆ แต่มันก็ยังเป็น "รถยนต์ไร้คนขับ" ที่ไว้ใจไม่ได้ 100% การเลือกใช้ภาษาที่มี Type System แข็งแกร่งอย่าง #Rust จึงไม่ใช่การทำให้งานยากขึ้น แต่เป็นการ "คาดเข็มขัดนิรภัย" ให้กับ Software ของเรา

หากเราต้องการให้ AI ช่วยงานระดับ Production เราต้องใช้ภาษาที่ Compiler กล้าที่จะปฏิเสธโค้ดขยะของ AI ตั้งแต่เนิ่นๆ และนั่นคือเหตุผลที่ AI should write Rust (only Rust 😉)

**Credit & Reference:**

1. [AI should write rust and only rust 😉 — Patrick Gray — Seattle Rust User Group, January 2026](https://www.youtube.com/watch?v=2lhr-QDWv-k)

<!-- NAVIGATION:START -->
<!-- markdownlint-disable MD033 -->
<div class="article-nav">
  <a class="nav-left" href="../index.html">← Introduction</a>
  <a class="nav-right" href="./index.html">Category index →</a>
</div>
<!-- markdownlint-enable MD033 -->
<!-- NAVIGATION:END -->
