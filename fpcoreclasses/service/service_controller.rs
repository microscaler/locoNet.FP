use std::collections::{HashMap, VecDeque};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct DeviceInfo {
    pub uri: String,
    pub serial_number: String,
}

pub trait IFiscalPrinter {
    fn device_info(&self) -> &DeviceInfo;
}

#[derive(Debug)]
pub struct PrinterConfig {
    pub uri: String,
}

#[derive(Debug)]
pub struct PrinterProperties;

#[derive(Debug)]
pub struct PrinterConfigWithId {
    pub id: String,
    pub uri: String,
}

#[derive(Debug)]
pub struct TaskInfoResult {
    pub task_status: String,
    pub result: Option<Box<dyn std::any::Any>>,
}

#[derive(Debug)]
pub struct TaskIdResult {
    pub task_id: String,
}

#[derive(Debug)]
pub struct DeviceStatus;

#[derive(Debug)]
pub struct PrintJob {
    pub finished: Option<bool>,
    pub result: Option<Box<dyn std::any::Any>>,
    pub task_status: String,
    pub task_id: String,
    pub async_timeout: i32,
}

impl PrintJob {
    pub const DEFAULT_TIMEOUT: i32 = 29000; // 29 seconds
}

#[derive(Debug)]
pub struct ServiceOptions {
    pub printers: HashMap<String, PrinterConfig>,
    pub printers_properties: HashMap<String, PrinterProperties>,
    pub auto_detect: bool,
    pub udp_beacon_port: u16,
    pub server_id: String,
}

pub struct ServiceControllerContext {
    consumer: Option<JoinHandle<()>>,
    task_queue: Arc<Mutex<VecDeque<String>>>,
    tasks: Arc<Mutex<HashMap<String, PrintJob>>>,
    config_options: ServiceOptions,
    is_ready: bool,
    server_id: String,
    udp_beacon_port: u16,
    auto_detect: bool,
}

impl ServiceControllerContext {
    pub fn new() -> Self {
        let config_options = ServiceOptions {
            printers: HashMap::new(),
            printers_properties: HashMap::new(),
            auto_detect: true,
            udp_beacon_port: 8001,
            server_id: Self::generate_server_id(),
        };

        let task_queue = Arc::new(Mutex::new(VecDeque::new()));
        let tasks = Arc::new(Mutex::new(HashMap::new()));

        let mut controller = ServiceControllerContext {
            consumer: None,
            task_queue,
            tasks,
            config_options,
            is_ready: false,
            server_id: config_options.server_id.clone(),
            udp_beacon_port: config_options.udp_beacon_port,
            auto_detect: config_options.auto_detect,
        };

        controller.setup();

        controller
    }

    fn generate_server_id() -> String {
        let uuid = Uuid::new_v4();
        base64::encode_config(uuid.as_bytes(), base64::URL_SAFE)
            .chars()
            .take(22)
            .collect()
    }

    fn setup(&mut self) {
        self.read_options();
        self.is_ready = true;

        let task_queue = Arc::clone(&self.task_queue);
        let tasks = Arc::clone(&self.tasks);
        self.consumer = Some(thread::spawn(move || {
            while let Ok(task_id) = task_queue.lock().unwrap().pop_front() {
                let mut tasks = tasks.lock().unwrap();
                if let Some(print_job) = tasks.get_mut(&task_id) {
                    // Here you would run the print job
                    print_job.finished = Some(true); // Mark as finished for simplicity
                }
            }
        }));
    }

    fn read_options(&mut self) {
        if self.config_options.server_id.is_empty() {
            self.config_options.server_id = Self::generate_server_id();
        }
        self.server_id = self.config_options.server_id.clone();
    }

    pub fn detect(&mut self, force_auto_detect: bool) -> bool {
        if self.is_ready {
            self.is_ready = false;

            self.config_options.printers.clear();

            // Autodetecting logic here
            if force_auto_detect || self.config_options.auto_detect {
                // Simulated detection logic
                // You would call your printer detection logic here
                println!("Autodetecting printers...");
            }

            // Logic for detecting configured printers
            if !self.config_options.printers.is_empty() {
                for (key, config) in &self.config_options.printers {
                    // Simulated connection
                    println!("Trying to connect to {}: {}", key, config.uri);
                }
            }

            println!(
                "Detection complete. Found {} printers.",
                self.config_options.printers.len()
            );
            self.is_ready = true;
            true
        } else {
            false
        }
    }

    pub fn get_task_info(&self, task_id: &str) -> TaskInfoResult {
        let tasks = self.tasks.lock().unwrap();
        if let Some(print_job) = tasks.get(task_id) {
            TaskInfoResult {
                task_status: print_job.task_status.clone(),
                result: print_job.result.clone(),
            }
        } else {
            TaskInfoResult {
                task_status: String::from("Unknown"),
                result: None,
            }
        }
    }

    pub fn run_async(&mut self, print_job: PrintJob) -> Result<Option<TaskIdResult>, String> {
        let task_id = self.enqueue(print_job);
        Ok(Some(TaskIdResult { task_id }))
    }

    fn enqueue(&mut self, print_job: PrintJob) -> String {
        let task_id = base64::encode_config(Uuid::new_v4().as_bytes(), base64::URL_SAFE)
            .chars()
            .take(22)
            .collect::<String>();

        self.tasks
            .lock()
            .unwrap()
            .insert(task_id.clone(), print_job);
        self.task_queue.lock().unwrap().push_back(task_id.clone());

        // Ensure the consumer is running
        self.ensure_consumer();

        task_id
    }

    fn ensure_consumer(&mut self) {
        // This can be used to ensure the consumer is running
        if self.consumer.is_none() || self.consumer.as_ref().unwrap().is_finished() {
            let task_queue = Arc::clone(&self.task_queue);
            let tasks = Arc::clone(&self.tasks);
            self.consumer = Some(thread::spawn(move || {
                while let Ok(task_id) = task_queue.lock().unwrap().pop_front() {
                    let mut tasks = tasks.lock().unwrap();
                    if let Some(print_job) = tasks.get_mut(&task_id) {
                        // Here you would run the print job
                        print_job.finished = Some(true); // Mark as finished for simplicity
                    }
                }
            }));
        }
    }
}

fn main() {
    let mut service_controller = ServiceControllerContext::new();
    service_controller.detect(true);

    let print_job = PrintJob {
        finished: None,
        result: None,
        task_status: String::from("Enqueued"),
        task_id: String::new(),
        async_timeout: 0,
    };

    match service_controller.run_async(print_job) {
        Ok(Some(task_id_result)) => println!("Task ID: {:?}", task_id_result.task_id),
        Ok(None) => println!("No Task ID generated"),
        Err(e) => println!("Error: {}", e),
    }
}
