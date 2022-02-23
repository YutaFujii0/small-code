

pub fn dp(nums: Vec<u32>) -> String {
    let n = nums.len();
    let mut dp_weight = vec![0u32; n];
    let mut s_dash = vec![0u8; n];
    let mut s_dash_dash = vec![0u8; n];

    for (i, num) in nums.iter().enumerate() {
        if i == 0 {
            dp_weight[i] = *num;
            s_dash_dash[i] = 1;
        } else if i == 1 {
            if num > &dp_weight[i-1] {
                dp_weight[i] = *num;
                s_dash_dash[i] = 1;
            } else {
                dp_weight[i] = dp_weight[i-1];
                s_dash_dash = s_dash.clone();
            }
        } else {
            if dp_weight[i-2] + num > dp_weight[i-1] {
                dp_weight[i] = dp_weight[i-2] + *num;
                s_dash_dash[i] = 1;
            } else {
                dp_weight[i] = dp_weight[i-1];
                s_dash_dash = s_dash.clone();
            }
        }
        let tmp = s_dash_dash;
        s_dash_dash = s_dash;
        s_dash = tmp;
    }
    print!("{:?}", s_dash[0]);
    print!("{:?}", s_dash[1]);
    print!("{:?}", s_dash[2]);
    print!("{:?}", s_dash[3]);
    print!("{:?}", s_dash[16]);
    print!("{:?}", s_dash[116]);
    print!("{:?}", s_dash[516]);
    println!("{:?}", s_dash[996]);
    s_dash.iter()
        .map(|i| i.to_string())
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1,200,1170,3,1700,4,517,7];
        assert_eq!(dp(nums), "10101010");
    }

    #[test]
    fn test2() {
        let nums = vec![1,2,3,1170,1700,4,517,7];
        assert_eq!(dp(nums), "10101010");
    }

    #[test]
    fn test3() {
        let nums = vec![1,2,3,1200,1700,800,17,7];
        assert_eq!(dp(nums), "01010101");
    }

    #[test]
    fn test4() {
        let nums = vec![100,2,3,1200,1700,800,17,7];
        assert_eq!(dp(nums), "10010101");
    }

    #[test]
    fn test5() {
        let nums = vec![1,2,3,1170,1700,4,7,517];
        assert_eq!(dp(nums), "10101001");
    }
}
