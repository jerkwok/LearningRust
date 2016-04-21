//Concurrency Basics
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

//Arc Mutex example
fn arcmutex(){
	
	let data = Arc::new(Mutex::new(vec![0,1,2,3,4,5]));

	let mut results = vec![];
	for i in 0..6 {
		let mutex = data.clone();
		
		let handler = thread::spawn(move || {
			let mut data = mutex.lock().unwrap();

			data[i] = data[i] + 1;
			println!("{:?}",data[i]);

		});
		
		//sleep will allow threads to execute in order
		// thread::sleep(Duration::from_millis(50));

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
	
	println!("Arc Mutex");
	arcmutex();
	println!("Channel");
	channel();

}