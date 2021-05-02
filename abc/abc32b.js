function main(input) {
  input = input.trim(' ').split('\n')
  let s = input[0]
  let [ k ] = input[1].split(' ').map(e => parseInt(e, 10))
  let ck = new Set();
  for(let i = 0; i < s.length-k+1; i++) {
    let t = s.substr(i, k)
    ck.add(t)
  }
  console.log(ck.size)
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))