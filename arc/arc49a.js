function main(input) {
  input = input.trim(' ').split('\n')
  let [ s ] = input[0].split(' ')
  let [ a, b, c, d ] = input[1].split(' ').map(e => parseInt(e, 10))
  a--;b--;c--;d--;
  let ans = ''
  if(a === -1) ans += '\"'
  for(let i = 0; i < s.length; i++) {
    ans += s[i]
    if(i === a || i === b || i === c || i === d) ans += '\"'
  }
  console.log(ans)
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))