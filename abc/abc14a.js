function main(input) {
  input = input.trim(' ').split('\n')
  let [ a ] = input[0].split(' ').map(e => parseInt(e, 10))
  let [ b ] = input[1].split(' ').map(e => parseInt(e, 10))
  console.log(Math.ceil(a/b)*b-a);  
}

main(require('fs').readFileSync('/dev/stdin', 'utf-8'))