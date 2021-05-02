function main(input) {
  input = input.trim(' ').split('\n')
  let [l, x, y, s, d] = input[0].split(' ').map(e => parseInt(e, 10))
  let a, b
  if(d-s < 0) {
    a = l - (s - d)
    b = s - d
  } else {
    a = d - s
    b = l - (d - s)
  }
  let ans = Math.min(a / (x + y), b / (y - x))
  if(y - x < 0) ans = a / (x + y)
  console.log(ans)
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))