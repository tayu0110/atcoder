function main(input) {
  input = input.trim(' ').split('\n')
  let [m, n, N] = input[0].split(' ').map(e => parseInt(e, 10))
  let c = 0
  let ans = N
  while(c + N >= m) {
    let nc = (c + N) % m
    let nn = Math.floor((c + N) / m) * n
    ans += nn
    N = nn
    c = nc
  }
  console.log(ans)
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))