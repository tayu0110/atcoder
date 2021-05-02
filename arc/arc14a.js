function main(input) {
  input = input.trim(' ').split('\n')
  let [n] = input[0].split(' ').map(e => parseInt(e, 10))
  if(n & 1) console.log('Red')
  else console.log('Blue')
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))