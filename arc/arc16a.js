function main(input) {
  input = input.trim(' ').split('\n')
  let [ n, m ] = input[0].split(' ').map(e => parseInt(e, 10))
  if(m === 1) console.log(2)
  else console.log(1)
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))