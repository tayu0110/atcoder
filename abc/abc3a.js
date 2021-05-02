function main(input) {
  input = input.trim(' ').split('\n')
  let [ n ] = input[0].split(' ').map(e => parseInt(e, 10))
  console.log(10000 * (n+1) / 2)
}

main(require('fs').readFileSync('/dev/stdin', 'utf-8'))