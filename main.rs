#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]
mod cqbase;
//use std::fmt::Display;
use std::default::Default;
use cqbase::CircQ;
use cqbase::Queue;

/////////////
impl<T:Default+std::fmt::Debug> Queue<T> for CircQ<T>    // no conditions on T allowed!
{
    ///// "public" functions

    fn size(&self) -> usize{
        self.size 
    }

    ///// implement the other trait Queue functions...
    fn current_capacity(&self) -> usize{
        self.capacity
    }

    fn push(&mut self, x:T) { 
        if (self.size >= self.capacity) { self.double_capacity(); }
        if (self.size > 0) { self.front = self.left(self.front); }
        //println!("{} gonna put val {}", self.qv[self.front], x);
        self.qv[self.front] = x;
        self.size+=1;
    }

    fn pop(&mut self) -> Option<T>{
        if (self.size <= 0) { return None; }
        let val = std::mem::replace(&mut self.qv[self.front], T::default());
        self.front = self.right(self.front);
        self.size -= 1; 
        Some(val)
    }
    
    fn peek(&self) -> Option<&T>{
        if (self.size <= 0) { return None; }
        Some(&self.qv[self.front])
    }

    fn enqueue(&mut self, x:T){
       if(self.size >= self.capacity) { self.double_capacity(); }
       let index = self.end();
       self.qv[index] = x;
       self.size+=1; 
    }

    fn dequeue(&mut self) -> Option<T>{
        if(self.size <= 0) {return None;}
        let index:usize = self.end() - 1;
        let val = std::mem::replace(&mut self.qv[index], T::default());
        self.size-=1;
        Some(val)
    }

    fn get(&self, i:usize) -> Option<&T>{
        if (self.size <= i) {return None;}
        let index:usize = self.ith(i);
        let val = &self.qv[index];
        Some(val)
    }

    fn set(&mut self, i:usize, x:T) -> Result<(), ()>{
        if(self.size <= i) {return Err(());} 
        let index:usize = self.ith(i);
        self.qv[index] = x;
        Ok(()) 
    }
    fn double_capacity(&mut self){
        let new_capacity = self.capacity * 2; 
        let mut new_v = Vec::new();
        new_v.resize_with(new_capacity, ||{T::default()});
        let mut i = self.size;
        while i > 0 {
            let index:usize = self.ith(i-1);
            new_v[i-1] = std::mem::take(&mut self.qv[index]); 
            i-=1;
        }
        self.front = 0;
        self.qv = new_v;
        self.capacity = new_capacity;
    }

    fn shrink(&mut self) -> usize{
        let new_cap = ((self.size*10)/9);//self.size is 90% of new_cap
        if (new_cap >= self.capacity) {return 0;}
        //self.qv.shrink_to(new_cap); //doesn't change qv.len()...
        let mut new_v = Vec::new();
        new_v.resize_with(new_cap, ||{T::default()});
        let mut i = self.size;
        while i > 0 {
            let index:usize = self.ith(i-1);
            new_v[i-1] = std::mem::take(&mut self.qv[index]); 
            i-=1;
        }
        self.front = 0;
        self.qv = new_v;
        let removed = self.capacity - new_cap;
        self.capacity = new_cap;
        removed
    }

/*testing 2nd method - does not work since it can create incontiguous
memory blocks
    fn double_capacity(&mut self){
        let new_capacity = self.capacity*2; 
        println!("len: {:?}",self.qv.capacity());
        self.qv.reserve_exact(self.capacity);
        self.capacity = new_capacity;
        println!("len: {:?}",self.qv.capacity());
    }
*/
    // ...
}//impl Queue for CircQ

/////////////////////// main  -  don't touch me
fn main() {
  let mut iq = CircQ::new();  // mut needed here, make the whole thing a mut
  iq.info();
  iq.push(1);  iq.push(3); iq.enqueue(2); iq.enqueue(4); iq.enqueue(6);
  iq.printq();
  iq.info();

  let mut q = CircQ::new();
  for i in 1..10 { q.push(i); } 
  for i in 10..15 { q.enqueue(i); }
  for i in 1..12 { print!("{} ",q.pop().unwrap()); }
  println!("dequeued: {}",q.dequeue().unwrap());
  let q2 = &mut q;
  q.info();
}//main

