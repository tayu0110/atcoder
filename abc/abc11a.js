function main(input) {
  input = input.trim(' ').split('\n')
  let [ n ] = input[0].split('\n').map(e => parseInt(e, 10))
  console.log((n+1 === 13) ? 1 : n+1)
}

main(require('fs').readFileSync('/dev/stdin', 'utf-8'))