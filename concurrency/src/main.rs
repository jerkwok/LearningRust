//Concurrency Basics
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

//Arc Mutex example
fn arcmutex(){
	
	let data = Arc::new(Mutex::new(vec![0,1,2,3,4,5]));

	let mut results = vec![];
	for i in 0..7 {
			let mutex = data.clone();
		
		let handler = thread::spawn(move || {
			let mut data = mutex.lock().unwrap();

			//Mutate this data!
			data[i] = data[i] + 1;
			println!("{:?}",data[i]);

			//Compiler is SMART; no data races
			data[0] = data[0] + 1;
			println!("{:?}",data[0]);
		});
		
		results.push(handler);
	}
}

fn channel(){
	//Channel example
    let (transmitter, reciever) = mpsc::channel();

    for i in 0..10 {
        let transmitter = transmitter.clone();

        thread::spawn(move || {
            println!("Hello World from Thread {}", i);
            transmitter.send(i).unwrap();
        });
    }

    for _ in 0..10 {
        println!("Recieved from Thread {}", reciever.recv().unwrap());
    }
}

fn main() {
	
	arcmutex();
	// channel();

}