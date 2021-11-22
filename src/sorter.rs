pub fn run() {

    let mut numbers = Vec::new();

    numbers.push(7);
    numbers.push(3);
    numbers.push(1);
    numbers.push(4);
    numbers.push(2);
    
    sort(&mut numbers);

    print_vector(numbers);
}

fn sort(to_sort: &mut Vec<i32>) {
    
    let size = to_sort.len();

    println!("To Sort length : {}", size);

    for i in 0..size {
        for j in i+1..size {
            println!("Sorting indexes i:[{}]:{} j:[{}]:{}", 
                    i, 
                    to_sort[i], 
                    j, 
                    to_sort[j]);

            if to_sort[i] > to_sort[j] {
                let j_tmp = to_sort[j];
                to_sort[j] = to_sort[i];
                to_sort[i] = j_tmp;
            }
        }
    }
}

fn print_vector(to_print: Vec<i32>) {
    for i in 0..to_print.len() {
        println!("[{}]:{}", i, to_print[i]);
    }
}