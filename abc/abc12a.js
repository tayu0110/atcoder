function main(input) {
  input = input.trim(' ').split('\n')
  let [a, b] = input[0].split(' ').map(e => parseInt(e, 10))
  let tmp = a
  a = b
  b = tmp
  console.log(`${a} ${b}`)
}

main(require('fs').readFileSync('/dev/stdin', 'utf-8'))