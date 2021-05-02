function main(input) {
  input = input.trim(' ').split('\n')
  let [n, l] = input[0].split(' ').map(e => parseInt(e, 10))
  let [s] = input[1].split(' ')
  let c = 0
  let t = 1
  for(let i = 0; i < s.length; i++) {
    if(s[i] === '+') t++
    else t--
    if(t > l) {
      c++
      t = 1
    }
  }
  console.log(c)
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))