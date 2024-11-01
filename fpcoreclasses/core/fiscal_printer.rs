// core/fiscal_printer.rs

use crate::credentials::Credentials;
use crate::device_info::DeviceInfo;
use crate::device_status::{
    DeviceStatus, DeviceStatusWithCashAmount, DeviceStatusWithDateTime, DeviceStatusWithRawResponse,
};
use crate::receipt::{Receipt, ReceiptInfo, ReversalReceipt};
use crate::request_frame::RequestFrame;
use crate::transfer_amount::TransferAmount;

/// Represents the capabilities of a connected fiscal printer.
pub trait FiscalPrinter {
    /// Gets information about the connected device.
    fn device_info(&self) -> DeviceInfo;

    /// Checks whether the device is currently ready to accept commands.
    fn check_status(&self) -> DeviceStatusWithDateTime;

    /// Gets the amount of cash available.
    fn cash(&self, credentials: &Credentials) -> DeviceStatusWithCashAmount;

    /// Sets the device date and time.
    fn set_date_time(&self, current_date_time: CurrentDateTime) -> DeviceStatus;

    /// Prints the specified receipt.
    fn print_receipt(&self, receipt: Receipt) -> (ReceiptInfo, DeviceStatus);

    /// Validates the receipt object.
    fn validate_receipt(&self, receipt: &Receipt) -> DeviceStatus;

    /// Prints the specified reversal receipt.
    fn print_reversal_receipt(
        &self,
        reversal_receipt: ReversalReceipt,
    ) -> (ReceiptInfo, DeviceStatus);

    /// Validates the reversal receipt object.
    fn validate_reversal_receipt(&self, reversal_receipt: &ReversalReceipt) -> DeviceStatus;

    /// Prints a deposit money note.
    fn print_money_deposit(&self, transfer_amount: TransferAmount) -> DeviceStatus;

    /// Prints a withdraw money note.
    fn print_money_withdraw(&self, transfer_amount: TransferAmount) -> DeviceStatus;

    /// Validates transfer amount object.
    fn validate_transfer_amount(&self, transfer_amount: &TransferAmount) -> DeviceStatus;

    /// Prints a Z-report.
    fn print_z_report(&self, credentials: &Credentials) -> DeviceStatus;

    /// Prints an X-report.
    fn print_x_report(&self, credentials: &Credentials) -> DeviceStatus;

    /// Prints a duplicate of the last fiscal receipt.
    fn print_duplicate(&self, credentials: &Credentials) -> DeviceStatus;

    /// Raw request.
    fn raw_request(&self, request_frame: RequestFrame) -> DeviceStatusWithRawResponse;

    /// Tries to fix the erroneous state of the device to the normal - ready for printing state.
    fn reset(&self, credentials: &Credentials) -> DeviceStatusWithDateTime;

    /// Sets a deadline for some operation.
    fn set_deadline(&self, dead_line: std::time::SystemTime);
}
