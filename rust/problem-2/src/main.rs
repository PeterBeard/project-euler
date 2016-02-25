// Each new term in the Fibonacci sequence is generated by adding the
// previous two terms. By starting with 1 and 2, the first 10 terms will be:
// 
// 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
// 
// By considering the terms in the Fibonacci sequence whose values do not
// exceed four million, find the sum of the even-valued terms.

fn main() {
    let (mut a, mut b) = (1, 2);
    let mut sum = 2;
    loop {
        let c = a + b;
        a = b;
        b = c;
        
        if c > 4000000 {
            break;
        }

        if c % 2 == 0 {
            sum += c;
        }
    }
    println!("The sum is {}", sum);
}
