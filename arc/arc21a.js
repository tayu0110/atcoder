function main(input) {
  input = input.trim(' ').split('\n')
  let a = []
  for(let i = 0; i < 4; i++) {
    let d = input[i].split(' ')
    a.push(d)
  }
  let flag = false
  for(let i = 0; i < 4; i++) {
    for(let j = 0; j < 4; j++) {
      if(i+1 < 4 && a[i][j] === a[i+1][j]) flag = true
      if(j+1 < 4 && a[i][j] === a[i][j+1]) flag = true
    }
  }
  if(flag) console.log('CONTINUE')
  else console.log('GAMEOVER')
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))