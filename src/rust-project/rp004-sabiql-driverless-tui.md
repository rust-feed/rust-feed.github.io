# "sabiql" TUI ที่ไม่ง้อ Database Driver

> 📅 วันที่เผยแพร่: 2026-03-06

สำหรับใครที่เป็นสาย terminal แล้วทำงานกับ PostgreSQL ทุกวัน คงเคยผ่านความรู้สึกนี้มาแล้ว เปิด DBeaver รอ splash screen โหลด กดคลิกผ่านหน้าต่างซ้อนหน้าต่าง แล้วก็วนกลับมาที่ psql เหมือนเดิม เพราะมันเร็วกว่า riii111 ก็เจอปัญหานี้เหมือนกัน เลยตัดสินใจสร้าง sabiql ขึ้นมาเอง TUI สำหรับ browse, query และ edit PostgreSQL เขียนด้วย Rust + Ratatui โดยมีหลักการง่ายๆ ว่า ถ้ามี psql อยู่ในเครื่องแล้ว ก็ควรจะรันได้เลย ไม่ต้องติดตั้ง database driver หรือ config connection pool เพิ่มอีกแม้แต่นิดเดียว

## สถาปัตยกรรม Driver-less และการจัดการ Overhead

แนวคิดที่ทำให้ "sabiql" แตกต่างจาก TUI tool อื่นคือสถาปัตยกรรม driver-less อย่างแท้จริง แทนที่จะ link กับ libpq โดยตรง มันเลือก spawn psql เป็น subprocess สำหรับทุก query เลย trade-off ที่ตามมาคือ spawn overhead สะสมเร็วมาก implementation แรกของ auto-completion engine รัน 6 queries ต่อตาราง ฐานข้อมูลที่มี 538 ตารางนั่นหมายถึง 3,228 psql spawns แค่ตอน startup เดียว CPU spike ขึ้น 3-5 วินาที

วิธีแก้คือ consolidate เป็น query เดียวที่ดึงเฉพาะ columns กับ FK ที่ใช้จริง แล้วเสริมด้วย two-tier caching Tier 1 เป็น TTL cache สำหรับ table list กับ schema info, Tier 2 เป็น LRU cache สำหรับ per-table details ที่ SQL completion กับ ER diagram ใช้ร่วมกัน ผลที่ได้คือ startup time ลดเหลือต่ำกว่า 100ms

## ฟีเจอร์ที่โดดเด่นของ Sabiql

ฟีเจอร์ที่ไม่เคยเห็นใน TUI database tool ตัวไหนมาก่อนคือ ER diagram กด e แล้วมันเปิด diagram ใน browser ให้เลย รองรับ generate แบบ focused เฉพาะตารางที่ต้องการด้วย นอกจากนี้ยังมี Inspector Pane ที่แบ่งเป็น 7 แท็บครอบคลุมตั้งแต่ Columns, Indexes, Foreign Keys ไปจนถึง RLS policies และ auto-generated DDL ที่งัดออกมาได้เลย

แก้ไข cell in-place ก็ได้ โดยมี safety guard ป้องกัน mass-update อัตโนมัติถ้าไม่มี WHERE clause และลบ row ได้แบบ vim-style dd แล้ว :w พร้อม SQL preview ก่อน execute เสมอ

## การใช้งานแบบ Vim-like และการตอบรับจากชุมชน

การใช้งานก็ vim-like ตลอด ทั้ง j/k สำหรับ scroll, g/G สำหรับ jump, f สำหรับ focus mode เต็มจอ และ Ctrl+K สำหรับ command palette config เก็บไว้ที่ `~/.config/sabiql/connections.toml` รองรับ pg_service.conf ที่มีอยู่แล้วด้วย install ได้เลยผ่าน `cargo install sabiql`

หลังปล่อยออกมาได้รับ shout-out จาก orhun (Ratatui maintainer) ติดรายการ awesome-ratatui และ Postgres Weekly ก็หยิบไปนำเสนอ เป็นสัญญาณที่ชัดเจนว่า community กำลังรอ tool แบบนี้อยู่ครับ

---

**Credit & Reference:**

1. [I built a fast, driver-less TUI for browsing PostgreSQL, with ER diagrams](https://dev.to/riii111_8782a7e607d04c8e8/i-built-a-fast-driver-less-tui-for-browsing-postgresql-with-er-diagrams-50k9)
2. [riii111/sabiql — GitHub](https://github.com/riii111/sabiql)

<!-- NAVIGATION:START -->
<!-- markdownlint-disable MD033 -->
<div class="article-nav">
  <a class="nav-left" href="../index.html">← Introduction</a>
  <a class="nav-right" href="./index.html">Category index →</a>
</div>
<!-- markdownlint-enable MD033 -->
<!-- NAVIGATION:END -->
