function main(input) {
  input = input.trim(' ').split('\n')
  let h = input.length
  let w = input[0].length
  for(let i = h-1; i >= 0; i--) {
    let s = ''
    for(let j = w-1; j >= 0; j--) {
      s += input[i][j]
    }
    console.log(s)
  }
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))