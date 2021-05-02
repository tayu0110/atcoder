function main(input) {
  input = input.trim(' ').split('\n')
  let s = input[0].split(' ')
  let res = []
  for(let e of s) {
    if(e === 'Left') res.push('<')
    else if(e === 'Right') res.push('>')
    else res.push('A')
  }
  let len = res.length
  let ans = ''
  for(let i = 0; i < len; i++) {
    ans += res[i]
    if(i !== len-1) ans += ' '
  }
  console.log(ans)
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))