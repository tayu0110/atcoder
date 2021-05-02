function main(input) {
  input = input.trim(' ').split('\n')
  let [ s, t ] = input[0].split(' ').map(e => parseInt(e, 10))
  console.log(t-s+1)
}

main(require('fs').readFileSync('/dev/stdin', 'utf-8'))