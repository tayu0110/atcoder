function main(input) {
  input = input.trim(' ').split('\n')
  let d = input[0].split(' ').map(e => parseInt(e, 10))
  let j = input[1].split(' ').map(e => parseInt(e, 10))
  let ans = 0
  for(let i = 0; i < 7; i++) {
    ans += Math.max(d[i], j[i])
  }
  console.log(ans)
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))