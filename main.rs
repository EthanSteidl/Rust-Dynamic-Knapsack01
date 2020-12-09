
struct Dynamic01Knapsack
{
    result: i32,
    objects: Vec<i32>,
}
impl Dynamic01Knapsack
{
    fn new() -> Dynamic01Knapsack
    {
        return Dynamic01Knapsack{result:0, objects:Vec::new()};
    }
    fn max(&mut self, left: i32, right: i32) -> i32
    {
        return if left < right {right} else {left}
    }
    fn knapsack01(&mut self, capacity: i32, items: i32, weights: &mut[i32], values: &mut[i32]) -> i32
    {
        let mut table: Vec<Vec<i32>> = Vec::with_capacity((items+1) as usize);
        for _ in 0..items+1
        {
            let mut next_row: Vec<i32> = Vec::with_capacity((capacity+1) as usize);
            for _ in 0..capacity+1
            {
                next_row.push(0);
            }
            table.push(next_row);
        }


        let mut i:usize = 0;
        
        while i < items as usize + 1 
        {
            for weight in 0..capacity+1 
            {
                if i == 0 || weight == 0
                {
                    table[i][weight as usize] = 0;
                }
                else if weights[i as usize - 1] <= weight
                {
                    table[i][weight as usize] = self.max( (values[i- 1] + table[i - 1][weight as usize - weights[i - 1] as usize]).clone(), (table[i - 1][weight as usize]).clone() );
                }
                else
                {
                    table[i][weight as usize] = table[i - 1][weight as usize];
                }
            }
            i += 1;
        }
        self.result = table[items as usize ][capacity as usize];
        let mut value = self.result;
        let mut weight = capacity as usize;

        i = items as usize -1;
        while value > 0
        {
            if value == table[i][weight]
            {
                if i==0
                {
                    break;
                }
                i -= 1;
            }
            else
            {
                self.objects.push(weights[i]);
                value -= values[i];
                weight -= weights[i] as usize;
                if i == 0
                {
                    break;
                }
                i -= 1;
            }
            
        }
        
        
        self.result
    }
}

fn main() {
    println!("Hello, world!");
    let mut knapsack = Dynamic01Knapsack::new();
    let mut values: [i32; 4] = [40, 100, 50, 60];
    let mut weights: [i32; 4] = [20, 10, 40, 30];
    let weight: i32 = 60;
    let size: i32 = 4;
    knapsack.knapsack01(weight, size, &mut weights, &mut values);

    println!("Result: {}\n Objects: {:?}", knapsack.result, knapsack.objects);
}
