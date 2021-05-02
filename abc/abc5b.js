function main(input) {
  input = input.trim(' ').split('\n')
  let [ n, ...t] = input
  t.map(e => parseInt(e, 10))
  t.sort((a, b) => a - b)
  console.log(t[0])
}

main(require('fs').readFileSync('/dev/stdin', 'utf-8'))