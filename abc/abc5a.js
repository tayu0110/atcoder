function main(input) {
  input = input.trim(' ').split('\n')
  let [ x, y ] = input[0].split(' ').map(e => parseInt(e, 10))
  console.log(Math.floor(y/x))
}

main(require('fs').readFileSync('/dev/stdin', 'utf-8'))