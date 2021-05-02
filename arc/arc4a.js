function main(input) {
  input = input.trim(' ').split('\n')
  let [ n ] = input[0].split(' ').map(e => parseInt(e, 10))
  let x = []
  let y = []
  for(let i = 1; i < input.length; i++) {
    let [ a, b ] = input[i].split(' ').map(e => parseInt(e, 10))
    x.push(a)
    y.push(b)
  }
  let ans = 0
  for(let i = 0; i < n; i++) {
    for(let j = i+1; j < n; j++) {
      let x1 = x[i]
      let y1 = y[i]
      let x2 = x[j]
      let y2 = y[j]
      let s = Math.sqrt((x2-x1)*(x2-x1) + (y2-y1)*(y2-y1))
      ans = Math.max(ans, s)
    }
  }
  console.log(ans)
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))