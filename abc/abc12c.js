function main(input) {
  input = input.trim(' ').split('\n')
  let [ n ] = input[0].split(' ').map(e => parseInt(e, 10))
  let r = 2025 - n
  for(let i = 1; i < 10; i++) if(r%i === 0 && r/i < 10) console.log(`${i} x ${r/i}`)
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))