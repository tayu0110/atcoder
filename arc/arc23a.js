function main(input) {
  input = input.trim(' ').split('\n')
  let [ y ] = input[0].split(' ').map(e => parseInt(e, 10))
  let [ m ] = input[1].split(' ').map(e => parseInt(e, 10))
  let [ d ] = input[2].split(' ').map(e => parseInt(e, 10))
  if(m === 1 || m === 2) {
    m += 12
    y--
  }
  let ans = 735369 - (365*y + Math.floor(y/4) - Math.floor(y/100) + Math.floor(y/400) + Math.floor(306 * (m+1) / 10) + d - 429)
  console.log(ans)
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))