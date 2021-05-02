function main(input) {
  input = input.trim(' ').split('\n')
  let [ n ] = input[0].split(' ').map(e => parseInt(e, 10))
  let a = input[1].split(' ').map(e => parseInt(e, 10))
  a.sort((a, b) => a - b)
  let f = 0, s = 0;
  for(let i = n-1; i >= 0; i--) {
    if((n-1-i) % 2 === 0) f += a[i]
    else s += a[i]
  }
  console.log(f)
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))