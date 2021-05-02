function main(input) {
  input = input.trim(' ').split('\n')
  let [ day ] = input[0].split(' ')
  if(day === 'Sunday' || day === 'Saturday') console.log(0)
  else if(day === 'Friday') console.log(1)
  else if(day === 'Thursday') console.log(2)
  else if(day === 'Wednesday') console.log(3)
  else if(day === 'Tuesday') console.log(4)
  else console.log(5)
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))