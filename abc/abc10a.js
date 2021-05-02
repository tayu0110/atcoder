function main(input) {
  input = input.trim(' ').split('\n')
  let [ s ] = input[0].split('\n')
  console.log(s + 'pp')
}

main(require('fs').readFileSync('/dev/stdin', 'utf-8'))