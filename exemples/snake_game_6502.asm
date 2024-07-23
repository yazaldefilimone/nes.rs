  jsr init
  jsr loop

init:
  jsr initSnake
  jsr generateApplePosition
  rts

initSnake:
  lda #2
  sta $02
  lda #4
  sta $03
  lda #$11
  sta $10
  lda #$10
  sta $12
  lda #$0f
  sta $14
  lda #$04
  sta $11
  sta $13
  sta $15
  rts

generateApplePosition:
  lda $fe
  sta $00
  lda $fe
  and #$03
  clc
  adc #2
  sta $01
  rts

loop:
  jsr readKeys
  jsr checkCollision
  jsr updateSnake
  jsr drawApple
  jsr drawSnake
  jsr spinWheels
  jmp loop

readKeys:
  lda $ff
  cmp #$77
  beq upKey
  cmp #$64
  beq rightKey
  cmp #$73
  beq downKey
  cmp #$61
  beq leftKey
  rts

upKey:
  lda #4
  bit $02
  bne illegalMove
  lda #1
  sta $02
  rts

rightKey:
  lda #8
  bit $02
  bne illegalMove
  lda #2
  sta $02
  rts

downKey:
  lda #1
  bit $02
  bne illegalMove
  lda #4
  sta $02
  rts

leftKey:
  lda #2
  bit $02
  bne illegalMove
  lda #8
  sta $02
  rts

illegalMove:
  rts

checkCollision:
  jsr checkAppleCollision
  jsr checkSnakeCollision
  rts

checkAppleCollision:
  lda $00
  cmp $10
  bne doneCheckingAppleCollision
  lda $01
  cmp $11
  bne doneCheckingAppleCollision
  inc $03
  inc $03
  jsr generateApplePosition

doneCheckingAppleCollision:
  rts

checkSnakeCollision:
  ldx #2
snakeCollisionLoop:
  lda $10,x
  cmp $10
  bne continueCollisionLoop
maybeCollided:
  lda $11,x
  cmp $11
  beq didCollide
continueCollisionLoop:
  inx
  inx
  cpx $03
  beq didntCollide
  jmp snakeCollisionLoop

didCollide:
  jmp gameOver

didntCollide:
  rts

updateSnake:
  ldx $03
  dex
  txa
updateloop:
  lda $10,x
  sta $12,x
  dex
  bpl updateloop

  lda $02
  lsr
  bcs up
  lsr
  bcs right
  lsr
  bcs down
  lsr
  bcs left

up:
  lda $10
  sec
  sbc #$20
  sta $10
  bcc upup
  rts
upup:
  dec $11
  lda #$1
  cmp $11
  beq collision
  rts

right:
  inc $10
  lda #$1f
  bit $10
  beq collision
  rts

down:
  lda $10
  clc
  adc #$20
  sta $10
  bcs downdown
  rts
downdown:
  inc $11
  lda #$6
  cmp $11
  beq collision
  rts

left:
  dec $10
  lda $10
  and #$1f
  cmp #$1f
  beq collision
  rts

collision:
  jmp gameOver

drawApple:
  ldy #0
  lda $fe
  sta ($00),y
  rts

drawSnake:
  ldx #0
  lda #1
  sta ($10,x)
  ldx $03
  lda #0
  sta ($10,x)
  rts

spinWheels:
  ldx #0
spinloop:
  nop
  nop
  dex
  bne spinloop
  rts

gameOver:
