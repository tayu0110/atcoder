function main(input) {
  input = input.trim(' ').split('\n')
  let [ n ] = input[0].split(' ').map(e => parseInt(e, 10))
  let ans = '1'
  for(let i = 0; i < n-1; i++) ans += '0'
  ans += '7'
  console.log(ans)
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))