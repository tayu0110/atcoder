function main(input) {
  input = input.trim(' ').split('\n')
  let [ n ] = input[0].split(' ').map(e => parseInt(e, 10))
  let w = input[1].split(' ')
  w[n-1] = w[n-1].substr(0, w[n-1].length-1)
  let ans = 0
  for(let s of w) {
    if(s === 'TAKAHASHIKUN' || s === 'Takahashikun' || s === 'takahashikun') ans++
  }
  console.log(ans)
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))