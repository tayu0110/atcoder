function main(input) {
  input = input.trim(' ').split('\n')
  let [ a ] = input[0].split(' ')
  if(a === 'a') console.log(-1)
  else console.log('a')
}

main(require('fs').readFileSync('/dev/stdin', 'utf-8'))