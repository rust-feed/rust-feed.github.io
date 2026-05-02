# RootNRoute ยกระดับ Rust สู่โลกของ Offensive & Defensive Security

> 📅 วันที่เผยแพร่: 2026-02-02

หากคุณเป็น Rust Developer ที่กำลังมองหาเนื้อหาที่ "Beyond Hello World" และอยากเห็นว่า #Rust สามารถนำไปใช้งานในระดับ System-level และ Security engineering ได้ลึกแค่ไหน ผมขอแนะนำให้รู้จักกับช่อง RootNRoute ของคุณ Diljit ครับ

## ทำไมถึงควรติดตามช่องนี้?

1. เจาะลึกเรื่อง Cryptography และ Engineering

   ช่องนี้มีเนื้อหาตั้งแต่พื้นฐานการทำ Symmetric Encryption อย่าง XOR และ AES-GCM ไปจนถึงการทำ Hybrid Encryption ด้วย RSA ซึ่งเป็นหัวใจสำคัญของระบบ Security สมัยใหม่

2. Project-based Learning สำหรับงาน Security

   - Offensive Rust: การสร้าง Shellcode Transformation Engine เพื่อทดสอบขีดความสามารถของ Antivirus/EDR
   - Ransomware Simulation: การจำลองการทำงานของ Ransomware ตั้งแต่การสร้าง Key Pair ไปจนถึงกระบวนการเข้ารหัสไฟล์ และการเขียน Decryptor สำหรับฝั่ง Blue Team
   - Web Security: การเขียน XSS Scanner ประสิทธิภาพสูง โดยใช้ Async Rust และ Tokio Runtime เพื่อจัดการ Concurrent Tasks

3. ความเข้าใจในระดับ Internals

   เนื้อหาครอบคลุมไปถึงกระบวนการทำงานของคอมไพเลอร์ (rustc), การจัดการ Memory (Ownership & Borrowing) และการทำ Binary Optimization (LTO, Opt-levels) เพื่อให้เครื่องมือที่สร้างขึ้นมีขนาดเล็กและรวดเร็วที่สุด

4. เนื้อหาในอนาคตที่น่าจับตา

   ทางช่องมีการวางแผนที่จะขยายเนื้อหาไปสู่เรื่องที่ลึกขึ้นอย่าง eBPF Programming, Linux Internals และการพัฒนา Mobile App ด้วย #Rust

## สไตล์การสอน

ช่องนี้เน้นการเรียนรู้ผ่านโปรเจกต์ (Project-based way) ซึ่งจะช่วยให้เราเห็นความสำคัญของฟีเจอร์ต่างๆ ใน #Rust เช่น Traits, Enums และ Dynamic Dispatching ผ่านการแก้ปัญหาที่เกิดขึ้นจริงระหว่างการเขียนโปรแกรม

สำหรับใครที่อยากเห็นศักยภาพของ #Rust ในมุมมองของ Ethical Hacker และ Security Researcher ช่อง RootNRoute คือทรัพยากรชั้นดีที่ควรค่าแก่การ Subscribe ไว้ครับ

**Credit & References:**

1. [RootNRoute](https://www.youtube.com/@rootnroute)

<!-- NAVIGATION:START -->
<!-- markdownlint-disable MD033 -->
<div class="article-nav">
  <a class="nav-left" href="../index.html">← Introduction</a>
  <a class="nav-right" href="./index.html">Category index →</a>
</div>
<!-- markdownlint-enable MD033 -->
<!-- NAVIGATION:END -->
