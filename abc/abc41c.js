function main(input) {
  input = input.trim(' ').split('\n')
  let [ n ] = input[0].split(' ').map(e => parseInt(e, 10))
  let a = input[1].split(' ').map(e => parseInt(e, 10))
  let v = []
  for(let i = 0; i < n; i++) {
    v.push({key: a[i], val: i+1})
  }
  v.sort((a, b) => b.key - a.key)
  for(let e of v) {
    console.log(e.val)
  }
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))