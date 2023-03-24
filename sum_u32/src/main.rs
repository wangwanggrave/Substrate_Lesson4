fn sum_u32(numbers: &[u32]) -> Option<u32>{
    let mut sum: u32 = 0;
    for number in numbers{
        match sum.checked_add(*number){
            Some(result) => sum = result,
            None => return None,
        }
    }
    Some(sum)
}



fn main() {
    //let nums = &[1,2,3,4,5];
    let nums = vec![u32::MAX, 1];
    match sum_u32(&nums){
        Some(result) => println!("Sum of {:?} is {}", nums, result),
        None => println!("Sum of {:?} caused an overflow!", nums),
    }
}
