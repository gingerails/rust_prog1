/********************
Name: Abi Kunkle
Date: 10/17/2022
Assignment: Program 1
Environment: IntelliJ IDEA 2022.2.1 on Windows 10
*********************/

use std::cmp::Ordering;
use std::collections::binary_heap::BinaryHeap;
use std::collections::vec_deque::VecDeque;
use std::io;
use rand::Rng;

/*
 main()
 Reads user input into variable by calling read_input(). Then, creates a queue and binary heap by calling build_processes.
 We then pull the processes from each collection to show they were stored in the correct order.
 */
fn main() {
    let input = read_input();
    let (process_queue, process_heap) = build_processes(input);
    pull_from_collection(process_queue, process_heap);
    println!("\n\nGoodbye.");
}

/*********************************
 Define the process struct and ord methods.
 *********************************/
#[derive(Copy, Clone,Eq, PartialEq)]
struct Process{
    id: i32,
    priority: i32,
    sleep_time: i32,
    description: &'static str
}
fn build_process(id: i32, priority:i32, sleep_time:i32, description: &'static str) -> Process{
    Process {
        id,
        priority,
        sleep_time,
        description
    }
}
impl PartialOrd for Process{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Process{
    fn cmp(&self, other: &Self) -> Ordering {
        (other.priority).cmp(&(self.priority))
    }
}
/*********************************
End Process struct definition
 *********************************/


/*
 read_input()
 */
fn read_input() -> String {
    println!("Enter the number of process nodes to generate:");
    let mut input = String::new();
    loop{
        io::stdin()
            .read_line(&mut input)
            .expect("Invalid input");
       if input.trim().parse::<i32>().is_ok(){
           break;
       }else{
           input.clear();
           println!("invalid input")
       }
   }
    return input;
}

/*
 takes in the valid user input and builds processes on loop
 */
fn build_processes(input:String) -> (VecDeque<Process>, BinaryHeap<Process>)  {
    println!("\nNow creating and adding {} process nodes to a Queue and to a binary minheap", input.trim());
    let input_int:i32 = input.trim().parse().unwrap();  // parsing input to an int so we can use it
    let process_description = "Process Node: ";
    let mut count = 1;//Start at 1.
    let mut process_queue: VecDeque<Process>=VecDeque::with_capacity(input_int as usize);
    let mut process_heap:BinaryHeap<Process> = BinaryHeap::new();

    // generates nodes via loop, calling the build_process function
    loop{
        let priority = rand_priority();
        let sleep_time = rand_sleep_time();
        let new_process = build_process(count, priority, sleep_time, process_description);
        let process_clone = new_process.clone();

        process_queue.push_back(new_process);
        process_heap.push(process_clone);
        if count==input_int {
            break;
        }
        count+=1;
    }
    println!("Verifying. The queue contains {} elements", input_int);
    println!("Verifying. The heap contains {} elements \n", input_int);

    return (process_queue, process_heap);
}

/*
 Drain minheap and queue. Calls print_process to print the current process in each collection on a loop.
 */
fn pull_from_collection(mut process_queue:VecDeque<Process>, mut process_heap:BinaryHeap<Process>){
    // pull from queue
    println!("Now, draining the Queue, one process at a time...");
    while !process_queue.is_empty(){
        let popped_process: Process = process_queue.pop_front().unwrap();
        print_process(&popped_process);
    }
    // pull from heap
    println!(" ");
    println!("Now, draining the MinHeap, one process at a time...");
    while !process_heap.is_empty(){
        let popped_process: Process = process_heap.pop().unwrap();
        print_process(&popped_process);
    }
}

/*
 generates and returns the priority value
 */
fn rand_priority() -> i32{
    // Generate random number in the range [0, 100]
    let priority: i32 =rand::thread_rng().gen_range(0..101);

    return priority;
}

/*
 generates and returns the sleep time in ms
 */
fn rand_sleep_time() -> i32{
    // Generate random number in the range [100, 2000]
    let sleep_time: i32 = rand::thread_rng().gen_range(100..2001);

    return sleep_time;
}

/*
 print the given process.
 */
fn print_process(process: &Process){
    println!("Pid: {:>5}, pri: {:>5}, sleep: {:>5}, desc: {} {:>1} ", process.id, process.priority, process.sleep_time, process.description, process.id);
}

