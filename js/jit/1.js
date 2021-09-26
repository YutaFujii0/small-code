function add(a, b) {
	return a+b
}

for (let i = 0; i < 1e6; i++) {
  add(i, i)
}