function main(input) {
  input = input.trim(' ').split('\n')
  let [a, k] = input[0].split(' ').map(e => parseInt(e, 10))
  if(k === 0) {
    console.log(2000000000000 - a)
    return
  }
  let cnt = 0
  while(a < 2000000000000) {
    a += 1 + k * a
    cnt++
  }
  console.log(cnt)
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))