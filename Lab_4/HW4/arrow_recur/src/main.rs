fn main() {}

#[allow(dead_code)]
fn arrow1(v: &[usize]) -> String {
    if v.len() == 0 {
        return String::new();
    } else {
        let mut z = arrow1(&v[..v.len() - 1]);
        let n = v[v.len() - 1];
        for i in 1..n {
            z.push_str(&("*".repeat(i)));
            z.push('\n')
        }
        for i in 0..n {
            z.push_str(&("*".repeat(n - i)));
            z.push('\n')
        }
        z
    }
}

#[test]
fn test_arrow_recur1() {
    assert_eq!(arrow1(&[]), (""));
    assert_eq!(arrow1(&[2]), ("*\n**\n*\n"));
    assert_eq!(
        arrow1(&[6]),
        ("*\n**\n***\n****\n*****\n******\n*****\n****\n***\n**\n*\n")
    );
}

#[allow(dead_code)]
fn arrow2(v: &[usize]) -> String {
    if v.is_empty() {
        return String::new();
    } else {
        if v[0] == 0 {
            return String::new();
        }
        let mut z:String = arrow2(&v[..v.len()-1]);
        let mut i:usize = 1; 
        let n = v[v.len() - 1];
        while i <= n {
            z.push_str(&(" ".repeat(n +  1 - i)));
            z.push_str(&("*".repeat(i)));
            z.push('\n');
            i += 1;
        }
        while i >= 1 {
            z.push_str(&(" ".repeat(n + 1 - i)));
            z.push_str(&("*".repeat(i)));
            z.push('\n');
            i -= 1; 
        }
        z
    }
}

#[test]
fn test_arrow_recur2() {
    assert_eq!(arrow2(&[]), (""));
    assert_eq!(arrow2(&[2]), ("  *\n **\n***\n **\n  *\n"));
}
