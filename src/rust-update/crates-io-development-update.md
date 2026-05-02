# crates.io: development update

> 📅 วันที่เผยแพร่: 2026-01-27

เมื่อวันที่ 21 มกราคม 2026 ทีมงาน crates.io ได้ปล่อย Development Update มีอะไรที่น่าสนใจบ้างมาดูกันครับ

เริ่มต้นที่หัวใจสำคัญที่สุดคือ Security การอัปเดตครั้งนี้ยกระดับ Supply Chain Security อย่างมีนัยสำคัญด้วยการผลักดัน Trusted Publishing ให้เป็นมาตรฐานใหม่ จากเดิมที่เราต้องคอยพะวงกับการจัดการ Long-lived API Tokens ที่เสี่ยงต่อการหลุดรั่ว ตอนนี้ crates.io ขยายการรองรับ OIDC-based authentication ไปยัง GitLab CI/CD แล้ว (นอกเหนือจาก GitHub Actions) แต่ความน่าสนใจในเชิง Implementation คือการ Refactor หลังบ้านให้รองรับ Multiple CI Providers ซึ่งเปิดทางให้แพลตฟอร์มอย่าง #Codeberg หรือ #Forgejo เข้ามาเชื่อมต่อได้ในอนาคต

ยิ่งไปกว่านั้น สำหรับ Crate ที่ต้องการความปลอดภัยสูงสุด เจ้าของสามารถเปิด "Trusted Publishing Only Mode" เพื่อบังคับปิดการ Publish ผ่าน Legacy Token แบบถาวร ตัดความเสี่ยงจาก Human Error หรือ Local Machine Compromise ได้อย่างเด็ดขาด รวมถึงการอุดช่องโหว่ระดับ CI โดยการ Block triggers อันตรายอย่าง `pull_request_target` และ `workflow_run` บน GitHub Actions เพื่อป้องกันการโจมตีแบบ Privilege Escalation ใน Pipeline

เมื่อมองลึกลงไปในส่วนของ Application Architecture ทีมงานกำลังทดลองไมเกรต Frontend ไปใช้ #Svelte (ทำไมถึงเลือก Svelte ?) แต่สิ่งที่สะท้อนแก่นแท้ของ Rust ไม่ใช่การเลือก UI Framework หากแต่เป็นวิธีการจัดการ Data Contract ระหว่าง Client-Server ทีมงานเลือกใช้ OpenAPI description ที่ Generate ตรงจาก Rust Backend เพื่อสร้าง API Client code ฝั่ง TypeScript แบบอัตโนมัติ ผลลัพธ์คือเราได้ระบบที่มี Type Safety ไหลลื่นข้าม Boundary (Rust types --> TypeScript types) ถ้า Backend Model เปลี่ยน Frontend จะรู้ทันทีตั้งแต่ Build time นี่คือสถาปัตยกรรมที่ลด Runtime Error และทำให้การ Refactor ระบบขนาดใหญ่ทำได้อย่างมั่นใจ ตามสไตล์ #Rustacean

ในระดับ Core Index มีการเพิ่มฟีเจอร์เล็กๆ แต่ทรงพลังอย่าง field `pubtime` เข้าไปใน Index Entry สิ่งนี้เปิดประตูสู่ความสามารถใหม่ของ Cargo ในการทำ "Time-travel dependency resolution" หรือการจำลองการ Resolve dependencies เสมือนว่าเราอยู่ในอดีต ซึ่งสำคัญมากสำหรับการ Debug หรือ Reproduce build ในสภาวะแวดล้อมที่ซับซ้อน

นอกจากนี้ในฝั่ง Observability หน้าเว็บยังมีการแสดงผล SLOC (Source Lines of Code) โดยใช้ `tokei` (Crate ยอดนิยมที่เขียนด้วย Rust สำหรับนับบรรทัดโค้ดที่เร็วจัดๆ) รันเป็น Background Job เพื่อให้ Developer ประเมินน้ำหนักของ Dependency (ละเอียดจัดๆ) ได้ก่อนตัดสินใจนำมาใช้ รวมถึงมีการปรับปรุง Data Integrity ด้วยการ Filter User Agent เพื่อกรอง Traffic จาก Bots/Mirrors ออก ทำให้สถิติดาวน์โหลดที่เห็นสะท้อนการใช้งานจริงจาก Cargo เท่านั้น

สุดท้ายคือเรื่องของ Scale ที่พิสูจน์ความแข็งแกร่งของ Infrastructure ตัวเลขล่าสุดระบุว่า `static.crates.io` ให้บริการข้อมูลถึง 1.6 Petabytes (ผ่าน 1.1 หมื่นล้าน Requests) ในเดือนเดียว ทีมงานแก้ปัญหาเรื่อง Scale และ Cost ด้วยการย้าย Sparse Index ไปอยู่บน Fastly CDN (รับ load ไปกว่า 740 TB) และในเชิงเทคนิค มีการปรับจูน Database Indexes เพื่อเพิ่ม Performance ของ Background Workers รวมถึงการแก้ปัญหา AWS Rate Limits เวลา Publish Workspace ขนาดใหญ่ ด้วยการทำ Batching CloudFront Invalidation ซึ่งเป็นการแก้ปัญหา Distributed Systems แบบคลาสสิกที่แสดงให้เห็นถึงความใส่ใจในรายละเอียดระดับ Low-level

การอัปเดตครั้งนี้ยืนยันว่า crates.io ไม่ได้เป็นเพียงที่ฝากไฟล์ แต่เป็นชิ้นส่วน Infrastructure ที่ถูกออกแบบด้วย Mindset เดียวกับภาษา #Rust คือ Safety, Speed, และ Correctness ตั้งแต่การจัดการ Memory ไปจนถึง HTTP Request และ Security Policy นี่คือมาตรฐานที่พวกเราในฐานะ Rust Developer ควรภูมิใจครับ

**Credit & Reference:**

1. [crates.io: development update](https://blog.rust-lang.org/2026/01/21/crates-io-development-update/)

<!-- NAVIGATION:START -->
<!-- markdownlint-disable MD033 -->
<div class="article-nav">
  <a class="nav-left" href="../index.html">← Introduction</a>
  <a class="nav-right" href="./index.html">Category index →</a>
</div>
<!-- markdownlint-enable MD033 -->
<!-- NAVIGATION:END -->
