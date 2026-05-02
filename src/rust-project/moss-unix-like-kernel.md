# "moss" Unix-like Kernel ที่เขียนด้วย Rust

> 📅 วันที่เผยแพร่: 2026-02-14

มีโปรเจคน่าสนใจมาแนะนำ moss (Linux-compatible kernel) ที่เขียนด้วย #Rust และ Aarch64 assembly

จุดเด่นที่ทำให้ moss แตกต่างคือการใช้ `async/await` ใน kernel context ครับ system calls ทั้งหมดเป็น async functions ทำให้ compiler ช่วย enforce ว่าไม่สามารถ hold spinlock ขณะ sleep ได้ ซึ่งช่วยกำจัด class ของ bugs ที่เป็น deadlocks ได้ที่ต้นเหตุ แถม future ไหนก็ได้สามารถห่อด้วย `.interruptable()` combinator เพื่อให้ signals สามารถขัดจังหวะการรอได้อย่างถูกต้อง

ปัจจุบัน moss สามารถรัน Arch Linux aarch64 userspace ได้จริง รวมถึง bash, BusyBox, coreutils, ps, top และ strace implement ไปแล้ว 105 Linux syscalls พร้อม SMP scheduling ผ่าน EEVDF, fork/execve/clone, signal handling และ ptrace support ที่เพียงพอต่อการรัน strace บน Arch binaries

ส่วนที่ผมชอบเป็นพิเศษคือการออกแบบ libkernel ที่แยก architecture-agnostic logic ออกมา ทำให้เทสได้บน host machine (x86) ก่อนรันบน bare metal ตอนนี้มี test suite 230+ tests เลยครับ

โปรเจคยัง active development อยู่ roadmap มี TCP/IP stack, read/write filesystem driver, และ systemd bringup ใครสนใจ OS dev, async Rust หรืออยาก contribute (port x86, เพิ่ม syscalls, เขียน driver) แวะดูได้ครับ

**Credit & Reference:**

1. [moss GitHub repo](https://github.com/hexagonal-sun/moss-kernel)

<!-- NAVIGATION:START -->
<!-- markdownlint-disable MD033 -->
<div class="article-nav">
  <a class="nav-left" href="../index.html">← Introduction</a>
  <a class="nav-right" href="./index.html">Category index →</a>
</div>
<!-- markdownlint-enable MD033 -->
<!-- NAVIGATION:END -->
