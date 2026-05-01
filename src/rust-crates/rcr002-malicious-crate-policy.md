# อัปเดตนโยบายแจ้งเตือน Malicious Crate บน crates.io

> 📅 วันที่เผยแพร่: 2026-02-14

ทีม crates.io ได้ประกาศเปลี่ยนแปลงนโยบายการแจ้งเตือน malicious crate โดยจะไม่มีการเขียนบล็อกโพสต์ทุกครั้งที่พบหรือได้รับรายงาน crate ที่เป็นอันตราย เนื่องจากในกรณีส่วนใหญ่ที่ผ่านมาไม่พบหลักฐานการใช้งานจริง การโพสต์แจ้งทุกครั้งจึงกลายเป็น noise มากกว่าจะเป็นสัญญาณเตือนที่เป็นประโยชน์

## การแจ้งเตือนผ่าน RustSec Advisory

อย่างไรก็ตาม เมื่อมีการลบ crate ที่มี malware ทางทีมจะเผยแพร่ RustSec advisory เสมอ สามารถติดตามได้ผ่าน RSS feed ของ RustSec ส่วนกรณีที่ crate นั้นมีการใช้งานจริงหรือมีการโจมตีจริง ทางทีมจะยังคงเผยแพร่ทั้งบล็อกโพสต์และ RustSec advisory พร้อมอาจมีการแจ้งผ่านช่องทางเพิ่มเติม เช่น social media หากเห็นสมควร

## กรณีศึกษา Malicious Crates ล่าสุด

สำหรับ malicious crates ที่ถูกลบล่าสุดตั้งแต่โพสต์ก่อนหน้าจนถึงปัจจุบัน ได้แก่ `finch_cli_rust`, `finch-rst` และ `sha-rst` ซึ่งถูกรายงานเมื่อวันที่ 9 ธันวาคม 2025 ว่าพยายามขโมย credentials โดยปลอมตัวเป็น crate `finch` และ `finch_cli` (RUSTSEC-2025-0150 ถึง 0152)

ต่อมาวันที่ 6 กุมภาพันธ์ `polymarket-clients-sdk` ถูกรายงานว่าพยายามขโมย credentials โดยปลอมตัวเป็น `polymarket-client-sdk` (RUSTSEC-2026-0010) และล่าสุดวันที่ 13 กุมภาพันธ์ `polymarket-client-sdks` ก็ถูกรายงานในลักษณะเดียวกัน (RUSTSEC-2026-0011) ทุกกรณีมีการลบ crate ปิดการใช้งานบัญชีผู้เผยแพร่ทันที และรายงานไปยัง upstream providers ที่เกี่ยวข้องแล้ว

---

**Credit & Reference:**

1. [crates.io: an update to the malicious crate notification policy](https://blog.rust-lang.org/2026/02/13/crates.io-malicious-crate-update/)

<!-- NAVIGATION:START -->
<!-- markdownlint-disable MD033 -->
<div class="article-nav">
  <a class="nav-left" href="../index.html">← Introduction</a>
  <a class="nav-right" href="./index.html">Category index →</a>
</div>
<!-- markdownlint-enable MD033 -->
<!-- NAVIGATION:END -->
