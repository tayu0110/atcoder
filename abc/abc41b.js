function main(input) {
    input = input.trim().split('\n')
    let [a, b, c] = input[0].split(' ').map(BigInt);
    const mod = 1000000007n
    let ans = a * b % mod * c % mod
    ans = Number(ans)
    console.log(ans)
}

main(require('fs').readFileSync('/dev/stdin', 'utf-8'))