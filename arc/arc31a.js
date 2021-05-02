function main(input) {
  input = input.trim(' ').split('\n')
  let [ s ] = input[0].split(' ')
  let n = s.length
  let f = true
  for(let i = 0; i < n; i++) {
    let j = n-1 - i
    if(j < i) break
    if(s[i] !== s[j]) f = false
  }
  if(f) console.log('YES')
  else console.log('NO')
  return
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))