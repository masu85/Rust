use crate::shared::Money;

#[derive(Clone)]
pub struct Order {
    pub id: String,
    pub status: Status,
    pub payment_state: PaymentStateType,
    pub open_to_capture_amount: Money

}

pub enum Status {
    CREATED, AUTHORIZED,
    INVALIDATED, UNCONFIRMED,
    APPROVED, DECLINED, FAILED
}

pub enum PaymentStateType {
    AUTH_APPROVED, AUTH_DECLINED, PARTIALLY_CAPTURED,
    CAPTURED, CAPTURE_DECLINED, VOIDED, EXPIRED
}


/*

private String orderId;
    private Status status = Status.CREATED;
    private LocalDateTime created;
    private PaymentStateType payment_state;
    private Money openToCaptureAmount;
    private List<Refund> refunds = new ArrayList<>();
    private List<PaymentEvent> events = new ArrayList<>();
 */