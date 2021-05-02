function main(input) {
  input = input.trim(' ').split('\n')
  let [ n ] = input[0].split(' ').map(e => parseInt(e, 10))
  console.log(Math.ceil(n/2))
}

main(require('fs').readFileSync('/dev/stdin', 'utf-8'))