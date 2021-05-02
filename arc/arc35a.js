function main(input) {
  input = input.trim(' ').split('\n')
  let [ s ] = input[0].split(' ')
  let len = s.length
  let f = true
  for(let i = 0; i < len; i++) {
    let j = len - 1 - i
    if(j < i) break
    if(s[i] === '*' || s[j] === '*') continue
    if(s[i] !== s[j]) f = false
  }
  if(f) console.log('YES')
  else console.log('NO')
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))