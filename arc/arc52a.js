function main(input) {
  input = input.trim(' ').split('\n')
  let [ s ] = input[0].split(' ')
  let ans = ''
  for(let e of s) {
    if(e === '1' || e === '2' || e === '3' || e === '4' || e === '5' || e === '6' || e === '7' || e === '8' || e === '9' || e === '0') ans += e
  }
  console.log(ans)
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))