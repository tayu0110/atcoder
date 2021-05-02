function main(input) {
  input = input.trim(' ').split('\n')
  let [n] = input[0].split(' ').map(e => parseInt(e, 10))
  let ans = 100000
  for(let i = 1; i < n; i++) {
    let a = '' + i
    let b = '' + (n - i)
    let res = 0
    for(let e of a) res += parseInt(e)
    for(let e of b) res += parseInt(e)
    ans = Math.min(ans, res)
  }
  console.log(ans)
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))