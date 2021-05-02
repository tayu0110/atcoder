function main(input) {
  input = input.trim(' ').split('\n')
  let [a, b, c] = input[0].split(' ').map(e => parseInt(e, 10))
  let ans
  if(a + b >= c) {
    ans = b + c
  } else {
    let t = a + b
    ans = t + b + 1
  }
  console.log(ans)
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))