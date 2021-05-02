function main(input) {
    input = input.trim().split('\n');
    let [a, b, c] = input[0].split(' ').map(val => parseInt(val, 10));

    console.log(2*a*b + 2*b*c + 2*c*a);
}

main(require('fs').readFileSync('/dev/stdin', 'utf8'));