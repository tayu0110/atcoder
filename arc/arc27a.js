function main(input) {
  input = input.trim(' ').split('\n')
  let [ h, m ] = input[0].split(' ').map(e => parseInt(e, 10))
  let ans = 18 * 60 - h * 60 - m
  console.log(ans)
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))