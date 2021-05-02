function main(input) {
    let x = parseInt(input, 10);

    let ans = 0;
    for(let i = 1; i*i*i*i <= x; i++) {
        if(i*i*i*i === x) {
            ans = i;
            break;
        }
    }

    console.log(ans);
}

main(require('fs').readFileSync('/dev/stdin', 'utf-8'));