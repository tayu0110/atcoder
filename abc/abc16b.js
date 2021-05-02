function main(input) {
  input = input.trim(' ').split('\n')
  let [a, b, c] = input[0].split(' ').map(e => parseInt(e, 10))
  if(a+b === c && a-b === c) console.log('?')
  else if(a+b === c) console.log('+')
  else if(a-b === c) console.log('-')
  else console.log('!')
}

main(require('fs').readFileSync('/dev/stdin', 'utf-8'))