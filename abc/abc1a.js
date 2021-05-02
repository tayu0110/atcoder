function main(input) {
  input = input.trim(' ').split('\n')
  let [ h1 ] = input[0].split(' ').map(e => parseInt(e, 10))
  let [ h2 ] = input[1].split(' ').map(e => parseInt(e, 10))
  let ans = h1 - h2
  console.log(ans)
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))