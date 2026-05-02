# Turso สถาปัตยกรรม SQLite ยุคใหม่ที่ถูก Rewrite ด้วย Rust

> 📅 วันที่เผยแพร่: 2026-01-29

เรามักยกย่อง SQLite ว่าเป็นงานศิลปะแห่งวงการ Software ด้วยยอดการ Deploy สูงที่สุดในโลกและความเสถียรระดับตำนาน แต่ภายใต้ความสำเร็จนั้น กลับมี Innovator’s Dilemma ซ่อนอยู่ Sylvain Kerkour (ผู้เขียน Black Hat Rust (ปูส้มโจรสลัดของเรา)) ได้วิเคราะห์ไว้อย่างน่าสนใจในบทความล่าสุดที่ผมนำมาอ้างอิงว่า ทำไม Database ที่เสถียรที่สุดโลกถึงเริ่มเจอปัญหาคอขวด และทำไม #Rust ถึงเป็นกุญแจดอกเดียวที่จะไขปัญหานี้ได้ ไม่ใช่แค่การเขียนใหม่เพื่อความเท่ แต่เป็นความจำเป็นทางวิศวกรรม

จุดเริ่มต้นคือความเสถียรครับ SQLite ต้นฉบับเขียนด้วย #C ซึ่งมีความเสี่ยงเรื่อง Memory Safety มหาศาล ทีมพัฒนาจึงต้องแลกมาด้วย Test Suite ที่ใหญ่จนน่าตกใจ โค้ดจริงมีแค่ ~155,000 บรรทัด แต่ Test code มีถึง ~92 ล้านบรรทัด (มากกว่าโค้ดถึง 590 เท่า) ปัญหาคือ Test Suite เหล่านี้ไม่ได้ Open source ทำให้การที่คนนอกจะเข้าไปแก้ Core Logic หรือเพิ่มฟีเจอร์ใหม่ๆ อย่างปลอดภัยแทบจะเป็นไปไม่ได้ บวกกับธรรมชาติของ #C ที่เป็น Weakly typed และไม่มี Memory safety guarantees ทำให้ SQLite ติดอยู่ในกรอบเดิมที่ไม่สามารถรองรับ Modern workload อย่าง Concurrent writes ได้อย่างมีประสิทธิภาพ

นี่คือจุดที่ Turso เข้ามาแก้ไขด้วยการ Rewrite Engine ใหม่ด้วย #Rust ครับ สิ่งที่น่าสนใจในเชิง Technical ไม่ใช่แค่การเปลี่ยนภาษา แต่คือการที่ #Rust เอื้อให้ Architect สามารถรื้อโครงสร้าง I/O ใหม่ได้หมดจด Turso ยังคงเคารพ SQLite File Format (B+Trees on disk) ทำให้คุยกับไฟล์เดิมรู้เรื่อง แต่ไส้ในถูกเปลี่ยนจาก Synchronous I/O แบบเดิม มาใช้ Async I/O ผ่าน `io_uring`ของ Linux ตั้งแต่รากฐาน ซึ่งการทำสิ่งนี้ใน #C บน Codebase เก่านั้นเสี่ยงเกินไป แต่ระบบ Type System และ Borrow Checker ของ #Rust ทำให้ทีม Turso กล้าที่จะ Implement ระบบที่ซับซ้อนอย่าง MVCC (Multi-Version Concurrency Control) เข้าไป ส่งผลให้ Turso รองรับ Concurrent Writes ได้จริง ซึ่งเป็นสิ่งที่ SQLite เดิมทำไม่ได้ (หรือทำได้ยากมาก)

นอกจากเรื่อง Performance แล้ว #Rust ยังช่วยยกระดับ Security และ Architecture ไปอีกขั้น ในขณะที่ SQLite เดิมต้องพึ่งพา OS หรือ 3rd party library ในการจัดการเรื่อง Encryption แต่ Turso สามารถใส่ Built-in Encryption มาให้เป็น Default (table stakes สำหรับปี 2020+) และด้วยความยืดหยุ่นของ Rust Abstractions ทำให้ Turso ฉีกข้อจำกัดเดิมที่ SQLite เป็นได้แค่ In-process library ให้กลายเป็น Database ที่ Scalable ได้จริง คือเริ่มจาก In-process ในแอปเล็กๆ และเมื่อสเกลก็สามารถแปลงร่างเป็น Networked Database (Server-Client) ได้โดยไม่ต้องเปลี่ยน Engine หรือ Migrate ไปใช้ Postgres ให้วุ่นวาย ตอบโจทย์ยุค AI Agents ที่ต้องการ Sandbox Database เฉพาะตัวที่เบาแต่แรง

ความเหนือชั้นของ #Rust ยังสะท้อนผ่าน Extension System ของ Turso ครับ ใครที่เคยเขียน Extension ให้ SQLite ใน #C จะรู้ว่าต้องจัดการกับ Pointer และ Memory เองอย่างระมัดระวัง แต่ใน #Rust ทาง Turso ได้เตรียม SDK ที่ใช้พลังของ Procedural Macros เข้ามาช่วย ตัวอย่างเช่นการเขียนฟังก์ชัน Crypto Extension เราสามารถใช้ Macro `#[scalar]` แปะบนฟังก์ชัน #Rust ธรรมดา แล้ว Macro จะจัดการเรื่อง Type conversion และ FFI binding ให้เบื้องหลัง ทำให้โค้ด Clean, Safe และ Maintainable กว่า #C แบบคนละเรื่อง

สรุปแล้วว่า การกำเนิดของ Turso คือบทพิสูจน์ที่ชัดเจนว่า #Rust คือ New Standard ของ System Programming อย่างแท้จริง มันเข้ามาแก้ Pain point ของภาษา #C ในจุดที่วิกฤตที่สุด คือความสามารถในการ Maintain และ Evolve ซอฟต์แวร์ที่มีความซับซ้อนสูง ให้สามารถรองรับ Modern Requirements อย่าง Async I/O และ Concurrency ได้อย่างปลอดภัย โดยไม่ต้องแลกมาด้วยความเสี่ยงเรื่อง Memory Corruption ครับ

**Credit & Reference:**

1. [Deep dive into Turso, the "SQLite rewrite in Rust" by Sylvain Kerkour (Jan 2026)](https://kerkour.com/turso-sqlite)
2. [Turso GitHub repo](https://github.com/tursodatabase/turso)
3. [Turso Documentation](https://docs.turso.tech/introduction)

<!-- NAVIGATION:START -->
<!-- markdownlint-disable MD033 -->
<div class="article-nav">
  <a class="nav-left" href="../index.html">← Introduction</a>
  <a class="nav-right" href="./index.html">Category index →</a>
</div>
<!-- markdownlint-enable MD033 -->
<!-- NAVIGATION:END -->
