#[derive(Debug)]
struct Item {
    name: String,
}

struct Range {
    start: u32,
    end: u32,
}

impl Iterator for Range {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }
        let result = Some(self.start);
        self.start += 1;
        result
    }
}

fn check_inventory(items: Vec<Item>, product: String) -> Vec<Item> {
    items.into_iter().filter(|i| i.name == product).collect()
}

fn main() {
    let mut vec: Vec<Item> = Vec::new();
    vec.push(Item {
        name: String::from("coat")
    });
    
    vec.push(Item {
    name: String::from("shirt")
    });
    
        vec.push(Item {
        name: String::from("shorts")
    });
    
    vec.push(Item {
    name: String::from("shoes")
    });
    
    let checked = check_inventory(vec, String::from("shirt"));
    
    println!("{:?}", checked);
    
    let mut range = Range {start: 0, end: 10};
    // for r in &range {
    //     println!("{}", r);
    // }
    
    let vec: Vec<u32> = range.filter(| x | x % 2 == 0).collect();
    println!("{:?}", vec);
}
