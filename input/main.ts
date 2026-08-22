function max(a: number, b: number): number {
    if (a > b) {
        return a;
    } else {
        return b;
    }
}

let result = max(10, 22);
console.log(result + 100);