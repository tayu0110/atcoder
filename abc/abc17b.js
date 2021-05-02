function main(input) {
  input = input.trim(' ').split('\n')
  let [ s ] = input
  if(s === '') {
    console.log('YES')
    return;
  }
  let len = s.length
  let flag = true
  for(let i = 0; i < len; i++) {
    if(i+1 < len && s[i] === 'c' && s[i+1] === 'h') {
      i++
      continue
    } else if(s[i] === 'o' || s[i] === 'k' || s[i] === 'u') continue
    else {
      flag = false
    }
  }
  if(flag) console.log('YES')
  else console.log('NO')
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))