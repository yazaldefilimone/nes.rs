<samp>

### Nintendo System

A Nintendo Entertainment System (NES) emulator written in Rust.

Architecture (distributed system)

```plaintext

+--------+     +--------+     +--------+     +--------+     +--------+     +--------+
|  CPU   |<--> |  BUS   |<--> | ROM    |<--> |  PPU   |<--> | GamePad|<--> |  APU   |
+--------+     +--------+     +--------+     +--------+     +--------+     +--------+
                                  |              |              |
                                  v              v              v
                              +-------+     +--------+      +--------+
                              | Logic |     | Render |      | Scroll |
                              +-------+     +--------+      +--------+

```

Features

- CPU
- RAM
- PPU
- APU

#### TODO

- [ ] CPU
  - [x] Load Accumulator
  - [ ] Store Accumulator
  - [ ] Add with Accumulator
  - [ ] Subtract with Accumulator
  - [ ] AND with Accumulator
  - [ ] OR with Accumulator
  - [ ] XOR with Accumulator
  - [ ] Compare Accumulator
  - [ ] Shift Left
  - [ ] Shift Right
  - [ ] Rotate Left
  - [ ] Rotate Right
  - [ ] Jump
  - [ ] Jump Subroutine
  - [ ] Return from Subroutine
  - [ ] Call Subroutine
  - [ ] Return from Interrupt
  - [ ] Restart
  - [ ] Interrupt
- [ ] PPU
  - [ ] VRAM
    - [ ] Read
    - [ ] Write
    - [ ] Scroll
    - [ ] Sprite
      - [ ] Read
      - [ ] Write
      - [ ] Sprite Collision
      - [ ] Sprite Overflow
  - [ ] CGRAM
    - [ ] Read
    - [ ] Write
  - [ ] OAM
    - [ ] Read
    - [ ] Write
    - [ ] Sprite Collision
    - [ ] Sprite Overflow
- [ ] APU

Links

- https://www.nesdev.org/obelisk-6502-guide/reference.html
