function main(input) {
  input = input.trim(' ').split('\n')
  let n = input[0].split(' ').map(e => parseInt(e, 10))
  let ck = new Set()
  let ans = []
  for(let i = 0; i < 5; i++) {
    for(let j = i+1; j < 5; j++) {
      for(let k = j+1; k < 5; k++) {
        let l = n[i] + n[j] + n[k]
        if(ck.has(l)) continue
        ck.add(l)
        ans.push(l)
      }
    }
  }
  ans.sort((a, b) => b - a)
  console.log(ans[2])
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))