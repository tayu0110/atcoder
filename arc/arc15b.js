function main(input) {
  input = input.trim(' ').split('\n')
  let [n] = input[0].split(' ').map(e => parseInt(e, 10))
  let M = []
  let m = []
  for(let i = 1; i < input.length; i++) {
    let [k, l] = input[i].split(' ').map(e => parseFloat(e, 10) * 10)
    M.push(k)
    m.push(l)
  }
  let ans = [0, 0, 0, 0, 0, 0]
  for(let i = 0; i < M.length; i++) {
    let k = M[i]
    let l = m[i]
    if(k >= 350) ans[0]++
    if(k >= 300 && k < 350) ans[1]++
    if(k >= 250 && k < 300) ans[2]++
    if(l >= 250) ans[3]++
    if(l < 0 && k >= 0) ans[4]++
    if(k < 0) ans[5]++
  }
  let res = `${ans[0]} ${ans[1]} ${ans[2]} ${ans[3]} ${ans[4]} ${ans[5]}`
  console.log(res)
}
main(require('fs').readFileSync('/dev/stdin', 'utf-8'))