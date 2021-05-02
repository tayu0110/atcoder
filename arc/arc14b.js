function main(input) {
  input = input.trim(' ').split('\n')
  let [n] = input[0].split(' ').map(e => parseInt(e, 10))
  let w = []
  for(let i = 1; i < input.length; i++) {
    w.push(input[i].split(' '))
  }
  let ck = new Set()
  let prev = ''
  let i = 0
  let ans = ['WIN', 'LOSE']
  for(let t of w) {
    let s = t[0]
    i++
    if(prev === '') {
      prev = s
      ck.add(s)
      continue;
    }
    if(ck.has(s)) {
      console.log(ans[i%2])
      return
    }
    ck.add(s)
    let pc = prev[prev.length-1]
    let bc = s[0]
    if(pc !== bc) {
      console.log(ans[i%2])
      return
    }
    prev = s
  }
  console.log('DRAW')
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))