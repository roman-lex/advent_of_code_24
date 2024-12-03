use std::fs;
fn main(){
    //day1();
    day2();
}

fn getInput(file:String) -> String{
    let file_path = String::from(file);
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    contents
}

fn day1(){
    let mut v: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();
    let file:String = String::from("input_day1_1.txt"); 
    let input = getInput(file);
    let mut sum = 0;

    for i in input.lines(){
        let nums: Vec<&str> = i.trim().split("   ").collect();
        let num = nums[0].parse::<i32>().unwrap();
        let num2: i32 = nums[1].parse::<i32>().unwrap();
        v.push(num);
        v2.push(num2);
    }
    v.sort();
    v2.sort();

    // Part 1
    for (i, elem) in v.iter().enumerate() {
        sum += elem.abs_diff(v2[i]);
    }

    // Part 2
    let mut sum2 = 0;
    for (i, elem) in v.iter().enumerate() {
        let mut cnt = 0;
        for j in v2.iter() {
            if elem == j {
                cnt += 1;
            }
        }
        sum2 += elem*cnt;
    }
    println!("Part 1: {}", sum);
    println!("Part 2: {}", sum2)
}

fn day2(){
    let file = String::from("input_day2_1.txt");
    let input = getInput(file);
    let mut cntSave = 0;
    let mut cntUnsave = 0;
    for i in input.lines(){
        let nums: Vec<&str> = i.trim().split(" ").collect();
        let mut v : Vec<i32> = Vec::new();
        for (i ,elem) in nums.iter().enumerate(){
            let num = nums[i].parse::<i32>().unwrap();
            v.push(num);
        }
        if isSave(v) == true{
            cntSave += 1;
        } else {
            cntUnsave += 1;
        }
    }
    println!("{}", cntSave);
}

fn isSave(v:Vec<i32>) -> bool {
    let mut save = true;
    let mut vorher = v[0];
    for (i, elem) in v.iter().enumerate(){
        if vorher + 3 < *elem {
            save = false;
        }
        vorher = *elem;
    }
    save
}
