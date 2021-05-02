function main(input) {
  input = input.trim(' ').split('\n')
  let [ n, ...s ] = input
  parseInt(n, 10)
  let v = {}
  s.forEach(e => {
    if(!v[e]) v[e] = 1
    else v[e]++
  })
  let max = 0
  let ans
  let keys = Object.keys(v)
  keys.forEach(e => {
    if(v[e] > max) {
      ans = e
      max = v[e]
    }
  })
  console.log(ans)
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))