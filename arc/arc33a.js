function main(input) {
  input = input.trim(' ').split('\n')
  let [ n ] = input[0].split('\n').map(e => parseInt(e, 10))
  let ans = n * (n+1) / 2
  console.log(ans)
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))