use std::cell::RefCell;
use std::ptr;

pub struct Focus<A>(RefCell<FocusInner<A>>);

struct FocusInner<A> {
    focus: *const A,
    left: usize,
    right: usize,
}

impl<A> Focus<A> {
    pub fn new() -> Self {
        Focus(RefCell::new(FocusInner {
            focus: ptr::null(),
            left: 0,
            right: 0,
        }))
    }

    pub fn set(&self, focus: &A, left: usize, right: usize) {
        let mut this = self.0.borrow_mut();
        this.focus = focus;
        this.left = left;
        this.right = right;
    }

    pub fn clear(&self) {
        self.0.borrow_mut().focus = ptr::null();
    }

    pub fn get(&self, index: usize) -> Option<(usize, &A)> {
        let this = self.0.borrow_mut();
        if this.focus.is_null() || index < this.left || index >= this.right {
            return None;
        }
        Some((index - this.left, unsafe { &*this.focus }))
    }
}

pub struct FocusMut<A>(RefCell<FocusMutInner<A>>);

struct FocusMutInner<A> {
    focus: *mut A,
    left: usize,
    right: usize,
}

impl<A> FocusMut<A> {
    pub fn new() -> Self {
        FocusMut(RefCell::new(FocusMutInner {
            focus: ptr::null_mut(),
            left: 0,
            right: 0,
        }))
    }

    pub fn set(&self, focus: &mut A, left: usize, right: usize) {
        let mut this = self.0.borrow_mut();
        this.focus = focus;
        this.left = left;
        this.right = right;
    }

    pub fn clear(&self) {
        self.0.borrow_mut().focus = ptr::null_mut();
    }

    pub fn get(&self, index: usize) -> Option<(usize, &mut A)> {
        let mut this = self.0.borrow_mut();
        if this.focus.is_null() || index < this.left || index >= this.right {
            return None;
        }
        Some((index - this.left, unsafe { &mut *this.focus }))
    }
}

impl<A> Default for Focus<A> {
    fn default() -> Self {
        Self::new()
    }
}

impl<A> Default for FocusMut<A> {
    fn default() -> Self {
        Self::new()
    }
}
