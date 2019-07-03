/*
* `SmallBox` can hold 5kgs
* `MediumBox` can hold 10kgs
* `LargeBox` can hold 20kgs
*/


struct Item {
    name: String,
    weight: f32
}

//get_weight`, `get_label`, `get_items_count` and `max_weight

struct SmallBox {
    label: String,
    items: Vec<Item>,
}

impl SmallBox {
    fn get_label(&self) -> &String {
        &self.label
    }
    
    fn get_weight(&self) -> f32 {
        let mut w: f32 = 0f32;
        for item in &self.items {
            w += item.weight;
        }
        w
    }
    
    fn get_items_count(&self) -> usize {
        self.items.len()
    }
    
    fn max_weight() -> f32 {
        5f32
    }
    
    fn remaining_weight(&self) -> f32 {
        SmallBox::max_weight() - self.get_weight()
    }
}

struct MediumBox {
    label: String,
    items: Vec<Item>,
}

impl MediumBox {
    fn get_label(&self) -> &String {
        &self.label
    }
    
    fn get_weight(&self) -> f32 {
        let mut w: f32 = 0f32;
        for item in &self.items {
            w += item.weight;
        }
        w
    }
    
    fn get_items_count(&self) -> usize {
        self.items.len()
    }
    
    fn max_weight() -> f32 {
        10f32
    }
    
    fn remaining_weight(&self) -> f32 {
        MediumBox::max_weight() - self.get_weight()
    }
}

struct LargeBox {
    label: String,
    items: Vec<Item>,
}

impl LargeBox {
    fn get_label(&self) -> &String {
        &self.label
    }
    
    fn get_weight(&self) -> f32 {
        let mut w: f32 = 0f32;
        for item in &self.items {
            w += item.weight;
        }
        w
    }
    
    fn get_items_count(&self) -> usize {
        self.items.len()
    }
    
    fn max_weight() -> f32 {
        20f32
    }
    
    fn remaining_weight(&self) -> f32 {
        LargeBox::max_weight() - self.get_weight()
    }
}

fn main() {

}
