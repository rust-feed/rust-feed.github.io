# เมื่อ Rust บุกโลก EVM: เจาะลึก Architecture ของ Arbitrum Stylus ผ่าน Claude Code Skill

> 📅 วันที่เผยแพร่: 2026-02-06

ในการพัฒนา Web3 มักมีช่องว่างขนาดใหญ่ที่น่าอึดอัดใจอยู่เรื่องหนึ่ง ฝั่งหนึ่งเรามี Tutorial ระดับ "Hello World" ที่สอนแค่ Deploy Counter Contract ง่ายๆ แล้วจบไป แต่อีกฝั่งคือ Production Codebase ขนาดมหึมาที่เต็มไปด้วย Configuration ซับซ้อนซึ่งต้องใช้เวลาหลายเดือนในการตกผลึก ปัญหาที่ Ben Greenberg (DevRel จาก Arbitrum) ค้นพบคือ เมื่อ Developer พยายามใช้ AI อย่าง Claude เพื่อปิดช่องว่างนี้ AI มักจะให้ Code ที่ "ดูเหมือนจะถูก" แต่กลับใช้ SDK เวอร์ชันเก่า หรือใช้ Pattern ที่พังทันทีเมื่อรันบนระบบใหม่อย่าง Arbitrum Stylus

นี่จึงเป็นที่มาของการสร้าง "Claude Code Skill" ชุดความรู้แบบ Structured Markdown ที่ไม่ได้แค่ Gen code แต่ทำหน้าที่เป็น Engineering Manager ที่คอยวาง Architecture ให้เราตั้งแต่ต้น และสิ่งที่น่าสนใจที่สุดสำหรับพวกเราชาว #Rust คือการที่ #Rust ถูกยกให้เป็น First-class citizen เคียงคู่กับ Solidity ในฐานะเครื่องมือสำหรับโปรเจกต์ที่ต้องการ "Maximum Performance" และ "Low Gas Cost"

หัวใจสำคัญที่ทำให้ #Rust เฉิดฉายบน EVM ผ่าน Arbitrum Stylus ได้นั้น อยู่ที่การจัดการความต่างของ Memory Model ระหว่าง #Rust และ EVM ซึ่งในเอกสาร (ที่ผมใช้อ้างอิง) ระบุชัดเจนถึงการใช้ `stylus-sdk` (โดยเฉพาะเวอร์ชัน 0.10+ ที่เสถียรต่อ breaking changes) เพื่อเข้ามาจัดการเรื่องนี้ ความงดงามทางเทคนิคเริ่มจาก Macro `sol_storage!` ซึ่งทำหน้าที่เป็น Abstraction Layer ในการ Map Rust Structs ให้ลงล็อกกับ Storage Slots ของ EVM (32-byte key-value pairs) ได้อย่างแนบเนียน

แทนที่เราจะต้องจัดการเรื่อง Slot Hashing เอง เราสามารถประกาศ `struct NftContract` และใช้ Type พิเศษอย่าง `mapping(uint256 => address)` ภายใน Macro ได้เลย สิ่งนี้ทำให้เราได้ Type Safety ตั้งแต่ Compile Time พร้อมกับประสิทธิภาพระดับ Native WASM ซึ่งเป็นสิ่งที่ Solidity แบบดั้งเดิมให้ไม่ได้

เมื่อเจาะลึกลงไปใน Implementation ของ Contract ผ่าน `impl` block เราจะเห็นความเข้มงวดที่เป็นเอกลักษณ์ของ #Rust เข้ามาช่วยกำจัด Bug ระดับ Low-level Attribute `#[entrypoint]` ถูกใช้เพื่อกำหนดจุดรับ Calldata จากภายนอก ในขณะที่ Logic ภายในฟังก์ชันอย่าง `mint` นั้นสะท้อนปรัชญาของ #Rust อย่างชัดเจน การแก้ไข State จะต้องทำผ่าน `&mut self` และการเข้าถึง Storage field อย่าง `total_supply` หรือ `owners` นั้น Stylus บังคับให้ใช้ Pattern `.get()` และ `.set()` อย่างชัดเจน แม้ดูเหมือนจะ Verbose กว่า Solidity เล็กน้อย แต่มันทำให้ Developer ตระหนักถึง "Cost" ของการทำ SLOAD และ SSTORE ในทุกบรรทัด นอกจากนี้ การจัดการตัวเลขยังถูกบังคับให้ใช้ Type จาก `alloy_sol_types` (เช่น U256) ร่วมกับ `Address` type เพื่อป้องกันปัญหา Overflow/Underflow ที่มักเกิดขึ้นใน Smart Contract ทั่วไป

ในมุมของ Developer Experience และ Tooling การออกแบบ System Architecture ผ่าน Claude Skill นี้เลือกใช้ Monorepo Structure ที่ขับเคลื่อนด้วย `pnpm workspace` แต่หัวใจหลักยังคงเป็น Cargo ecosystem สำหรับส่วนของ Contract โปรเจกต์จะถูก Scaffold ขึ้นมาพร้อมกับ `nitro-devnode` (Local Arbitrum chain ใน Docker) ซึ่งพร้อมรันทันที สิ่งที่น่าประทับใจคือ Workflow ที่ไร้รอยต่อ เราเขียน #Rust ใช้ `cargo stylus deploy` เพื่อส่งขึ้น Chain และใช้ `cargo stylus export-abi` เพื่อ Gen ABI ออกมาให้ Frontend เรียกใช้ผ่าน `wagmi` และ `viem` ได้โดยตรง ซึ่ง TypeScript ฝั่ง Frontend จะ Catch error ได้ทันทีถ้าเราเรียกชื่อฟังก์ชันผิดหรือส่ง Type ผิด นี่คือ End-to-End Type Safety ที่เชื่อมโลกของ Smart Contract และ Client Side เข้าด้วยกันอย่างสมบูรณ์

สุดท้ายนี้ สิ่งที่ Ben Greenberg นำเสนอไม่ใช่แค่ Tool ช่วยเขียนโค้ด แต่เป็นการประกาศว่า #Rust พร้อมแล้วที่จะเป็นแกนหลักใน Application Layer ของ Blockchain การที่ Decision Tree ของระบบแนะนำให้เลือก Stylus Rust เมื่อต้องการประสิทธิภาพสูงสุด เป็นเครื่องยืนยันว่าเราไม่ได้เขียน #Rust บน Blockchain เพียงเพราะเราชอบภาษานี้ แต่เพราะมันมอบ Architecture ที่เหนือกว่า ทั้งในแง่ของ Execution Speed, Gas Optimization และความปลอดภัยของ Memory Safety

หากใครที่เคยลังเลว่าจะเอาความรู้ #Rust มาใช้ในโลก EVM อย่างไร นี่คือ Use case ที่พิสูจน์แล้วว่า #Rust คือ "Real Deal" ในยุคถัดไปของ Smart Contract Development

**Credit & Reference:**

1. [How I Built a Claude Code Skill That Scaffolds Complete Arbitrum dApps](https://dev.to/arbitrum/how-i-built-a-claude-code-skill-that-scaffolds-complete-arbitrum-dapps-2njl)
2. [arbitrum-dapp-skill Documentation](https://hummusonrails.github.io/arbitrum-dapp-skill/)
3. [X post](https://x.com/hummusonrails/status/2019337368033992833)
4. [Build a dApp on Arbitrum with Claude (Video)](https://www.youtube.com/watch?v=vsejiaOTmJA&t=2s)
5. [arbitrum-dapp-skill GitHub repo](https://github.com/hummusonrails/arbitrum-dapp-skill)

<!-- NAVIGATION:START -->
<!-- markdownlint-disable MD033 -->
<div class="article-nav">
  <a class="nav-left" href="../index.html">← Introduction</a>
  <a class="nav-right" href="./index.html">Category index →</a>
</div>
<!-- markdownlint-enable MD033 -->
<!-- NAVIGATION:END -->
