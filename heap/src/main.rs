struct HeapNode<T: Copy, F>
where F: Fn(&T, &T) -> bool
{
    items: Vec<T>,
    size: usize,
    out_of_order: F,
}

#[allow(dead_code)]
impl<T: Copy, F> HeapNode<T, F>
where T: Copy, F: Fn(&T, &T) -> bool
{
    fn new(items: Vec<T>, out_of_order: F) -> Self {
        let size = items.len();
        HeapNode { items, size , out_of_order }
    }

    fn left_child_index(parent_index: usize) -> usize {
        parent_index * 2 + 1
    }

    fn right_child_index(parent_index: usize) -> usize {
        parent_index * 2 + 2
    }

    fn parent_index(child_index: usize) -> usize {
        (child_index - 1) / 2
    }

    fn has_left_child(&self, parent_index: usize) -> bool {
        Self::left_child_index(parent_index) < self.size
    }

    fn has_right_child(&self, parent_index: usize) -> bool {
        Self::right_child_index(parent_index) < self.size
    }

    fn has_parent(&self, index: usize) -> bool {
        // Self::parent_index(index) >= 0
        index != 0
    }

    fn left_child(&self, parent_index: usize) -> &T {
        &self.items[Self::left_child_index(parent_index)]
    }

    fn right_child(&self, parent_index: usize) -> &T {
        &self.items[Self::right_child_index(parent_index)]
    }

    fn parent(&self, index: usize) -> &T {
        &self.items[Self::parent_index(index)]
    }

    fn swap(&mut self, index_one: usize, index_two: usize) {
        let tmp = self.items[index_one];
        self.items[index_one] = self.items[index_two];
        self.items[index_two] = tmp;
    }

    fn peak(&self) -> &T {
        &self.items[0]
    }

    fn insert(&mut self, item: T) {
        self.items.push(item);
        self.size += 1;
        self.heapify_up();
    }

    fn poll(&mut self) -> Option<T> {
        if self.size == 0 {
            return None
        } else if self.size == 1 {
            self.size = 0;
            return self.items.pop()
        }
        let peak = *self.peak();
        match self.items.pop() {
            Some(item) => self.items[0] = item,
            _ => (),
        }
        self.size -= 1;
        self.heapify_down();
        Some(peak)
    }

    fn heapify_down(&mut self) {
        let mut cursor = 0;
        while self.has_left_child(cursor) {
            let mut smaller_child_index = Self::left_child_index(cursor);
            if self.has_right_child(cursor) && (self.out_of_order)(
                &self.left_child(cursor),
                &self.right_child(cursor)
            ) {
                smaller_child_index = Self::right_child_index(cursor);
            }
            if (self.out_of_order)(
                &self.items[cursor],
                &self.items[smaller_child_index]
            ) {
                self.swap(cursor, smaller_child_index);
                cursor = smaller_child_index;
            } else {
                break;
            }
        }
    }

    fn heapify_up(&mut self) {
        let mut cursor = self.size - 1;
        while self.has_parent(cursor) && (self.out_of_order)(
            &self.parent(cursor),
            &self.items[cursor]
        ) {
            let tmp = Self::parent_index(cursor);
            self.swap(cursor, tmp);
            cursor = tmp;
        }
    }
}

fn main() {
    println!("Hello, world!");
    let items = vec![5, 4, 2, 1];
    let mut heap = HeapNode::new(items, compare);
    heap.insert(8);
    println!("heap state {:?}", heap.items);
    heap.poll();
    println!("heap state {:?}", heap.items);
    heap.poll();
    println!("heap state {:?}", heap.items);
    heap.poll();
    println!("heap state {:?}", heap.items);
    heap.poll();
    println!("heap state {:?}", heap.items);
    heap.poll();
    println!("heap state {:?}", heap.items);
}

fn compare(parent: &i32, child: &i32) -> bool {
    parent < child
}

