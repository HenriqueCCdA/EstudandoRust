pub struct Solution;

impl Solution{
    pub fn two_sums(v: Vec<i32>, alvo: i32) -> Vec<i32> {

        println!("v = {:?}", v);
        println!("{}", alvo);
        for (i, e1) in v.iter().enumerate() {
            // println!("(i, e1) = {:?}", (i, e1));

            for (j, e2) in  v.iter().skip(i + 1).enumerate() {
                // println!("(j, e2) = {:?}", (j + i + 1, e2));

                if e1 + e2 == alvo {
                    // println!("[e1, e2] = {:?}, [i,  j] = {:?}", vec![e1, e2], vec![i, j]);
                    // println!("[i, j + 1 + i] = {:?}", vec![i, j + 1 +i]);
                    return vec![i as i32, j as i32 + i as i32 + 1 ]
                }
            }
        }

        vec![]
    }

    pub fn two_sums_hasmap(v: Vec<i32>, alvo: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut result: Vec<i32> = Vec::new();

        println!("v = {:?}, alvo = {}", v, alvo);

        for i in 0..v.len(){

            if map.contains_key(&(alvo - v[i])){
                result.push(map[&(alvo - v[i])]);
                result.push(i as i32);
                return result;
            }

            map.insert(v[i], i as i32);
            println!("map = {:?}", map);

        }

        print!("result = {:?}", result);

        return result;
    }
}
