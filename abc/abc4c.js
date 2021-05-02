function main(input) {
  input = input.trim(' ').split('\n')
  let [n] = input[0].split(' ').map(e => parseInt(e, 10))
  n %= 30
  if(n === 0) {
    console.log(123456)
    return
  }
  let now = Math.ceil(n/5)
  let k = (now + 1 === 7 ? 1 : now + 1)
  let m = (n%5 === 0 ? 5 : n%5)
  let ans = []
  for(let i = 0; i < 6; i++) {
    if(i === m) ans.push(now)
    else {
      ans.push(k)
      k++
      if(k === now) k++
      if(k === 7) k = 1
    }
  }
  let res = ''
  for(let e of ans) res += e
  console.log(res)
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))