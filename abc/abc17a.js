function main(input) {
  input = input.trim(' ').split('\n')
  let [s1, e1] = input[0].split(' ').map(e => parseInt(e, 10))
  let [s2, e2] = input[1].split(' ').map(e => parseInt(e, 10))
  let [s3, e3] = input[2].split(' ').map(e => parseInt(e, 10))
  let ans = s1*e1 + s2*e2 + s3*e3
  ans /= 10
  console.log(ans)
}

main(require('fs').readFileSync('/dev/stdin', 'utf-8'))