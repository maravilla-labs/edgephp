(module
  (type (;0;) (func (param i32 i32)))
  (type (;1;) (func (result i32)))
  (type (;2;) (func (param i32 i32) (result i32)))
  (type (;3;) (func (param i32 i32) (result i32)))
  (type (;4;) (func (param i32) (result i32)))
  (type (;5;) (func))
  (import "env" "print" (func (;0;) (type 0)))
  (func (;1;) (type 1) (result i32)
    (local i32 i32)
    i32.const 0
    i32.load
    local.tee 0
    i32.const 16
    i32.add
    local.set 1
    i32.const 0
    local.get 1
    i32.store
    local.get 0)
  (func (;2;) (type 2) (param i32 i32) (result i32)
    local.get 0)
  (func (;3;) (type 3) (param i32 i32) (result i32)
    local.get 0)
  (func (;4;) (type 3) (param i32 i32) (result i32)
    local.get 0)
  (func (;5;) (type 3) (param i32 i32) (result i32)
    local.get 0)
  (func (;6;) (type 3) (param i32 i32) (result i32)
    (local i32 i32 i64 i64 i32 i32)
    local.get 0
    i32.load8_u
    local.set 2
    local.get 1
    i32.load8_u
    local.set 3
    local.get 2
    i32.const 2
    i32.eq
    local.get 3
    i32.const 2
    i32.eq
    i32.and
    if  ;; label = @1
      local.get 0
      i32.const 4
      i32.add
      i64.load
      local.set 4
      local.get 1
      i32.const 4
      i32.add
      i64.load
      local.set 5
      local.get 4
      local.get 5
      i64.gt_s
      local.set 6
      call 1
      local.tee 7
      i32.const 1
      i32.store8
      local.get 7
      i32.const 4
      i32.add
      local.get 6
      i32.store8
      local.get 7
      return
    end
    call 1
    local.tee 7
    i32.const 1
    i32.store8
    local.get 7
    i32.const 4
    i32.add
    i32.const 0
    i32.store8
    local.get 7)
  (func (;7;) (type 3) (param i32 i32) (result i32)
    (local i32)
    call 1
    local.tee 2
    i32.const 1
    i32.store8
    local.get 2)
  (func (;8;) (type 4) (param i32) (result i32)
    (local i32 i32)
    local.get 0
    i32.load8_u
    local.set 1
    local.get 1
    i32.const 1
    i32.eq
    if  ;; label = @1
      local.get 0
      return
    end
    local.get 0)
  (func (;9;) (type 4) (param i32) (result i32)
    (local i32)
    local.get 0
    i32.load8_u
    local.set 1
    local.get 1
    i32.const 4
    i32.eq
    if  ;; label = @1
      local.get 0
      return
    end
    local.get 0)
  (func (;10;) (type 5)
    i32.const 0
    i32.const 1048576
    i32.store)
  (memory (;0;) 4)
  (export "_start" (func 10))
  (export "memory" (memory 0))
  (data (;0;) (i32.const 4096) "output"))
