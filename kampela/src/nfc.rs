//! NFC packet collector and decoder

use nfca_parser::frame::Frame;

use kampela_system::{
    PERIPHERALS, BUF_THIRD, CH_TIM0,
};
use cortex_m::interrupt::free;
use crate::BUFFER_STATUS;

use kampela_system::devices::psram::{AddressPsram, ExternalPsram, PsramAccess};

use core::ops::DerefMut;

pub const FREQ: u16 = 22;
const NFC_MIN_VOLTAGE: i32 = 6000; //Affects initiation time, but lower values result in unreliable nfc reception

#[derive(Clone, Debug)]
pub enum BufferStatus {
    R0W1,
    R0Wh,
    R1W2,
    R1Wh,
    R2W0,
    R2Wh,
    RhW0,
    RhW1,
    RhW2,
}

#[derive(Debug)]
pub enum BufRegion {
    Reg0,
    Reg1,
    Reg2,
}


#[derive(Debug)]
pub enum BufferError {
    UnexpectedIfDone7,
    UnexpectedReadDone,
}

impl BufferStatus {
    pub fn new() -> Self {
        Self::RhW0
    }
    pub fn pass_if_done7(&mut self) -> Result<(), BufferError> {
        let new_self = match self {
            Self::R0W1 => Self::R0Wh,
            Self::R0Wh => return Err(BufferError::UnexpectedIfDone7),
            Self::R1W2 => Self::R1Wh,
            Self::R1Wh => return Err(BufferError::UnexpectedIfDone7),
            Self::R2W0 => Self::R2Wh,
            Self::R2Wh => return Err(BufferError::UnexpectedIfDone7),
            Self::RhW0 => Self::R0W1,
            Self::RhW1 => Self::R1W2,
            Self::RhW2 => Self::R2W0,
        };
        *self = new_self;
        Ok(())
    }
    pub fn pass_read_done(&mut self) -> Result<(), BufferError> {
        let new_self = match self {
            Self::R0W1 => Self::RhW1,
            Self::R0Wh => Self::R1W2,
            Self::R1W2 => Self::RhW2,
            Self::R1Wh => Self::R2W0,
            Self::R2W0 => Self::RhW0,
            Self::R2Wh => Self::R0W1,
            Self::RhW0 => return Err(BufferError::UnexpectedReadDone),
            Self::RhW1 => return Err(BufferError::UnexpectedReadDone),
            Self::RhW2 => return Err(BufferError::UnexpectedReadDone),
        };
        *self = new_self;
        Ok(())
    }
    pub fn read_from(&self) -> Option<BufRegion> {
        match self {
            Self::R0W1 => Some(BufRegion::Reg0),
            Self::R0Wh => Some(BufRegion::Reg0),
            Self::R1W2 => Some(BufRegion::Reg1),
            Self::R1Wh => Some(BufRegion::Reg1),
            Self::R2W0 => Some(BufRegion::Reg2),
            Self::R2Wh => Some(BufRegion::Reg2),
            Self::RhW0 => None,
            Self::RhW1 => None,
            Self::RhW2 => None,
        }
    }
    pub fn is_write_halted(&self) -> bool {
        match self {
            Self::R0W1 => false,
            Self::R0Wh => true,
            Self::R1W2 => false,
            Self::R1Wh => true,
            Self::R2W0 => false,
            Self::R2Wh => true,
            Self::RhW0 => false,
            Self::RhW1 => false,
            Self::RhW2 => false,
        }
    }
}

pub fn turn_nfc_collector_correctly(_: &mut NfcCollector, nfc_buffer: &[u16; 3*BUF_THIRD]) {
    let mut read_from = None;
    free(|cs| {
        let buffer_status = BUFFER_STATUS.borrow(cs).borrow();
        read_from = buffer_status.read_from();
    });
    let decoder_input = match read_from {
        Some(BufRegion::Reg0) => &nfc_buffer[..BUF_THIRD],
        Some(BufRegion::Reg1) => &nfc_buffer[BUF_THIRD..2*BUF_THIRD],
        Some(BufRegion::Reg2) => &nfc_buffer[2*BUF_THIRD..],
        None => return,
    };
    let frames = Frame::process_buffer_miller_skip_tails::<_, FREQ>(decoder_input, |frame| frame_selected(&frame));

    for frame in frames.into_iter() {
        if let Frame::Standard(_) = frame {
        }
        else {unreachable!()}
    }

    free(|cs| {
        let mut buffer_status = BUFFER_STATUS.borrow(cs).borrow_mut();
        let was_write_halted = buffer_status.is_write_halted();
        buffer_status.pass_read_done().expect("to do");
        if was_write_halted & ! buffer_status.is_write_halted() {
            if let Some(ref mut peripherals) = PERIPHERALS.borrow(cs).borrow_mut().deref_mut() {
                peripherals.ldma_s.linkload().write(|w_reg| unsafe { w_reg.linkload().bits(1 << CH_TIM0) });
            }
            else {panic!("can not borrow peripherals, buffer_status: {:?}, got some new frames", buffer_status)}
        }
    });
}

fn frame_selected(frame: &Frame) -> bool {
    if let Frame::Standard(_) = frame {
        true
    }
    else {false}
}

pub enum NfcCollector {
    Empty,
    InProgress,
    Done,
}

impl NfcCollector {
    pub fn new() -> Self {
        Self::Empty
    }
    pub fn _add_packet(&mut self, _: &mut ExternalPsram) {
        match self {
            NfcCollector::Empty => {
            },
            NfcCollector::InProgress => {

            },
            NfcCollector::Done => {
            }
        }
    }
}

#[derive(Debug)]
pub enum NfcPayloadError {
    AccessOnPayload,
//    AccessOnPublicKey,
//    AccessOnSignature,
//    ExcessData,
//    NoCompactPayload,
//    NoCompactPublicKey,
//    NoCompactSignature,
}

#[derive(Debug)]
pub struct TransferDataReceived {
    data: (),
}

pub fn process_nfc_payload() -> Result<TransferDataReceived, NfcPayloadError> {
    let psram_data = PsramAccess {
        start_address: AddressPsram::zero(),
        total_len: 0,
    };
    Ok(TransferDataReceived{
        data: ()
    })
}

//TODO: implement more error cases, i.e. old specs
pub enum NfcError {
    InvalidAddress,
}

pub enum NfcResult {
    Empty,
}

enum NfcState {
    Operational(usize),
    Done,
}
pub enum NfcStateOutput {
    Operational(usize),
    Done(NfcResult),
}


pub struct NfcReceiver <'a> {
    buffer: &'a [u16; 3*BUF_THIRD],
    collector: NfcCollector,
    state: NfcState,
}

impl <'a> NfcReceiver<'a> {
    pub fn new(nfc_buffer: &'a [u16; 3*BUF_THIRD]) -> Self {
        Self {
            buffer: nfc_buffer,
            collector: NfcCollector::new(),
            state: NfcState::Done,
        }
    }

    fn process(&mut self) -> Option<Result<NfcResult, NfcError>> {
        turn_nfc_collector_correctly(&mut self.collector, self.buffer);

        match self.collector {
            NfcCollector::Empty => Some(Ok(NfcResult::Empty)),
            NfcCollector::Done => Some(Ok(NfcResult::Empty)),
            NfcCollector::InProgress => None,
        }
    }

    pub fn advance(&mut self, voltage: i32) -> Option<Result<NfcStateOutput, NfcError>> {
        if voltage < NFC_MIN_VOLTAGE { return None }
        match self.state {
            NfcState::Operational(i) => {
                let res = self.process();
                match res {
                    Some(r) => {
                        match r {
                            Err(e) => { Some(Err(e)) },
                            Ok(r) => {
                                self.state = NfcState::Done;
                                Some(Ok(NfcStateOutput::Done(r)))
                            },
                        }
                    },
                    None => {
                        self.state = NfcState::Operational(i + 1);
                        Some(Ok(NfcStateOutput::Operational(i + 1)))
                    },
                }
            },
            NfcState::Done => { Some(Ok(NfcStateOutput::Done(NfcResult::Empty))) }
        }
    }
}



// if got_transaction.is_some() {

//     let transaction = got_transaction.unwrap();
//     let context = signing_context(SIGNING_CTX);
//     let signature = pair_derived.sign(attach_rng(context.bytes(&transaction.2), &mut SeRng{}));
//     let mut signature_with_id: [u8; 65] = [1; 65];
//     signature_with_id[1..].copy_from_slice(&signature.to_bytes());
//     let signature_into_qr: [u8; 130] = hex::encode(signature_with_id).into_bytes().try_into().expect("static known length");

//     ui.handle_rx(transaction.0, transaction.1, signature_into_qr);

//     break
// }
// None
