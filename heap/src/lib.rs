/// Heap Data Structure
///
/// # Examples
///
/// ```
/// fn compare(parent: &i32, child: &i32) -> bool {
///     parent < child
/// }
///
/// let mut heap = datastructures::HeapNode::new(compare);
/// heap.insert(3);
/// heap.insert(5);
/// heap.insert(1);
/// let poll = heap.poll();
/// assert_eq!(5, poll.unwrap());
/// ```
pub struct HeapNode<T: Copy, F>
where F: Fn(&T, &T) -> bool
{
    items: Vec<T>,
    size: usize,
    out_of_order: F,
}

impl<T: Copy, F> HeapNode<T, F>
where T: Copy, F: Fn(&T, &T) -> bool
{
    pub fn new(out_of_order: F) -> Self {
        HeapNode { items: vec![], size: 0 , out_of_order }
    }

    pub fn left_child_index(parent_index: usize) -> usize {
        parent_index * 2 + 1
    }

    pub fn right_child_index(parent_index: usize) -> usize {
        parent_index * 2 + 2
    }

    pub fn parent_index(child_index: usize) -> usize {
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

    pub fn peak(&self) -> &T {
        &self.items[0]
    }

    pub fn insert(&mut self, item: T) {
        self.items.push(item);
        self.size += 1;
        self.heapify_up();
    }

    pub fn poll(&mut self) -> Option<T> {
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
