# เมื่อความน่าจะเป็นของ AI ปะทะกับความเข้มงวดของ Rust บทเรียนจากการ Modernize ระบบ COBOL ด้วย Rust

> 📅 วันที่เผยแพร่: 2026-02-25

หากจะนำ COBOL ซึ่งเป็นภาษาที่ขับเคลื่อนโลกการเงินมานานกว่า 60 ปี มาแปลงเป็นภาษา Modern อย่าง Rust ด้วยพลังของ AI (Claude Opus) นั้น หลายคนอาจมองว่าความท้าทายอยู่ที่ความฉลาดของ Model หรือความแม่นยำในการแปลภาษา แต่จากกรณีศึกษาของคุณ Venkateshwar Rao Nagala ที่ได้ลงมือสร้างระบบนี้ขึ้นมา สิ่งที่ปรากฏชัดเจนยิ่งกว่าคือ ความโกลาหลของ AI ที่ต้องการระเบียบวินัยขั้นสูงสุดในการกำกับดูแล และนั่นคือจุดที่ Rust ก้าวเข้ามามีบทบาท ในฐานะโครงสร้างพื้นฐาน (Infrastructure) เพียงหนึ่งเดียวที่สามารถรับมือกับความไม่แน่นอนนี้ได้

## กรงขังที่ปลอดภัยสำหรับ AI Agents

โปรเจกต์นี้ถูกออกแบบบนสถาปัตยกรรม Model Context Protocol (MCP) โดยผู้เขียนได้แยกการทำงานออกเป็น 4 Microservices ที่เขียนด้วย Rust (Actix-web) ทั้งหมด หน้าที่ของมันไม่ใช่แค่การรับส่งข้อมูล แต่มันคือการสร้างกรงขังที่ปลอดภัยให้กับ AI Agents ในขณะที่ Python อาจเป็นภาษาแม่ของ AI แต่มันกลับยอมประนีประนอมมากเกินไปสำหรับงาน Infrastructure ที่ต้องจัดการกับ Critical Banking Logic

ผู้เขียนจึงเลือก Rust เพราะระบบนี้ต้องการ Type System ที่เข้มงวดและ Memory Safety ที่ปราศจากการรั่วไหล เพื่อให้มั่นใจว่าเมื่อ AI Agents ตัดสินใจเรียกใช้ Tools หลายตัวพร้อมกัน (Concurrency) ระบบจะยังคงสถานะที่ถูกต้อง (Correctness) และไม่มีทางเกิด Race Condition ที่คาดเดาไม่ได้

## Metaprogramming และ Dynamic Dependency

ความลึกซึ้งของ Rust ในโปรเจกต์นี้แสดงออกมาอย่างชัดเจนในส่วนของ Rust MCP Server ซึ่งทำหน้าที่รับ Code ที่ AI เจนเนอเรทออกมาไปคอมไพล์ ความท้าทายที่ซ่อนอยู่คือ AI มักจะจินตนาการ Dependencies ขึ้นมาเองตามบริบทของโค้ด เช่น การเรียกใช้ `rust_decimal` หรือ `num-format` โดยที่เราไม่สามารถ Hard-code `Cargo.toml` ไว้ล่วงหน้าได้

ปัญหานี้บังคับให้ผู้เขียนต้องใช้ Logic ในระดับ Metaprogramming เพื่อ Parse source code ที่ได้รับมา และทำการ Dynamic Dependency Injection เข้าไปในไฟล์ Config ก่อนสั่ง Build กระบวนการนี้หากทำในภาษาอื่นอาจเต็มไปด้วยความเสี่ยง แต่ Rust ช่วยให้การจัดการ String Manipulation และ File I/O เป็นไปอย่างรัดกุมและรวดเร็ว จนสามารถทำ Automated Compilation Pipeline ได้ในระดับ Sub-millisecond

## ครูผู้เข้มงวดชื่อ Rust Compiler

ในระหว่างการพัฒนา ความเข้มงวดของ Rust Compiler กลายเป็นทั้งกำแพงและครูผู้สอน โดยเฉพาะเมื่อต้องจัดการกับ Shared State ระหว่าง Async Threads ใน Actix-web ผู้เขียนเล่าถึงปัญหาคลาสสิกอย่าง `RwLock<Option<String>> doesn't implement Clone` ซึ่งไม่ใช่แค่ Error ที่น่ารำคาญ แต่มันคือการเตือนสติว่าเรากำลังพยายามเข้าถึงข้อมูลในลักษณะที่ไม่ปลอดภัย (Thread Safety)

ในขณะที่ภาษาอื่นอาจปล่อยให้โค้ดชุดนี้รันผ่านไปและไประเบิดทีหลังเมื่อมีโหลดสูงๆ Rust บังคับให้หยุดและออกแบบ Data Ownership ใหม่ตั้งแต่ต้น ผลลัพธ์คือเมื่อโค้ดผ่านการ Compile ทีมพัฒนาแทบจะมั่นใจได้ทันทีว่า Runtime Error จะเป็นศูนย์ ซึ่งเป็นคุณสมบัติที่ประเมินค่าไม่ได้สำหรับระบบ Enterprise

## Zero-Trust Layer ด้วย AgentGateway

แต่ส่วนที่สำคัญที่สุดของ Case Study นี้คือความปลอดภัยในยุคของ AI Agents เมื่อเราอนุญาตให้ "Purple Agent" (AI Modernizer) เข้าถึง S3 เพื่ออ่าน Source Code คำถามที่น่ากลัวคือ "จะเกิดอะไรขึ้นถ้า Agent ถูก Compromise หรือเกิด Hallucination?" ผู้เขียนจึงสร้าง AgentGateway ขึ้นมาด้วย Rust เพื่อทำหน้าที่เป็น Zero-Trust Layer ดักทุก Request

ระบบนี้ใช้ JWT Authentication และ Role-Based Access Control (RBAC) ที่ทำงานได้อย่างรวดเร็วด้วย Actix-web ที่รองรับ 8 Worker Threads การใช้ Pattern Matching (`match`) ของ Rust ในการจัดการ `Result<HttpResponse, Error>` ทำให้มั่นใจได้ว่าทุกความเป็นไปได้ของการเรียกใช้งาน ไม่ว่าจะสำเร็จหรือล้มเหลว จะถูกจัดการอย่างหมดจด ไม่มีช่องโหว่ให้หลุดรอดไปได้

## บทสรุป: ทางรอดเดียวของระบบการเงิน

ท้ายที่สุด การทดลองของคุณ Venkat ไม่ได้พิสูจน์แค่ว่า Rust สามารถทดแทน COBOL ได้ แต่มันแสดงให้เห็นปรัชญาที่ลึกซึ้งกว่านั้น ว่าในโลกที่ AI เต็มไปด้วยความน่าจะเป็น เราต้องการโครงสร้างพื้นฐานที่มีความเป็นเหตุเป็นผลอย่างสมบูรณ์มาคานอำนาจ

Rust ไม่ได้ถูกเลือกเพราะมัน "เร็ว" แต่ถูกเลือกเพราะมัน "ถูกต้อง" และเมื่อเดิมพันคือระบบการเงินของโลกที่ทำงานมากว่า 60 ปี การยอมแลกความสะดวกสบายในการเขียนโค้ดกับความถูกต้องแม่นยำของ Rust จึงไม่ใช่ทางเลือก แต่อาจเป็นทางรอดเดียวที่สมเหตุสมผล

---

**Credit & Reference:**

1. [Mainframe-Modernization GitHub repo](https://github.com/venkatnagala/Mainframe-Modernization)
2. [Mainframe Modernization COBOL to Rust with AgentGateway Solo io Hackathon 2026](https://www.youtube.com/watch?v=5s6MMIfxNf0)

<!-- NAVIGATION:START -->
<!-- markdownlint-disable MD033 -->
<div class="article-nav">
  <a class="nav-left" href="../index.html">← Introduction</a>
  <a class="nav-right" href="./index.html">Category index →</a>
</div>
<!-- markdownlint-enable MD033 -->
<!-- NAVIGATION:END -->
