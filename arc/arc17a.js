function main(input) {
  input = input.trim(' ').split('\n')
  let [ n ] = input[0].split(' ').map(e => parseInt(e, 10))
  for(let i = 2; i < n; i++) {
    if(n % i === 0) {
      console.log('NO')
      return;
    }
  }
  console.log('YES')
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))