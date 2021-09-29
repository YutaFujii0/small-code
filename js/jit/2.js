function foo(obj) {
  obj.x
}

foo({ x: 1, y: 1 })
foo({ x: 1 })
foo({ x: 1, c: 1 })


function foo(obj) {
  obj.y
}

foo({ x: 1, y: 1 })