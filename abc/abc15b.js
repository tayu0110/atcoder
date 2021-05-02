function main(input) {
  input = input.trim(' ').split('\n')
  let [ n ] = input[0].split(' ').map(e => parseInt(e, 10))
  let a = input[1].split(' ').map(e => parseInt(e, 10))
  let sum = 0
  a.forEach(val => {
    sum += val
    if(val === 0) n--;
  })
  let ave = Math.ceil(sum / n)
  console.log(ave)
}

main(require('fs').readFileSync('/dev/stdin', 'utf-8'))