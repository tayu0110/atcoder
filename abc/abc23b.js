function main(input) {
  input = input.trim(' ').split('\n')
  let [ n ] = input[0].split(' ').map(e => parseInt(e, 10))
  let s = input[1]
  if(n % 2 === 0) {
    console.log(-1)
    return
  }
  let m = Math.floor(n/2)
  if(s[m] !== 'b') {
    console.log(-1)
    return
  }
  for(let i = 1; i <= Math.floor(n/2); i++) {
    if(i%3 === 1) {
      let a = s[m-i]
      let c = s[m+i]
      if(a === 'a' && c === 'c') continue
      console.log(-1)
      return
    } else if(i%3 === 2) {
      let c = s[m-i]
      let a = s[m+i]
      if(a === 'a' && c === 'c') continue
      console.log(-1)
      return
    } else {
      let b1 = s[m-i]
      let b2 = s[m+i]
      if(b1 === 'b' && b2 === 'b') continue
      console.log(-1)
      return
    }
  }
  console.log(Math.floor(n/2))
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))