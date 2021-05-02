function main(input) {
  input = input.trim(' ').split('\n')
  let [ n ] = input[0].split(' ').map(e => parseInt(e, 10))
  let a = input[1].split(' ').map(e => parseInt(e, 10))
  let t = 0
  for(let e of a) if(e % 2 === 1) t++
  if(t % 2 === 0) console.log('YES')
  else console.log('NO')
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))