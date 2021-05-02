function main(input) {
  input = input.trim(' ').split('\n')
  let [ s ] = input
  let ans = ""
  let now;
  let num = 0
  let len = s.length
  for(let i = 0; i < len; i++) {
    if(i === 0) {
      now = s[i]
      num++;
      continue;
    }
    if(s[i] === now) {
      num++
    } else {
      ans += `${now}${num}`
      now = s[i]
      num = 1
    }
  }
  ans += `${now}${num}`
  console.log(ans)
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))