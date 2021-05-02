function main(input) {
  input = input.trim(' ').split('\n')
  let [ s ] = input[0].split(' ')
  let t = ''
  for(let i = 0; i < s.length; i++) {
    let e = s[i];
    if(e === 'O' || e === 'D') t += '0'
    else if(e === 'I') t += '1'
    else if(e === 'Z') t += '2'
    else if(e === 'S') t += '5'
    else if(e === 'B') t += '8'
    else t += e
  }
  console.log(t)
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))