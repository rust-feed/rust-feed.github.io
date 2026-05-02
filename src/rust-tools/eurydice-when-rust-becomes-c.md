# Eurydice เมื่อ Rust ถอดร่างเป็น C

> 📅 วันที่เผยแพร่: 2026-01-31

ในฐานะ #Rustacean เราน่าจะคุ้นเคยกับการที่ `rustc` ทำหน้าที่เป็นผู้คุมกฎที่เข้มงวด ก่อนจะส่งไม้ต่อให้ LLVM แปลงตรรกะอันซับซ้อนของเราให้กลายเป็น Machine Code ที่ดุดัน แต่เมื่อไม่นานมานี้ โลกของ Compiler ได้ต้อนรับผู้เล่นใหม่อย่าง Eurydice (โปรเจกต์ภายใต้ Aeneas จาก Inria และ Microsoft) ที่ไม่ได้พยายามจะสร้าง binary ให้เร็วที่สุด แต่กลับมีเป้าหมายที่ทะเยอทะยานในเชิงวิศวกรรมซอฟต์แวร์ นั่นคือการแปลง #Rust ให้กลายเป็น "Readable C" โดยที่ยังคงโครงสร้างและ Semantics ของต้นฉบับไว้อย่างครบถ้วน

คำถาม ทำไมเราถึงต้องการถอยหลังกลับไปสู่ C (นั่นสิ) คำตอบไม่ได้อยู่ที่ประสิทธิภาพ แต่อยู่ที่ "ความเชื่อมั่น" ในอุตสาหกรรมที่ซีเรียสเรื่องความปลอดภัยระดับสูงสุด เครื่องมือประเภท Formal Verification หรือ Compliance Tools จำนวนมากถูกสร้างขึ้นมาเพื่อวิเคราะห์ภาษา C มานานนับทศวรรษ การจะรอให้โลกทั้งใบหมุนตาม #Rust ทันทีอาจเป็นเรื่องยาก Eurydice จึงอาสาเป็นสะพานเชื่อม โดยอนุญาตให้เราเขียน Logic ด้วย Memory Safety Model ของ #Rust แต่สามารถ export ออกมาเป็น C code ที่ tools เหล่านี้อ่านรู้เรื่อง และนำไป verify ต่อได้ทันที ซึ่งต่างจาก `mrustc` ที่เน้นแค่การ bootstrap จนได้ C code ที่เต็มไปด้วย bit-twiddling ที่มนุษย์อ่านไม่รู้เรื่อง

ความน่าสนใจเชิงเทคนิคเริ่มตั้งแต่ Architecture ของมัน Eurydice ไม่ได้เขียน Parser เองให้เสียเวลา แต่ทำงานร่วมกับเครื่องมืออีกตัวชื่อ Charon โดย Charon จะเข้าไปดึง MIR (Medium-level Intermediate Representation) จาก `rustc` แล้ว dump ออกมาเป็น JSON ทำให้มั่นใจได้ว่า Code ที่ถูกนำมาแปลงนั้นผ่านการ Type Check และ Borrow Check จาก Rust compiler มาเรียบร้อยแล้ว จากนั้น Eurydice จะแปลง JSON นี้เข้าสู่ IR ของ KaRaMeL (Tool ที่ใช้แปลงภาษา F*) แล้วค่อย generate ออกมาเป็น C (ซับซ้อนเลยทีเดียว)

แต่ความท้าทายที่แท้จริง คือการจัดการกับ "Semantics Mismatch" ระหว่างสองภาษา ยกตัวอย่างเช่นเรื่อง Evaluation Order ใน #Rust รับประกันลำดับการทำงานจากซ้ายไปขวาที่ชัดเจน แต่ C นั้นเต็มไปด้วย Undefined Behavior ในเรื่องลำดับการ evaluate arguments สมมติเรามีฟังก์ชันคำนวณ LCM ที่เขียนว่า `(a * b) / gcd(a, b)` หากการคูณ `a * b` ทำให้เกิด Panic (Overflow) #Rust รับประกันว่าโปรแกรมจะ Panic ก่อนที่จะเรียกฟังก์ชัน `gcd` เสมอ

เพื่อรักษาพฤติกรรมนี้ Eurydice จะไม่แปลงเป็น one-liner ใน C ตรงๆ แต่จะ inject ตัวแปรชั่วคราว (เช่น `uint64_t uu____0`) เข้ามาเพื่อ force execution order ให้การคูณเกิดขึ้นก่อนการหารและการเรียกฟังก์ชันเสมอ แม้ชื่อตัวแปรอาจจะดูขัดตา แต่นี่คือราคาที่ต้องจ่ายเพื่อความถูกต้องที่แม่นยำ

เรื่องราวยิ่งซับซ้อนขึ้นเมื่อเราพูดถึง Dynamically Sized Types (DSTs) และ Generics เพราะ C ไม่มี concept ของ Generics Eurydice จึงต้องทำ Monomorphization สร้างฟังก์ชันแยกตาม Type ที่ใช้งานจริง แต่สิ่งที่น่าทึ่งคือการจัดการกับ Struct ที่มี field สุดท้ายเป็น `?Sized` (เช่น Slice) ซึ่งใน C มักจะแทนด้วย Flexible Array Member (FAM) หาก Eurydice ตรวจพบว่า Code #Rust ส่วนนั้นมีการระบุขนาด Array ที่แน่นอน (เช่น `[u8; 4]`) มันจะฉลาดพอที่จะ generate C struct แบบ fixed-size ออกมา และทำการ Elide Bounds Checks (ตัดการเช็ค index) ทิ้งไปเลย เพราะถือว่าผ่านการพิสูจน์ความปลอดภัยจากฝั่ง #Rust มาแล้ว

อย่างไรก็ตาม ความพยายามในการแปลง DST ไปมาระหว่างแบบ Flexible และแบบ Fixed-size นี้ ในทางเทคนิคถือว่าละเมิดกฎ Strict Aliasing ของภาษา C ซึ่งเป็นประเด็นละเอียดอ่อนมาก Jonathan Protzenko ผู้พัฒนาหลักจึงแนะนำให้ compile C ที่ได้จาก Eurydice ด้วย flag `-fno-strict-aliasing` เพื่อป้องกันไม่ให้ C Compiler ทำ optimization ที่ผิดพลาด นี่คือตัวอย่างที่ชัดเจนว่าการเขียน Compiler-to-Compiler translator นั้นต้องเข้าใจลึกซึ้งถึง Undefined Behavior ของทั้งภาษาต้นทางและปลายทาง

แม้ในปัจจุบัน (อ้างอิงข้อมูลเดือนมกราคม ปี 2026) Eurydice จะยังมีข้อจำกัดเรื่องการรองรับฟีเจอร์ใหม่ๆ อย่าง `const generics` และเหมาะกับโปรเจกต์ขนาดเล็กแบบ Self-contained (เช่น Cryptography routines) มากกว่าระบบใหญ่ยักษ์ อีกทั้งตัวมันเองยังเขียนด้วย #OCaml ซึ่งอาจเพิ่ม Dependency ในการ build แต่มันก็ได้พิสูจน์ให้เห็นแล้วว่า การนำ "Safety" ของ #Rust ไปสู่ Ecosystem เดิมของ C โดยไม่สูญเสีย "Readability" นั้นเป็นไปได้ และเป็นอีกก้าวสำคัญที่ทำให้ #Rust แทรกซึมเข้าไปในพื้นที่ High-assurance software ได้ลึกซึ้งยิ่งขึ้นครับ

**Credit & Reference:**

1. [Compiling Rust to readable C with Eurydice](https://lwn.net/SubscriberLink/1055211/0c358474dee845ec/)
2. [Eurydice GitHub repo](https://github.com/AeneasVerif/eurydice)

<!-- NAVIGATION:START -->
<!-- markdownlint-disable MD033 -->
<div class="article-nav">
  <a class="nav-left" href="../index.html">← Introduction</a>
  <a class="nav-right" href="./index.html">Category index →</a>
</div>
<!-- markdownlint-enable MD033 -->
<!-- NAVIGATION:END -->
