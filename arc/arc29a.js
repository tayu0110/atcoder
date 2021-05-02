function main(input) {
  input = input.trim(' ').split('\n')
  let [n] = input[0].split(' ').map(e => parseInt(e, 10))
  let t = []
  let sum = 0
  for(let i = 1; i < n+1; i++) {
    t.push(parseInt(input[i].split(' '), 10))
    sum += t[i-1]
  }
  let ans = 100100100
  for(let i = 0; i < (1<<n); i++) {
    let k = 0
    for(let j = 0; j < n; j++) {
      if(i & (1<<j)) k += t[j]
    }
    let l = sum - k
    ans = Math.min(ans, Math.max(l, k))
  }
  console.log(ans)
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))