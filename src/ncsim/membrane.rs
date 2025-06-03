// NCSim Membrane
#![allow(dead_code)]

pub mod membrane {

	#[derive(Debug, Clone, Copy)]
	struct Membrane(u8);

	impl Membrane {
		// Constants for bit masks
		// 0b7654_3210:
		// 
		const INOUT_MASK: u8 = 0b0000_0001; // In or out: Bits 0


		//const STATUS_MASK: u8 = 0b0001_1000; // Bits 3-4
		//const FLAG_MASK: u8 = 0b1110_0000; // Bits 5-7

		fn inout_mask(&self) -> u8 {
			self.0 & Self::INOUT_MASK
		}
	}

}
