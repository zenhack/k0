
use super::portio;

// Polling serial driver

pub struct SerialPort(u16);

pub const COM1 : SerialPort = SerialPort(0x3f8);

pub fn init(serial_port : SerialPort) {
  let SerialPort(port) = serial_port;
  unsafe {
    portio::outb(port + INT_ENABLE, 0); // disable interrupts on the device
    portio::outb(port + LINE_CNTL_REG, DLAB_SET);

    // set the baud rate divisor:
    portio::outb(port + BAUD_DIV_LSB, 3);
    portio::outb(port + BAUD_DIV_MSB, 0);

    // clear DLAB, ask for 8 bit chars with no parity:
    portio::outb(port + LINE_CNTL_REG, DLAB_CLR | DATABITS_8 | NO_PAIRITY);
  }
}

pub fn putc(serial_port : SerialPort, byte : u8) {
  let SerialPort(port) = serial_port;
  unsafe {
    while (portio::inb(port + LINE_STATUS_REG) & LINE_READY_STATUS) == 0 {
    }
    portio::outb(port + DATA_REG, byte)
  }
}

// Bits in LINE_CNTL_REG
const DLAB_SET          : u8 = 1 << 7;
const DLAB_CLR          : u8 = 0;
const DATABITS_8        : u8 = 3;
const NO_PAIRITY        : u8 = 0;
const LINE_READY_STATUS : u8 = 1 << 5;

// Meaningful only when DLAB is clear:
const DATA_REG   : u16 = 0;
const INT_ENABLE : u16 = 1;

// Meaningful only when DLAB is set:
const BAUD_DIV_LSB : u16 = 0;
const BAUD_DIV_MSB : u16 = 1;

// Always meaningful:
const LINE_CNTL_REG   : u16 = 3;
const LINE_STATUS_REG : u16 = 5;
