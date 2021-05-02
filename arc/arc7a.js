function main(input) {
  input = input.trim(' ').split('\n')
  let [ x ] = input[0].split(' ')
  let [ s ] = input[1].split(' ')
  let res = ""
  for(let i = 0; i < s.length; i++) {
    if(s[i] === x) continue
    res += s[i]
  }
  console.log(res)
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))