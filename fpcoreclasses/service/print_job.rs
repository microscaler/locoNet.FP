use std::option::Option;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub enum PrintJobAction {
    None,
    Cash,
    RawRequest,
    Receipt,
    ReversalReceipt,
    Withdraw,
    Deposit,
    XReport,
    ZReport,
    SetDateTime,
    Duplicate,
    Reset,
}

pub type Run = fn(&dyn std::any::Any) -> Option<Box<dyn std::any::Any>>;

pub trait IFiscalPrinter {
    fn set_deadline(&mut self, deadline: SystemTime);
    fn cash(&self, credentials: &Credentials) -> Option<Box<dyn std::any::Any>>;
    fn raw_request(&self, request: &RequestFrame) -> Option<Box<dyn std::any::Any>>;
    fn validate_receipt(&self, receipt: &Receipt) -> DeviceStatus;
    fn print_receipt(&self, receipt: &Receipt) -> (Box<dyn std::any::Any>, DeviceStatus);
    fn validate_reversal_receipt(&self, reversal_receipt: &ReversalReceipt) -> DeviceStatus;
    fn print_reversal_receipt(
        &self,
        reversal_receipt: &ReversalReceipt,
    ) -> (Box<dyn std::any::Any>, DeviceStatus);
    fn validate_transfer_amount(&self, transfer_amount: &TransferAmount) -> DeviceStatus;
    fn print_money_withdraw(
        &self,
        transfer_amount: &TransferAmount,
    ) -> Option<Box<dyn std::any::Any>>;
    fn print_money_deposit(
        &self,
        transfer_amount: &TransferAmount,
    ) -> Option<Box<dyn std::any::Any>>;
    fn print_x_report(&self, credentials: &Credentials) -> Option<Box<dyn std::any::Any>>;
    fn print_z_report(&self, credentials: &Credentials) -> Option<Box<dyn std::any::Any>>;
    fn set_date_time(&self, date_time: &CurrentDateTime) -> Option<Box<dyn std::any::Any>>;
    fn print_duplicate(&self, credentials: &Credentials) -> Option<Box<dyn std::any::Any>>;
    fn reset(&self, credentials: &Credentials) -> Option<Box<dyn std::any::Any>>;
}

pub struct PrintJob {
    pub enqueued: SystemTime,
    pub started: Option<SystemTime>,
    pub finished: Option<SystemTime>,
    pub deadline: SystemTime,
    pub action: PrintJobAction,
    pub printer: Option<Box<dyn IFiscalPrinter>>,
    pub task_status: TaskStatus,
    pub document: Option<Box<dyn std::any::Any>>,
    pub result: Option<Box<dyn std::any::Any>>,
    pub task_id: Option<String>,
    pub async_timeout: u64,
    timeout: u64,
}

#[derive(Debug)]
pub enum TaskStatus {
    Enqueued,
    Running,
    Finished,
    Unknown,
}

#[derive(Debug)]
pub struct DeviceStatus {
    errors: Vec<String>,
}

impl DeviceStatus {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn add_error(&mut self, code: &str, message: &str) {
        self.errors.push(format!("{}: {}", code, message));
    }
}

impl PrintJob {
    pub const DEFAULT_TIMEOUT: u64 = 29_000; // 29 seconds

    pub fn new() -> Self {
        PrintJob {
            enqueued: SystemTime::now(),
            started: None,
            finished: None,
            deadline: UNIX_EPOCH + Duration::from_secs(u64::MAX),
            action: PrintJobAction::None,
            printer: None,
            task_status: TaskStatus::Enqueued,
            document: None,
            result: None,
            task_id: None,
            async_timeout: Self::DEFAULT_TIMEOUT,
            timeout: 0,
        }
    }

    pub fn set_timeout(&mut self, value: u64) {
        self.timeout = value;
        if value <= 0 {
            self.deadline = UNIX_EPOCH + Duration::from_secs(u64::MAX);
        } else {
            self.deadline = self.enqueued + Duration::from_millis(value);
        }
    }

    pub fn run(&mut self) {
        if self.printer.is_none() {
            return;
        }

        self.started = Some(SystemTime::now());

        if self.deadline <= self.started.unwrap() {
            self.finished = Some(SystemTime::now());
            self.task_status = TaskStatus::Finished;
            let mut device_status = DeviceStatus::new();
            device_status.add_error("E999", "User timeout occurred while sending the request");
            self.result = Some(Box::new(device_status));
            return;
        }

        self.task_status = TaskStatus::Running;

        if let Some(printer) = self.printer.as_mut() {
            printer.set_deadline(self.deadline);
            let doc = self.document.as_ref().map(|d| d.as_ref());
            match self.action {
                PrintJobAction::Cash => {
                    let credentials = doc.unwrap_or_else(|| Box::new(Credentials::default()));
                    self.result = printer.cash(credentials);
                }
                PrintJobAction::RawRequest => {
                    if let Some(document) = doc {
                        self.result =
                            printer.raw_request(document.downcast_ref::<RequestFrame>().unwrap());
                    }
                }
                PrintJobAction::Receipt => {
                    if let Some(document) = doc {
                        let receipt = document.downcast_ref::<Receipt>().unwrap();
                        let validate_status = printer.validate_receipt(receipt);
                        if validate_status.errors.is_empty() {
                            let (info, status) = printer.print_receipt(receipt);
                            self.result = Some(info);
                        } else {
                            self.result = Some(Box::new(validate_status));
                        }
                    }
                }
                PrintJobAction::ReversalReceipt => {
                    if let Some(document) = doc {
                        let reversal_receipt = document.downcast_ref::<ReversalReceipt>().unwrap();
                        let validate_status = printer.validate_reversal_receipt(reversal_receipt);
                        if validate_status.errors.is_empty() {
                            let (info, status) = printer.print_reversal_receipt(reversal_receipt);
                            self.result = Some(info);
                        } else {
                            self.result = Some(Box::new(validate_status));
                        }
                    }
                }
                PrintJobAction::Withdraw => {
                    if let Some(document) = doc {
                        let transfer_amount = document.downcast_ref::<TransferAmount>().unwrap();
                        let validate_status = printer.validate_transfer_amount(transfer_amount);
                        if validate_status.errors.is_empty() {
                            self.result = printer.print_money_withdraw(transfer_amount);
                        } else {
                            self.result = Some(Box::new(validate_status));
                        }
                    }
                }
                PrintJobAction::Deposit => {
                    if let Some(document) = doc {
                        let transfer_amount = document.downcast_ref::<TransferAmount>().unwrap();
                        let validate_status = printer.validate_transfer_amount(transfer_amount);
                        if validate_status.errors.is_empty() {
                            self.result = printer.print_money_deposit(transfer_amount);
                        } else {
                            self.result = Some(Box::new(validate_status));
                        }
                    }
                }
                PrintJobAction::XReport => {
                    let credentials = doc.unwrap_or_else(|| Box::new(Credentials::default()));
                    self.result = printer.print_x_report(credentials);
                }
                PrintJobAction::ZReport => {
                    let credentials = doc.unwrap_or_else(|| Box::new(Credentials::default()));
                    self.result = printer.print_z_report(credentials);
                }
                PrintJobAction::SetDateTime => {
                    if let Some(document) = doc {
                        let date_time_document =
                            document.downcast_ref::<CurrentDateTime>().unwrap();
                        if date_time_document.device_date_time == UNIX_EPOCH {
                            date_time_document.device_date_time = SystemTime::now();
                        }
                        self.result = printer.set_date_time(date_time_document);
                    }
                }
                PrintJobAction::Duplicate => {
                    let credentials = doc.unwrap_or_else(|| Box::new(Credentials::default()));
                    self.result = printer.print_duplicate(credentials);
                }
                PrintJobAction::Reset => {
                    let credentials = doc.unwrap_or_else(|| Box::new(Credentials::default()));
                    self.result = printer.reset(credentials);
                }
                _ => {}
            }
        }

        self.finished = Some(SystemTime::now());
        self.task_status = TaskStatus::Finished;
        if let Some(printer) = self.printer.as_mut() {
            printer.set_deadline(UNIX_EPOCH + Duration::from_secs(u64::MAX));
        }
    }
}

// Placeholder structs
#[derive(Debug)]
pub struct Credentials {
    // Your fields here
}

impl Default for Credentials {
    fn default() -> Self {
        Credentials {
            // Initialize default values
        }
    }
}

#[derive(Debug)]
pub struct RequestFrame {
    // Your fields here
}

#[derive(Debug)]
pub struct Receipt {
    // Your fields here
}

#[derive(Debug)]
pub struct ReversalReceipt {
    // Your fields here
}

#[derive(Debug)]
pub struct TransferAmount {
    // Your fields here
}

#[derive(Debug)]
pub struct CurrentDateTime {
    pub device_date_time: SystemTime,
}
